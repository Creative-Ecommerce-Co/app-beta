use near_sdk::{BorshDeserialize, BorshSerialize, AccountId};
use crate::models::Media; // Import Media struct

/// Represents the status of a review.
#[derive(BorshDeserialize, BorshSerialize)]
pub enum ReviewStatus {
    Approved,
    Pending,
    Rejected,
}

/// Represents a review for a product.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Review {
    pub reviewer_id: AccountId, // ID of the user who reviewed the product
    pub rating: u8,              // Rating given by the reviewer (1-5)
    pub comment: String,         // Review comment
    pub created_at: u64,         // Timestamp when the review was created
    pub updated_at: Option<u64>,  // Timestamp when the review was last updated
    pub status: ReviewStatus,     // Status of the review (approved, pending, rejected)
    pub helpful_votes: u32,       // Count of helpful votes
    pub is_verified: bool,         // Indicates if the reviewer is a verified purchaser
    pub response: Option<String>,  // Store owner's response to the review
    pub media: Vec<Media>,         // Media associated with the review
}

impl Review {
    /// Creates a new review.
    ///
    /// # Arguments
    ///
    /// * `reviewer_id` - ID of the user who is reviewing the product.
    /// * `rating` - Rating given by the reviewer (1-5).
    /// * `comment` - Review comment.
    /// * `created_at` - Timestamp when the review is created.
    /// * `is_verified` - Indicates if the reviewer is a verified purchaser.
    /// * `media` - Optional media associated with the review.
    pub fn new(reviewer_id: AccountId, rating: u8, comment: String, created_at: u64, is_verified: bool, media: Vec<Media>) -> Self {
        Self {
            reviewer_id,
            rating,
            comment,
            created_at,
            updated_at: None,
            status: ReviewStatus::Pending,
            helpful_votes: 0,
            is_verified,
            response: None,
            media, // Initialize media
        }
    }

    /// Updates the review with a new comment and rating.
    pub fn update(&mut self, rating: u8, comment: String, updated_at: u64) {
        self.rating = rating;
        self.comment = comment;
        self.updated_at = Some(updated_at);
    }

    /// Marks the review as helpful.
    pub fn mark_helpful(&mut self) {
        self.helpful_votes += 1;
    }

    /// Sets the status of the review.
    pub fn set_status(&mut self, status: ReviewStatus) {
        self.status = status;
    }

    /// Adds a response from the store owner to the review.
    ///
    /// # Arguments
    ///
    /// * `response` - The response from the store owner.
    pub fn add_response(&mut self, response: String) {
        self.response = Some(response);
    }

    /// Retrieves the response from the store owner.
    pub fn get_response(&self) -> Option<&String> {
        self.response.as_ref()
    }
}