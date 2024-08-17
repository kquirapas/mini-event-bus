#[derive(Clone, Debug, PartialEq)]
pub enum EventKind {
    Stub(String),
    Run,
    Stop,
    Ping,
    Pong,
    Shutdown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    kind: EventKind,
}

impl Event {
    pub fn get_kind(&self) -> EventKind {
        self.kind.clone()
    }

    pub fn stub(stub: String) -> Event {
        Event {
            kind: EventKind::Stub(stub),
        }
    }

    pub fn run() -> Event {
        Event {
            kind: EventKind::Run,
        }
    }

    pub fn stop() -> Event {
        Event {
            kind: EventKind::Stop,
        }
    }

    pub fn ping() -> Event {
        Event {
            kind: EventKind::Ping,
        }
    }

    pub fn shutdown() -> Event {
        Event {
            kind: EventKind::Shutdown,
        }
    }
}
