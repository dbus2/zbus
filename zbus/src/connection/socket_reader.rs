use std::{collections::HashMap, sync::Arc};

use tracing::{debug, instrument, trace};

use crate::{async_lock::Mutex, connection::MsgBroadcaster, Executor, OwnedMatchRule, Task};

use super::raw::{
    socket::{ReadHalf, WriteHalf},
    Connection as RawConnection,
};

#[derive(Debug)]
pub(crate) struct SocketReader {
    raw_conn: Arc<RawConnection<Box<dyn ReadHalf>, Box<dyn WriteHalf>>>,
    senders: Arc<Mutex<HashMap<Option<OwnedMatchRule>, MsgBroadcaster>>>,
}

impl SocketReader {
    pub fn new(
        raw_conn: Arc<RawConnection<Box<dyn ReadHalf>, Box<dyn WriteHalf>>>,
        senders: Arc<Mutex<HashMap<Option<OwnedMatchRule>, MsgBroadcaster>>>,
    ) -> Self {
        Self { raw_conn, senders }
    }

    pub fn spawn(self, executor: &Executor<'_>) -> Task<()> {
        executor.spawn(self.receive_msg(), "socket reader")
    }

    // Keep receiving messages and put them on the queue.
    #[instrument(name = "socket reader", skip(self))]
    async fn receive_msg(self) {
        loop {
            trace!("Waiting for message on the socket..");
            let msg = self.raw_conn.try_receive_message().await.map(Arc::new);
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

                if let Err(e) = sender.broadcast(msg.clone()).await {
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
}
