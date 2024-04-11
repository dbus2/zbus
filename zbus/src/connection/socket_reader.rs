use std::{collections::HashMap, sync::Arc};

use crate::abstractions::logging::{debug, trace};
use event_listener::Event;
use zvariant::{
    serialized::{self, Context},
    Endian,
};

use crate::{
    async_lock::Mutex,
    connection::MsgBroadcaster,
    message::header::{PrimaryHeader, MAX_MESSAGE_SIZE, MIN_MESSAGE_SIZE},
    padding_for_8_bytes, Executor, Message, OwnedMatchRule, Task,
};

use super::socket::ReadHalf;

#[derive(Debug)]
pub(crate) struct SocketReader {
    socket: Box<dyn ReadHalf>,
    senders: Arc<Mutex<HashMap<Option<OwnedMatchRule>, MsgBroadcaster>>>,
    already_received_bytes: Option<Vec<u8>>,
    prev_seq: u64,
    activity_event: Arc<Event>,
}

impl SocketReader {
    pub fn new(
        socket: Box<dyn ReadHalf>,
        senders: Arc<Mutex<HashMap<Option<OwnedMatchRule>, MsgBroadcaster>>>,
        already_received_bytes: Vec<u8>,
        activity_event: Arc<Event>,
    ) -> Self {
        Self {
            socket,
            senders,
            already_received_bytes: Some(already_received_bytes),
            prev_seq: 0,
            activity_event,
        }
    }

    pub fn spawn(self, executor: &Executor<'_>) -> Task<()> {
        executor.spawn(self.receive_msg(), "socket reader")
    }

    // Keep receiving messages and put them on the queue.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", name = "socket reader", skip(self))
    )]
    async fn receive_msg(mut self) {
        loop {
            trace!("Waiting for message on the socket..");
            let msg = self.read_socket().await;
            match &msg {
                Ok(msg) => trace!("Message received on the socket: {:?}", msg),
                Err(e) => trace!("Error reading from the socket: {:?}", e),
            };

            let mut senders = self.senders.lock().await;
            for (rule, sender) in &*senders {
                if let Ok(msg) = &msg {
                    if let Some(rule) = rule.as_ref() {
                        match rule.matches(msg) {
                            Ok(true) => (),
                            Ok(false) => continue,
                            Err(e) => {
                                debug!("Error matching message against rule: {:?}", e);

                                continue;
                            }
                        }
                    }
                }

                if let Err(e) = sender.broadcast_direct(msg.clone()).await {
                    // An error would be due to either of these:
                    //
                    // 1. the channel is closed.
                    // 2. No active receivers.
                    //
                    // In either case, just log it.
                    trace!(
                        "Error broadcasting message to stream for `{:?}`: {:?}",
                        rule,
                        e
                    );
                }
            }
            trace!("Broadcasted to all streams: {:?}", msg);

            if msg.is_err() {
                senders.clear();
                trace!("Socket reading task stopped");

                return;
            }
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    async fn read_socket(&mut self) -> crate::Result<Message> {
        self.activity_event.notify(usize::MAX);
        let mut bytes = self
            .already_received_bytes
            .take()
            .unwrap_or_else(|| Vec::with_capacity(MIN_MESSAGE_SIZE));
        let mut pos = bytes.len();
        #[cfg(unix)]
        let mut fds = vec![];
        if pos < MIN_MESSAGE_SIZE {
            bytes.resize(MIN_MESSAGE_SIZE, 0);
            // We don't have enough data to make a proper message header yet.
            // Some partial read may be in raw_in_buffer, so we try to complete it
            // until we have MIN_MESSAGE_SIZE bytes
            //
            // Given that MIN_MESSAGE_SIZE is 16, this codepath is actually extremely unlikely
            // to be taken more than once
            while pos < MIN_MESSAGE_SIZE {
                let res = self.socket.recvmsg(&mut bytes[pos..]).await?;
                let len = {
                    #[cfg(unix)]
                    {
                        fds.extend(res.1);
                        res.0
                    }
                    #[cfg(not(unix))]
                    {
                        res
                    }
                };
                pos += len;
                if len == 0 {
                    return Err(crate::Error::InputOutput(
                        std::io::Error::new(
                            std::io::ErrorKind::UnexpectedEof,
                            "failed to receive message",
                        )
                        .into(),
                    ));
                }
            }
        }

        let (primary_header, fields_len) = PrimaryHeader::read(&bytes)?;
        let header_len = MIN_MESSAGE_SIZE + fields_len as usize;
        let body_padding = padding_for_8_bytes(header_len);
        let body_len = primary_header.body_len() as usize;
        let total_len = header_len + body_padding + body_len;
        if total_len > MAX_MESSAGE_SIZE {
            return Err(crate::Error::ExcessData);
        }

        // By this point we have a full primary header, so we know the exact length of the complete
        // message.
        bytes.resize(total_len, 0);

        // Now we have an incomplete message; read the rest
        while pos < total_len {
            let res = self.socket.recvmsg(&mut bytes[pos..]).await?;
            let read = {
                #[cfg(unix)]
                {
                    fds.extend(res.1);
                    res.0
                }
                #[cfg(not(unix))]
                {
                    res
                }
            };
            pos += read;
            if read == 0 {
                return Err(crate::Error::InputOutput(
                    std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "failed to receive message",
                    )
                    .into(),
                ));
            }
        }

        // If we reach here, the message is complete; return it
        let seq = self.prev_seq + 1;
        self.prev_seq = seq;
        let endian = Endian::from(primary_header.endian_sig());
        let ctxt = Context::new_dbus(endian, 0);
        #[cfg(unix)]
        let bytes = serialized::Data::new_fds(bytes, ctxt, fds);
        #[cfg(not(unix))]
        let bytes = serialized::Data::new(bytes, ctxt);
        Message::from_raw_parts(bytes, seq)
    }
}
