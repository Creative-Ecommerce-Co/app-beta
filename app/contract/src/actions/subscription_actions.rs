use near_sdk::{env, AccountId};
use crate::models::{Subscription, SubscriptionTier, UserSubscription};
use crate::storage::Storage;

pub fn add_subscription_tier(storage: &mut Storage, tier: SubscriptionTier) {
    storage.add_subscription_tier(tier);
}

pub fn update_subscription_tier(storage: &mut Storage, tier_name: String, updated_tier: SubscriptionTier) {
    if storage.get_subscription_tier(tier_name.clone()).is_some() {
        storage.update_subscription_tier(tier_name, updated_tier);
    } else {
        env::panic(b"Subscription tier does not exist");
    }
}

pub fn get_subscription_tier(storage: &Storage, tier_name: String) -> Option<SubscriptionTier> {
    storage.get_subscription_tier(tier_name)
}

pub fn remove_subscription_tier(storage: &mut Storage, tier_name: String) {
    if storage.get_subscription_tier(tier_name.clone()).is_some() {
        storage.remove_subscription_tier(tier_name);
    } else {
        env::panic(b"Subscription tier does not exist");
    }
}

pub fn subscribe_user(storage: &mut Storage, user_id: AccountId, tier_name: String, expiration: u64) {
    if let Some(tier) = storage.get_subscription_tier(tier_name.clone()) {
        let user_subscription = UserSubscription {
            user_id: user_id.clone(),
            tier,
            expiration,
        };
        storage.add_user_subscription(user_id, user_subscription);
    } else {
        env::panic(b"Subscription tier does not exist");
    }
}

pub fn get_user_subscription(storage: &Storage, user_id: AccountId) -> Option<UserSubscription> {
    storage.get_user_subscription(user_id)
}

pub fn unsubscribe_user(storage: &mut Storage, user_id: AccountId) {
    if storage.get_user_subscription(user_id.clone()).is_some() {
        storage.remove_user_subscription(user_id);
    } else {
        env::panic(b"User subscription does not exist");
    }
}