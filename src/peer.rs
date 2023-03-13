use tracing::{debug, info, instrument};

use crate::config::Config;
use crate::event::Event;
use crate::event_queue::{EventQueue, self};
use crate::state::State;
use crate::connection::Connection;

#[derive(Debug)]
pub struct Peer {
    state: State,
    event_queue: EventQueue,
    tcp_connection: Option<Connection>,
    config: Config,
}

impl Peer {
    pub fn new(config: Config) -> Self {
        let state = State::Idle;
        let event_queue = EventQueue::new();
        Self {
            state,
            event_queue,
            tcp_connection: None,
            config
        }
    }

    #[instrument]
    pub fn start(&mut self) {
        info!("peer is started.");
        self.event_queue.enqueue(Event::ManualStart);
    }

    #[instrument]
    pub async fn next(&mut self) {
        if let Some(event) = self.event_queue.dequeue() {
            info!("event is occured, event={:?}.", event);
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match &self.state {
            State::Idle => {
                match event {
                    Event::ManualStart => {
                        self.tcp_connection = Connection::connect(&self.config).await.ok();
                        if self.tcp_connection.is_some() {
                            self.event_queue.enqueue(Event::TcpConnectionConfirmed);
                        } else {
                            panic!("TCP Connection の確率ができませんでした。{:?}", self.config)
                        }
                        self.state = State::Connect;
                    },
                    _ => {}
                }
            },
            State::Connect => match event {
                Event::TcpConnectionConfirmed => {
                    self.state = State::OpenSent
                },
                _ => {}
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn peer_can_transition_to_connect_state() {
        let config: Config = "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let mut peer = Peer::new(config);
        peer.start();

        tokio::spawn(async move {
            let remote_config = "64513 127.0.0.2 65412 127.0.0.1 passive".parse().unwrap();
            let mut remote_peer = Peer::new(remote_config);
            remote_peer.start();
            remote_peer.next().await;
        });

        // 先に remote_peer 側の処理が進むことを保証するための wait
        tokio::time::sleep(Duration::from_secs(1)).await;
        peer.next().await;
        assert_eq!(peer.state, State::Connect);
    }
}
