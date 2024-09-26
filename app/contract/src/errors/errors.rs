use near_sdk::env;

#[derive(Debug)]
pub enum ContractError {
    StorefrontAlreadyExists,
    StorefrontDoesNotExist,
    ProductDoesNotExist,
    SubscriptionTierDoesNotExist,
    UserSubscriptionDoesNotExist,
    Unauthorized,
}

impl ContractError {
    pub fn panic(&self) -> ! {
        match self {
            ContractError::StorefrontAlreadyExists => env::panic(b"Storefront already exists for this user"),
            ContractError::StorefrontDoesNotExist => env::panic(b"Storefront does not exist for this user"),
            ContractError::ProductDoesNotExist => env::panic(b"Product does not exist"),
            ContractError::SubscriptionTierDoesNotExist => env::panic(b"Subscription tier does not exist"),
            ContractError::UserSubscriptionDoesNotExist => env::panic(b"User subscription does not exist"),
            ContractError::Unauthorized => env::panic(b"Unauthorized action"),
        }
    }
}