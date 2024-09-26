// app/contract/src/models/product.rs

use near_sdk::{BorshDeserialize, BorshSerialize, AccountId};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Product {
    pub owner_id: AccountId,      // ID of the user who owns the product
    pub name: String,
    pub price: u64,
    pub description: String,
    pub collection: Option<String>, // Collection the product belongs to
    pub tags: Vec<String>,          // Tags for the product
    pub status: ProductStatus,      // Status of the product (active, inactive, etc.)
    pub media: Vec<Media>,          // Media associated with the product (images, videos)
    pub variations: Vec<Variation>,  // Variations of the product (size, color, etc.)
    pub reviews: Vec<Review>,       // Reviews for the product
    pub discount: Option<Discount>,  // Optional discount for the product
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Media {
    pub media_type: MediaType, // Type of media (image, video)
    pub url: String,            // URL of the media
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum MediaType {
    Image,
    Video,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Variation {
    pub id: String,             // Unique identifier for the variation
    pub attributes: Vec<Attribute>, // Attributes for the variation (size, color, etc.)
    pub price: u64,             // Price for the variation
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Attribute {
    pub name: String,           // Name of the attribute (e.g., "Size")
    pub value: String,          // Value of the attribute (e.g., "Large")
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Review {
    pub reviewer_id: AccountId, // ID of the user who reviewed the product
    pub rating: u8,              // Rating given by the reviewer (1-5)
    pub comment: String,         // Review comment
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Discount {
    pub discount_type: DiscountType, // Type of discount
    pub valid_until: Option<u64>,      // Optional timestamp for when the discount expires
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum DiscountType {
    Percentage(u8),               // Percentage discount (0-100)
    FixedAmount(u64),             // Fixed amount discount
    Timed {                       // Timed discount with start and end
        start_time: u64,
        end_time: u64,
    },
}