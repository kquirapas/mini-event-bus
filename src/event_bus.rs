use super::event::*;
use anyhow::Result;
use tokio::{signal, sync::broadcast};

#[derive(Debug)]
pub struct EventBus {
    pub sender: broadcast::Sender<Event>,
    pub receiver: broadcast::Receiver<Event>,
}

impl EventBus {
    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = broadcast::channel(capacity);
        Self { sender, receiver }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: Event) -> Result<usize> {
        Ok(self.sender.send(event)?)
    }

    pub fn count_connections(&self) -> usize {
        self.sender.receiver_count()
    }

    pub async fn watch_for_shutdown(&mut self) -> Result<()> {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Received SIGINT");
                println!("Sending shutdown signal to all running services");

                self.publish(Event::stop()).unwrap();

                println!("Waiting for all connected services to shutdown");
            }
        }

        let expected_count = self.count_connections() - 1;
        let mut count = 0;

        loop {
            tokio::select! {
                result_event = self.receiver.recv() => {
                    let event = result_event?;

                    if let EventKind::Shutdown = event.get_kind() {
                        count += 1;
                    }
                }
            }

            // if num of received shutdown events == num of connected to event bus
            if count == expected_count {
                break;
            }
        }

        Ok(())
    }
}
