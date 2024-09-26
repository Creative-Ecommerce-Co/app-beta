use near_sdk::AccountId;
use crate::storage::Storage;

pub struct Notification {
    pub recipient: AccountId,
    pub message: String,
    pub timestamp: u64,
}

impl Notification {
    pub fn new(recipient: AccountId, message: String) -> Self {
        Self {
            recipient,
            message,
            timestamp: env::block_timestamp(),
        }
    }
}

pub fn send_notification(storage: &mut Storage, recipient: AccountId, message: String) {
    let notification = Notification::new(recipient.clone(), message);
    storage.add_notification(recipient, notification);
}

pub fn get_notifications(storage: &Storage, user_id: AccountId) -> Vec<Notification> {
    storage.get_notifications(user_id)
}

pub fn remove_notification(storage: &mut Storage, user_id: AccountId, timestamp: u64) {
    storage.remove_notification(user_id, timestamp);
}