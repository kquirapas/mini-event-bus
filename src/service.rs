use super::{event::*, event_bus::*};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait Service {
    fn with_config(cfg: ServiceConfig) -> Self;
    async fn start(&mut self) -> Result<()>;
    async fn shutdown(&mut self) -> Result<()>;
    fn log(&mut self, message: String);
}

pub struct ServiceConfig {
    pub name: String,
    pub sender: broadcast::Sender<Event>,
    pub receiver: broadcast::Receiver<Event>,
}

impl ServiceConfig {
    pub fn new(name: String, event_bus: &EventBus) -> Self {
        ServiceConfig {
            name,
            sender: event_bus.sender.clone(),
            receiver: event_bus.subscribe(),
        }
    }
}
