use std::{cell::RefCell, fmt::Debug};

use fyrox::{core::pool::Handle, scene::node::Node, script::ScriptMessagePayload};

#[derive(Default, Clone, Debug)]
pub struct MyEvent<T: ScriptMessagePayload + Clone> {
    subscribers: RefCell<Vec<Handle<Node>>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ScriptMessagePayload + Clone> MyEvent<T> {
    pub fn new() -> Self {
        Self {
            subscribers: RefCell::new(Vec::new()),
            _marker: std::marker::PhantomData,
        }
    }
    /// Subscribe to the event, returns a unique handle
    pub fn subscribe(&self, node: Handle<Node>) -> Handle<Node> {
        self.subscribers.borrow_mut().push(node);
        node
    }
    /// Unsubscribe using the handle returned by `subscribe`
    pub fn unsubscribe(&self, handle: Handle<Node>) {
        self.subscribers.borrow_mut().retain(|&id| id != handle);
    }
    /// Fire the event to all current subscribers
    pub fn fire(&self, data: T, context: &fyrox::script::ScriptContext) {
        for &subscriber in self.subscribers.borrow().iter() {
            context
                .message_sender
                .send_to_target(subscriber, data.clone());
        }
    }
}
