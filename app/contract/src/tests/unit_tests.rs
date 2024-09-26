// app/contract/src/tests/unit_tests.rs

use near_sdk::{AccountId};
use crate::models::{Product, Storefront, ProductStatus};

#[test]
fn test_storefront_add_product() {
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
    let mut storefront = Storefront::new(user_id.clone());
    let product_id = "product1".to_string();
    let product = Product {
        owner_id: user_id.clone(),
        name: "Test Product".to_string(),
        price: 100,
        description: "A test product".to_string(),
        collection: None,
        tags: vec![],
        status: ProductStatus::Active,
        media: vec![],
        variations: vec![],
        reviews: vec![],
        discount: None,
    };

    // Add product to the storefront
    storefront.add_product(product_id.clone(), product.clone());

    // Retrieve the product
    let retrieved_product = storefront.get_product(product_id.clone()).unwrap();
    assert_eq!(retrieved_product.name, product.name);
}

#[test]
fn test_storefront_remove_product() {
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
    let mut storefront = Storefront::new(user_id.clone());
    let product_id = "product1".to_string();
    let product = Product {
        owner_id: user_id.clone(),
        name: "Test Product".to_string(),
        price: 100,
        description: "A test product".to_string(),
        collection: None,
        tags: vec![],
        status: ProductStatus::Active,
        media: vec![],
        variations: vec![],
        reviews: vec![],
        discount: None,
    };

    // Add product to the storefront
    storefront.add_product(product_id.clone(), product.clone());

    // Remove the product
    storefront.remove_product(product_id.clone());

    // Ensure the product is removed
    assert!(storefront.get_product(product_id).is_none());
}

#[test]
fn test_storefront_update_product() {
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
    let mut storefront = Storefront::new(user_id.clone());
    let product_id = "product1".to_string();
    let mut product = Product {
        owner_id: user_id.clone(),
        name: "Test Product".to_string(),
        price: 100,
        description: "A test product".to_string(),
        collection: None,
        tags: vec![],
        status: ProductStatus::Active,
        media: vec![],
        variations: vec![],
        reviews: vec![],
        discount: None,
    };

    // Add product to the storefront
    storefront.add_product(product_id.clone(), product.clone());

    // Update the product
    product.price = 150;
    storefront.update_product(product_id.clone(), product.clone());

    // Retrieve the updated product
    let updated_product = storefront.get_product(product_id.clone()).unwrap();
    assert_eq!(updated_product.price, 150);
}

#[test]
fn test_storefront_remove_nonexistent_product() {
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
    let mut storefront = Storefront::new(user_id.clone());
    let product_id = "product1".to_string();

    // Attempt to remove a product that doesn't exist
    storefront.remove_product(product_id.clone());

    // Ensure that no panic occurs and the product is still nonexistent
    assert!(storefront.get_product(product_id).is_none());
}

#[test]
fn test_storefront_add_duplicate_product() {
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
    let mut storefront = Storefront::new(user_id.clone());
    let product_id = "product1".to_string();
    let product = Product {
        owner_id: user_id.clone(),
        name: "Test Product".to_string(),
        price: 100,
        description: "A test product".to_string(),
        collection: None,
        tags: vec![],
        status: ProductStatus::Active,
        media: vec![],
        variations: vec![],
        reviews: vec![],
        discount: None,
    };

    // Add product to the storefront
    storefront.add_product(product_id.clone(), product.clone());

    // Attempt to add the same product again
    storefront.add_product(product_id.clone(), product.clone());

    // Retrieve the product and ensure it hasn't changed
    let retrieved_product = storefront.get_product(product_id.clone()).unwrap();
    assert_eq!(retrieved_product.price, 100);
}

#[test]
fn test_storefront_get_all_products() {
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
    let mut storefront = Storefront::new(user_id.clone());

    let product1 = Product {
        owner_id: user_id.clone(),
        name: "Product 1".to_string(),
        price: 100,
        description: "First product".to_string(),
        collection: None,
        tags: vec![],
        status: ProductStatus::Active,
        media: vec![],
        variations: vec![],
        reviews: vec![],
        discount: None,
    };

    let product2 = Product {
        owner_id: user_id.clone(),
        name: "Product 2".to_string(),
        price: 200,
        description: "Second product".to_string(),
        collection: None,
        tags: vec![],
        status: ProductStatus::Active,
        media: vec![],
        variations: vec![],
        reviews: vec![],
        discount: None,
    };

    // Add products to the storefront
    storefront.add_product("product1".to_string(), product1);
    storefront.add_product("product2".to_string(), product2);

    // Retrieve all products
    let all_products = storefront.get_all_products();
    assert_eq!(all_products.len(), 2);
    assert_eq!(all_products[0].name, "Product 1");
    assert_eq!(all_products[1].name, "Product 2");
}