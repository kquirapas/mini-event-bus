use super::{
    event::{Event, EventKind},
    service::{Service, ServiceConfig},
    utils::*,
};
use anyhow::Result;
use async_trait::async_trait;

/// Cowsay Service
///
/// Cowsay Service does nothing if self.config is None
/// forcing a drop for shutdown
pub struct Cowsay {
    config: Option<ServiceConfig>,
}

#[async_trait]
impl Service for Cowsay {
    fn with_config(cfg: ServiceConfig) -> Self {
        Cowsay { config: Some(cfg) }
    }

    async fn start(&mut self) -> Result<()> {
        if self.config.is_none() {
            return Ok(());
        }

        println!("Running Cowsay service");

        // run module in perpetuity until a shutdown is received
        loop {
            tokio::select! {
                // ok to unwrap, None previously checked
                event = self.config.as_mut().unwrap().receiver.recv() => {
                    if let Err(e) = &event {
                        self.log(e.to_string());
                    }

                    match event.unwrap().get_kind() {
                        EventKind::Ping => {
                            self.log(String::from("Pong!"));
                        }

                        EventKind::Run => {
                            get_cow_say();
                        }

                        EventKind::Stop => {
                            self.log(String::from("Received shutdown request"));
                            self.shutdown().await?;
                            break;
                        }

                        EventKind::Stub(stub) => {
                            self.log(format!("Received stub: {}", stub));
                        },

                        // Events not processed by Cowsay Service
                        _ => {
                            // Shutdown
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        if self.config.is_none() {
            return Ok(());
        }

        self.log(String::from("Cleaning up"));
        self.log(String::from("Shutting down"));
        self.config
            .as_mut()
            // ok to unwrap, None previously checked
            .unwrap()
            .sender
            .send(Event::shutdown())?;

        // drop ServiceConfig to drop receiver
        self.config = None;
        Ok(())
    }

    fn log(&mut self, message: String) {
        if self.config.is_some() {
            println!("{}: {}", self.config.as_mut().unwrap().name, message);
        }
    }
}
