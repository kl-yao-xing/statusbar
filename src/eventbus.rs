use time::OffsetDateTime;
use tokio::sync::mpsc;

use crate::eventbus;

#[derive(Debug, Clone)]
pub enum Event {
    TimeTick(OffsetDateTime),
    FormattedTime(String),
}

#[derive(Clone)]
pub struct EventBus {
    sender: mpsc::UnboundedSender<Event>,
}

pub struct EventConsumer {
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventBus {
    pub fn new() -> (Self, EventConsumer) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let eventbus = Self { sender };
        let consumer = EventConsumer::new(receiver);
        (eventbus, consumer)
    }
    pub fn send_event(&self, evt:Event) -> Result<(), mpsc::error::SendError<Event>> {
        self.sender.send(evt)
    }
}

impl EventConsumer {
    pub fn new(receiver: mpsc::UnboundedReceiver<Event>) -> Self {
        Self { receiver }
    }

    pub async fn recv(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
    pub async fn run(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match event {
            Event::TimeTick(time) => {
                // do something
            }
            Event::FormattedTime(formatted_time) => {
                // do something
            }
            _ => {
                // do otherthing
            }
        }
    }
}