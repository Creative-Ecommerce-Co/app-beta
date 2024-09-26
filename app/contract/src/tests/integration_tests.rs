// app/contract/src/tests/integration_tests.rs

use near_sdk::{env, testing_env, AccountId};
use near_sdk::MockedBlockchain;
use crate::actions::{add_product, update_product, get_product, remove_product};
use crate::models::{Product, Storefront, ProductStatus};
use crate::storage::Storage;
use crate::container_management::{ContainerDeployment, Container, ContainerStatus};

#[test]
fn test_add_product() {
    let mut storage = Storage::new();
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
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

    // Add a storefront for the user
    let storefront = Storefront::new(user_id.clone());
    storage.add_storefront(user_id.clone(), storefront);

    // Add product to the storefront
    add_product(&mut storage, user_id.clone(), product_id.clone(), product.clone());

    // Retrieve the product
    let retrieved_product = get_product(&storage, user_id.clone(), product_id.clone()).unwrap();
    assert_eq!(retrieved_product.name, product.name);
}

#[test]
fn test_update_product() {
    let mut storage = Storage::new();
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
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

    // Add a storefront for the user
    let storefront = Storefront::new(user_id.clone());
    storage.add_storefront(user_id.clone(), storefront);

    // Add product to the storefront
    add_product(&mut storage, user_id.clone(), product_id.clone(), product.clone());

    // Update the product
    product.price = 150;
    update_product(&mut storage, user_id.clone(), product_id.clone(), product.clone());

    // Retrieve the updated product
    let updated_product = get_product(&storage, user_id.clone(), product_id.clone()).unwrap();
    assert_eq!(updated_product.price, 150);
}

#[test]
fn test_remove_product() {
    let mut storage = Storage::new();
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
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

    // Add a storefront for the user
    let storefront = Storefront::new(user_id.clone());
    storage.add_storefront(user_id.clone(), storefront);

    // Add product to the storefront
    add_product(&mut storage, user_id.clone(), product_id.clone(), product.clone());

    // Remove the product
    remove_product(&mut storage, user_id.clone(), product_id.clone());

    // Ensure the product is removed
    assert!(get_product(&storage, user_id.clone(), product_id.clone()).is_none());
}

#[test]
fn test_add_product_to_nonexistent_storefront() {
    let mut storage = Storage::new();
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
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

    // Attempt to add a product without a storefront
    let result = std::panic::catch_unwind(|| {
        add_product(&mut storage, user_id.clone(), product_id.clone(), product.clone());
    });

    assert!(result.is_err());
}

#[test]
fn test_update_nonexistent_product() {
    let mut storage = Storage::new();
    let user_id = AccountId::new_unchecked("user.testnet".to_string());
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

    // Add a storefront for the user
    let storefront = Storefront::new(user_id.clone());
    storage.add_storefront(user_id.clone(), storefront);

    // Attempt to update a product that doesn't exist
    let result = std::panic::catch_unwind(|| {
        update_product(&mut storage, user_id.clone(), product_id.clone(), product.clone());
    });

    assert!(result.is_err());
}

#[test]
fn test_container_management() {
    let mut container_deployment = ContainerDeployment::new();
    let owner_id = AccountId::new_unchecked("user.testnet".to_string());
    let container_id = "container1".to_string();

    // Deploy a new container
    container_deployment.deploy_container(owner_id.clone(), container_id.clone());

    // Check if the container is deployed
    let container = container_deployment.containers.iter().find(|c| c.id == container_id).unwrap();
    assert_eq!(container.owner_id, owner_id);
    assert_eq!(container.status, ContainerStatus::Running);

    // Stop the container
    container_deployment.stop_container(&container_id);
    assert_eq!(container.status, ContainerStatus::Stopped);

    // Attempt to stop a nonexistent container
    let result = std::panic::catch_unwind(|| {
        container_deployment.stop_container("nonexistent_container");
    });
    assert!(result.is_err());
}