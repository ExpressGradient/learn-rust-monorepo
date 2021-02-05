pub trait Messenger {
    fn send_message(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: u32,
    max: u32
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: u32) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;

        if self.value > self.max {
            self.messenger.send_message("Your free plan has expired");
        } else {
            self.messenger.send_message("You are still in your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send_message(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn test_sent_messages() {
        let mock_messenger: MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new(&mock_messenger, 10);

        limit_tracker.set_value(5);

        assert_eq!(mock_messenger.sent_messages.borrow().get(0).unwrap(), "You are still in your quota");
    }
}