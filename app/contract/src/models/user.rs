use near_sdk::{BorshDeserialize, BorshSerialize, AccountId};
use sha2::{Sha256, Digest}; // For hashing passwords

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub id: AccountId,                // Unique identifier for the user
    pub name: String,                  // User's name
    pub email: String,                 // User's email
    pub hashed_password: String,       // Hashed password for account security
    pub phone_number: Option<String>,  // User's phone number
    pub storefront_id: Option<String>, // ID of the user's storefront
    pub preferences: UserPreferences,   // User preferences
    pub is_two_factor_enabled: bool,   // Flag for two-factor authentication
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserPreferences {
    pub language: String,              // Preferred language
    pub notifications_enabled: bool,   // Notification preferences
}

impl User {
    /// Hashes the password using SHA-256.
    pub fn set_password(&mut self, password: &str) {
        let mut hasher = Sha256::new();
        hasher.update(password);
        self.hashed_password = format!("{:x}", hasher.finalize());
    }

    /// Validates the provided password against the stored hashed password.
    pub fn validate_password(&self, password: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(password);
        let hashed_input = format!("{:x}", hasher.finalize());
        self.hashed_password == hashed_input
    }

    /// Sets the user's phone number.
    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number = Some(phone_number);
    }

    /// Retrieves the user's phone number.
    pub fn get_phone_number(&self) -> Option<&String> {
        self.phone_number.as_ref()
    }
}