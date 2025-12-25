use tokio::sync::mpsc;


#[derive(Debug, Clone)]
pub enum Event {
    TimeUpdated(String),
}

#[derive(Clone)]
pub struct EventBus {
    sender: mpsc::UnboundedSender<Event>,
}

pub struct EventConsumer {
    pub(crate) receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventBus {
    pub fn new() -> (Self, EventConsumer) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let eventbus = Self { sender };
        let consumer = EventConsumer {receiver};
        (eventbus, consumer)
    }
    pub fn send_event(&self, evt:Event) -> Result<(), mpsc::error::SendError<Event>> {
        self.sender.send(evt)
    }
}
