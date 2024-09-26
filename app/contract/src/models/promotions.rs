use near_sdk::{BorshDeserialize, BorshSerialize};

/// Represents a discount that can be applied to a product.
#[derive(BorshDeserialize, BorshSerialize)]
pub enum DiscountType {
    Percentage(u8),               // Percentage discount (0-100)
    FixedAmount(u64),             // Fixed amount discount
    Timed {                       // Timed discount with start and end
        start_time: u64,
        end_time: u64,
        discount: Box<Discount>,  // Nested discount
    },
    ItemDiscount {                // Discount applicable to specific items
        item_ids: Vec<String>,     // List of item IDs
        discount: Box<Discount>,    // Nested discount
    },
    BundleDiscount {              // Discount for bundles of products
        bundle_ids: Vec<String>,    // List of product IDs in the bundle
        discount: Box<Discount>,     // Nested discount
    },
    Affiliate {                   // Affiliate discount
        affiliate_id: String,       // ID of the affiliate
        discount: Box<Discount>,     // Nested discount
    },
}

/// Represents a discount that can be applied to a product.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Discount {
    pub discount_type: DiscountType, // Type of discount
    pub valid_until: Option<u64>,      // Optional timestamp for when the discount expires
}

/// Implements methods related to discounts.
impl Discount {
    /// Creates a new discount.
    ///
    /// # Arguments
    ///
    /// * `discount_type` - The type of discount.
    /// * `valid_until` - An optional timestamp indicating when the discount expires.
    pub fn new(discount_type: DiscountType, valid_until: Option<u64>) -> Self {
        Self { discount_type, valid_until }
    }

    /// Checks if the discount is still valid based on the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `current_time` - The current timestamp to check against the discount's expiration.
    pub fn is_valid(&self, current_time: u64) -> bool {
        self.valid_until.map_or(true, |expiry| current_time < expiry)
    }
}