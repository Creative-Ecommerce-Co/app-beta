use near_sdk::{BorshDeserialize, BorshSerialize, AccountId};
use crate::models::Product;

/// Represents an order placed by a user.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Order {
    pub order_id: String,                // Unique identifier for the order
    pub user_id: AccountId,              // ID of the user who placed the order
    pub products: Vec<OrderItem>,        // List of products in the order
    pub total_amount: u64,               // Total amount for the order
    pub status: OrderStatus,              // Current status of the order
    pub created_at: u64,                  // Timestamp when the order was created
    pub tracking_number: Option<String>,   // Tracking number for shipment
    pub tracking_url: Option<String>,      // URL for tracking the shipment
    pub store_owner_id: AccountId,        // ID of the store owner
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OrderItem {
    pub product_id: String,               // ID of the product
    pub quantity: u32,                    // Quantity of the product ordered
    pub price: u64,                       // Price of the product at the time of order
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum OrderStatus {
    Pending,                              // Order has been placed but not yet processed
    Confirmed,                            // Order has been confirmed
    Shipped,                              // Order has been shipped
    Delivered,                            // Order has been delivered
    Canceled,                             // Order has been canceled
}

impl Order {
    /// Creates a new order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Unique identifier for the order.
    /// * `user_id` - ID of the user who placed the order.
    /// * `products` - List of products in the order.
    /// * `total_amount` - Total amount for the order.
    /// * `created_at` - Timestamp when the order was created.
    /// * `store_owner_id` - ID of the store owner to send alerts.
    pub fn new(order_id: String, user_id: AccountId, products: Vec<OrderItem>, total_amount: u64, created_at: u64, store_owner_id: AccountId) -> Self {
        Self {
            order_id,
            user_id,
            products,
            total_amount,
            status: OrderStatus::Pending,
            created_at,
            tracking_number: None,
            tracking_url: None,
            store_owner_id,
        }
    }

    /// Cancels the order if it is still pending and sends an alert to the store owner.
    pub fn cancel(&mut self) -> Result<(), String> {
        if self.status == OrderStatus::Pending {
            self.status = OrderStatus::Canceled;
            self.send_alert("Order has been canceled".to_string());
            Ok(())
        } else {
            Err("Order cannot be canceled".to_string())
        }
    }

    /// Initiates a return for the order and sends an alert to the store owner.
    pub fn initiate_return(&mut self) -> Result<(), String> {
        if self.status == OrderStatus::Delivered {
            self.status = OrderStatus::Canceled; // Change status to canceled for simplicity
            self.send_alert("Return initiated for order".to_string());
            Ok(())
        } else {
            Err("Order cannot be returned".to_string())
        }
    }

    /// Sends an alert to the store owner.
    fn send_alert(&self, message: String) {
        // Logic to send alert (e.g., email, SMS, in-app notification)
        // This is a placeholder for the actual notification implementation
        println!("Alert to {}: {}", self.store_owner_id, message);
    }
}