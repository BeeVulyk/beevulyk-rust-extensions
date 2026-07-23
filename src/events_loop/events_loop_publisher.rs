use std::sync::Arc;

use super::events_loop::EventsLoopMessage;

pub struct EventsLoopPublisher<TModel: Send + Sync + 'static> {
    name: Arc<String>,
    sender: tokio::sync::mpsc::UnboundedSender<EventsLoopMessage<TModel>>,
}

impl<TModel: Send + Sync + 'static> EventsLoopPublisher<TModel> {
    pub fn new(
        name: Arc<String>,
    ) -> (
        Self,
        tokio::sync::mpsc::UnboundedReceiver<EventsLoopMessage<TModel>>,
    ) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let result = Self { name, sender };

        (result, receiver)
    }

    pub fn send(&self, model: TModel) {
        if let Err(_) = self.sender.send(EventsLoopMessage::NewMessage(model)) {
            println!("Can not send model to event loop {}", self.name.as_str());
        }
    }

    pub fn stop(&self) {
        if let Err(err) = self.sender.send(EventsLoopMessage::Shutdown) {
            tracing::error!(?err, name = %self.name, "Can not send shutdown message to event loop.");
        }
    }
}
