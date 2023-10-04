use std::{
    collections::HashMap,
    sync::{self, Arc},
};

use futures_util::future::poll_fn;
use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument, trace};

use crate::{
    async_lock::Mutex, raw::Connection as RawConnection, Executor, MsgBroadcaster, OwnedMatchRule,
    Socket, Task,
};

#[derive(Debug)]
pub(crate) struct SocketReader {
    raw_conn: Arc<sync::Mutex<RawConnection<Box<dyn Socket>>>>,
    senders: Arc<Mutex<HashMap<Option<OwnedMatchRule>, MsgBroadcaster>>>,
}

impl SocketReader {
    pub fn new(
        raw_conn: Arc<sync::Mutex<RawConnection<Box<dyn Socket>>>>,
        senders: Arc<Mutex<HashMap<Option<OwnedMatchRule>, MsgBroadcaster>>>,
    ) -> Self {
        Self { raw_conn, senders }
    }

    pub fn spawn(self, executor: &Executor<'_>, cancellation_token: Option<CancellationToken>) -> Task<()> {
        executor.spawn(self.receive_msg(cancellation_token), "socket reader")
    }

    // Keep receiving messages and put them on the queue.
    #[instrument(name = "socket reader", skip(self))]
    async fn receive_msg(self, cancellation_token: Option<CancellationToken>) {
        loop {
            trace!("Waiting for message on the socket..");

            let poll_task = poll_fn(|cx| {
                let mut raw_conn = self.raw_conn.lock().expect("poisoned lock");
                raw_conn.try_receive_message(cx)
            });

            // Stop receiving new messages from dbus.
            let msg = match &cancellation_token {
                Some(token) => {
                    tokio::select! {
                        _ = token.cancelled() => {
                            self.senders.lock().await.clear();
                            trace!("Socket reading task stopped with token");

                            return;
                        },
                        x = poll_task => x.map(Arc::new)
                    }},
                None => poll_task.await.map(Arc::new)
            };

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
