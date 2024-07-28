/* You use RefCell<> when circumistances force you to take only immutable reference, but
you really need to change the reference. For example, in our case, we have a trait that defines
the reference to self (&self) and this reference is immutable. But we need to change some field
of a sturct that implements that trait: */
use std::cell::RefCell;

trait Messenger<T> {
    fn send(&self, message: T); // defines immutable reference to self
}

struct MyMessenger {
    message_queue: RefCell<Vec<String>>,
}
impl MyMessenger {
    fn new() -> MyMessenger {
        MyMessenger {
            message_queue: RefCell::new(vec![]),
        }
    }
}
impl Messenger<&str> for MyMessenger {
    fn send(&self, message: &str) {
        self.message_queue.borrow_mut().push(message.to_string()); // &x.borrow_mut() -> &mut x
    }
}
// Note: you can't create 2 mutable reference, only .borrow() as many times as you want,
// or one .borrow_mut()

pub fn main() {
    let messenger = MyMessenger::new();
    dbg!(&messenger.message_queue);

    messenger.send("Message #11");
    messenger.send("Message #777");

    dbg!(&messenger.message_queue);
}
