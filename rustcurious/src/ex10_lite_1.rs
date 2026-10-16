// https://rustcurious.com/10

// Exercise: ChannelMonitor Lite
//
// Fix the borrowing error three different ways:
//
// 1. Change the *order* of access
//      Instead of getting `sender` from `self.connection`
//      at the top of `handle_event`, do it in `send`.
//
// 2. Change the *area* of data borrowed
//      Revert the change from the step above. Instead:
//      Change `send` from a method to a function,
//      and give it references to only the data it needs.
//
// 3. Change the *data layout* to match the access
//      Revert the changes from the steps above. Instead:
//      Move `send` from ChannelMonitor to a new type
//      that collects together the fields `send` mutates.

type Sender = std::sync::mpsc::Sender<String>;

pub struct ChannelMonitor {
    connection: Option<Sender>,
    sent_ok: u64,
    sent_err: u64,
}

impl ChannelMonitor {
    pub fn new() -> Self {
        ChannelMonitor {
            connection: None,
            sent_ok: 0,
            sent_err: 0,
        }
    }

    pub fn connect(&mut self, sender: Sender) {
        self.connection = Some(sender);
    }

    pub fn handle_event(&mut self, event: Event) {
        let msg = match event {
            Event::NewSubscriber(name) => format!("New sub: {}", name),
            Event::NewComment(comment) => format!("Comment: {}", comment),
        };
        self.send(msg);
    }

    fn send(&mut self, msg: String) {
        let Some(sender) = &self.connection else {
            return;
        };
        match sender.send(msg) {
            Ok(()) => self.sent_ok += 1,
            Err(_) => self.sent_err += 1,
        }
    }

    pub fn sent_ok(&self) -> u64 {
        self.sent_ok
    }

    pub fn sent_err(&self) -> u64 {
        self.sent_err
    }
}

// -------------------------------------------------------
// No need to change anything below this line.

pub enum Event {
    NewSubscriber(String),
    NewComment(String),
}

#[test]
fn test() {
    use std::sync::mpsc;

    let mut cm = ChannelMonitor::new();
    cm.handle_event(Event::NewComment("first post".into()));
    let (sender, receiver) = mpsc::channel();
    cm.connect(sender);
    cm.handle_event(Event::NewSubscriber("hugebrain".into()));
    cm.handle_event(Event::NewComment("c++ 4eva".into()));
    assert_eq!(cm.sent_ok(), 2);
    assert_eq!(receiver.recv().unwrap(), "New sub: hugebrain");
    assert_eq!(receiver.recv().unwrap(), "Comment: c++ 4eva");

    // Sending after the receiver is dropped causes an error
    std::mem::drop(receiver);
    cm.handle_event(Event::NewSubscriber("chad".into()));
    assert_eq!(cm.sent_err(), 1);
}
