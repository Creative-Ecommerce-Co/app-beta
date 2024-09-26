// app/contract/src/models/subscription.rs

use near_sdk::{BorshDeserialize, BorshSerialize, AccountId};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SubscriptionTier {
    pub name: String,          // Name of the subscription tier
    pub price: u64,            // Monthly price of the subscription tier
    pub resource_limit: u32,   // Resource limit for the tier (e.g., number of products, storage)
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserSubscription {
    pub user_id: AccountId,            // ID of the user subscribed
    pub tier: SubscriptionTier,         // Current subscription tier
    pub expiration: u64,                // Expiration timestamp of the subscription
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Subscription {
    pub tiers: Vec<SubscriptionTier>,   // List of available subscription tiers
}

impl Subscription {
    /// Creates a new subscription with predefined tiers.
    pub fn new() -> Self {
        Self {
            tiers: vec![
                SubscriptionTier {
                    name: "Basic".to_string(),
                    price: 1000, // Price in yoctoNEAR (1 NEAR = 10^24 yoctoNEAR)
                    resource_limit: 10, // Limit for products
                },
                SubscriptionTier {
                    name: "Pro".to_string(),
                    price: 3000,
                    resource_limit: 100,
                },
                SubscriptionTier {
                    name: "Enterprise".to_string(),
                    price: 5000,
                    resource_limit: 1000,
                },
            ],
        }
    }

    /// Retrieves the subscription tier by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the subscription tier to retrieve.
    pub fn get_tier(&self, name: &str) -> Option<&SubscriptionTier> {
        self.tiers.iter().find(|tier| tier.name == name)
    }

    /// Checks if a user subscription is still valid.
    ///
    /// # Arguments
    ///
    /// * `subscription` - The user subscription to check.
    /// * `current_time` - The current timestamp to check against the subscription's expiration.
    pub fn is_subscription_valid(&self, subscription: &UserSubscription, current_time: u64) -> bool {
        subscription.expiration > current_time
    }
}