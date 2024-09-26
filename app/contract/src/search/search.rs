use near_sdk::AccountId;
use crate::models::{Product, Storefront};
use crate::storage::Storage;

pub fn search_products_by_name(storage: &Storage, name: String) -> Vec<Product> {
    let mut results = Vec::new();
    let user_ids = storage.get_all_users();
    for user_id in user_ids {
        if let Some(storefront) = storage.get_storefront(user_id) {
            let products = storefront.search_products_by_name(name.clone());
            results.extend(products);
        }
    }
    results
}

pub fn search_storefronts_by_name(storage: &Storage, name: String) -> Vec<Storefront> {
    let mut results = Vec::new();
    let user_ids = storage.get_all_users();
    for user_id in user_ids {
        if let Some(storefront) = storage.get_storefront(user_id.clone()) {
            if storefront.name.contains(&name) {
                results.push(storefront);
            }
        }
    }
    results
}

pub fn search_products_by_tag(storage: &Storage, tag: String) -> Vec<Product> {
    let mut results = Vec::new();
    let user_ids = storage.get_all_users();
    for user_id in user_ids {
        if let Some(storefront) = storage.get_storefront(user_id) {
            let products = storefront.get_all_products();
            for product in products {
                if product.tags.contains(&tag) {
                    results.push(product);
                }
            }
        }
    }
    results
}

pub fn search_products_by_collection(storage: &Storage, collection_name: String) -> Vec<Product> {
    let mut results = Vec::new();
    if let Some(product_ids) = storage.get_collection(collection_name) {
        for product_id in product_ids {
            let user_ids = storage.get_all_users();
            for user_id in user_ids {
                if let Some(product) = storage.get_product(user_id.clone(), product_id.clone()) {
                    results.push(product);
                }
            }
        }
    }
    results
}