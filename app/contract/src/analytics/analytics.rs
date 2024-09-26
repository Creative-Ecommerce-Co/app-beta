use near_sdk::{env, AccountId};
use crate::models::{Product, Storefront, Analytics};
use crate::storage::Storage;

pub fn track_product_view(storage: &mut Storage, user_id: AccountId, product_id: String) {
    if let Some(mut product) = storage.get_product(user_id.clone(), product_id.clone()) {
        product.views += 1; // Assuming Product has a views field
        storage.add_product(user_id, product_id, product);
    } else {
        env::panic(b"Product does not exist");
    }
}

pub fn track_product_sale(storage: &mut Storage, user_id: AccountId, product_id: String, quantity: u32) {
    if let Some(mut product) = storage.get_product(user_id.clone(), product_id.clone()) {
        product.sales += quantity; // Assuming Product has a sales field
        storage.add_product(user_id, product_id, product);
    } else {
        env::panic(b"Product does not exist");
    }
}

pub fn get_product_views(storage: &Storage, user_id: AccountId, product_id: String) -> u32 {
    if let Some(product) = storage.get_product(user_id, product_id) {
        product.views // Assuming Product has a views field
    } else {
        env::panic(b"Product does not exist");
    }
}

pub fn get_product_sales(storage: &Storage, user_id: AccountId, product_id: String) -> u32 {
    if let Some(product) = storage.get_product(user_id, product_id) {
        product.sales // Assuming Product has a sales field
    } else {
        env::panic(b"Product does not exist");
    }
}

pub fn get_storefront_analytics(storage: &Storage, user_id: AccountId) -> Analytics {
    if let Some(storefront) = storage.get_storefront(user_id) {
        let products = storefront.get_all_products();
        let total_views: u32 = products.iter().map(|p| p.views).sum();
        let total_sales: u32 = products.iter().map(|p| p.sales).sum();
        Analytics {
            total_views,
            total_sales,
        }
    } else {
        env::panic(b"Storefront does not exist");
    }
}