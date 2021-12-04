use stubs::hook::v0::StreamPlayerConnectEventsResponse;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ConnectionEvent {
    stream: broadcast::Sender<StreamPlayerConnectEventsResponse>,
}

impl ConnectionEvent {
    pub fn subscribe(&self) -> broadcast::Receiver<StreamPlayerConnectEventsResponse> {
        self.stream.subscribe()
    }

    pub fn handle_event(&self, addr: String, name: String, ucid: String, id: u32) {
        log::info!("ConnectionEvent::handle_event");

        // if there are no active streams, ignore the message
        if self.stream.receiver_count() == 0 {
            log::info!("ConnectionEvent::handle_event - no active streams");
            return;
        }

        self.stream
            .send(StreamPlayerConnectEventsResponse {
                addr,
                name,
                ucid,
                id
            })
            .ok();
    }
}

impl Default for ConnectionEvent {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(256);
        Self { stream: tx }
    }
}
