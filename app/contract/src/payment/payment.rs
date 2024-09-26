use near_sdk::{env, AccountId};
use crate::models::{Order, OrderStatus};
use crate::storage::Storage;

pub fn process_payment(storage: &mut Storage, order_id: String, user_id: AccountId, amount: u64) -> Result<(), String> {
    if let Some(mut order) = storage.get_order(order_id.clone()) {
        if order.user_id == user_id && order.total_amount == amount {
            order.status = OrderStatus::Confirmed;
            storage.update_order(order_id, order);
            Ok(())
        } else {
            Err("Payment details do not match the order".to_string())
        }
    } else {
        Err("Order does not exist".to_string())
    }
}

pub fn refund_payment(storage: &mut Storage, order_id: String, user_id: AccountId) -> Result<(), String> {
    if let Some(mut order) = storage.get_order(order_id.clone()) {
        if order.user_id == user_id && order.status == OrderStatus::Confirmed {
            order.status = OrderStatus::Canceled;
            storage.update_order(order_id, order);
            Ok(())
        } else {
            Err("Order cannot be refunded".to_string())
        }
    } else {
        Err("Order does not exist".to_string())
    }
}

pub fn get_payment_status(storage: &Storage, order_id: String) -> Result<OrderStatus, String> {
    if let Some(order) = storage.get_order(order_id) {
        Ok(order.status)
    } else {
        Err("Order does not exist".to_string())
    }
}