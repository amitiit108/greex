pub struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Notifier
    }

    pub fn send_notification(&self, message: &str) {
        println!("Notification: {}", message);
    }
}
