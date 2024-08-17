mod cowsay;
mod event;
mod event_bus;
mod service;
mod utils;

use anyhow::Result;
use cowsay::Cowsay;
use event::Event;
use event_bus::EventBus;
use service::{Service, ServiceConfig};
use spinners::{Spinner, Spinners};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running Event Bus");
    let mut event_bus = EventBus::with_capacity(100);

    println!("Creating Cowsay Service");
    let cowsay_cfg = ServiceConfig::new(String::from("cowsay"), &event_bus);
    let mut cowsay_service = Cowsay::with_config(cowsay_cfg);

    println!("Preload some events for testing");

    event_bus.publish(Event::run())?;
    event_bus.publish(Event::run())?;
    event_bus.publish(Event::run())?;

    println!("Running services");

    // for coolio effect
    let mut spinner = Spinner::new(Spinners::Line, "Services are running...".into());

    let (cowsay, watch_for_shutdown) =
        tokio::join!(cowsay_service.start(), event_bus.watch_for_shutdown());

    cowsay?;
    watch_for_shutdown?;

    // stop coolio effect. LOL.
    spinner.stop_with_newline();

    println!("All services shutdown");

    println!("Network shutting down");

    Ok(())
}
