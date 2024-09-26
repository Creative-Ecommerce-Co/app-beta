use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{BorshDeserialize, BorshSerialize, AccountId};
use crate::models::{Product, Storefront, Media};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Storage {
    // Maps user accounts to their associated storefronts
    user_storefronts: LookupMap<AccountId, Storefront>,
    collections: LookupMap<String, Vec<String>>, // Maps collection names to product IDs
}

impl Storage {
    // Initialize the storage
    pub fn new() -> Self {
        Self {
            user_storefronts: LookupMap::new(b"u".to_vec()),
            collections: LookupMap::new(b"c".to_vec()),
        }
    }

    // Add a new storefront for a user
    pub fn add_storefront(&mut self, user_id: AccountId, storefront: Storefront) {
        self.user_storefronts.insert(&user_id, &storefront);
    }

    // Retrieve a user's storefront
    pub fn get_storefront(&self, user_id: AccountId) -> Option<Storefront> {
        self.user_storefronts.get(&user_id)
    }

    // Remove a user's storefront
    pub fn remove_storefront(&mut self, user_id: AccountId) {
        self.user_storefronts.remove(&user_id);
    }

    // Retrieve all storefronts
    pub fn get_all_storefronts(&self) -> Vec<Storefront> {
        self.user_storefronts.values().collect()
    }

    // Add a product to a user's storefront
    pub fn add_product(&mut self, user_id: AccountId, product_id: String, product: Product) {
        if let Some(mut storefront) = self.user_storefronts.get_mut(&user_id) {
            storefront.products.insert(&product_id, &product);
            self.user_storefronts.insert(&user_id, &storefront);
        }
    }

    // Retrieve a product from a user's storefront
    pub fn get_product(&self, user_id: AccountId, product_id: String) -> Option<Product> {
        if let Some(storefront) = self.user_storefronts.get(&user_id) {
            return storefront.products.get(&product_id);
        }
        None
    }

    // Remove a product from a user's storefront
    pub fn remove_product(&mut self, user_id: AccountId, product_id: String) {
        if let Some(mut storefront) = self.user_storefronts.get_mut(&user_id) {
            storefront.products.remove(&product_id);
            self.user_storefronts.insert(&user_id, &storefront);
        }
    }

    // Add a tag to a product
    pub fn add_tag_to_product(&mut self, user_id: AccountId, product_id: String, tag: String) {
        if let Some(mut product) = self.get_product(user_id.clone(), product_id.clone()) {
            product.tags.push(tag);
            self.add_product(user_id, product_id, product);
        }
    }

    // Remove a tag from a product
    pub fn remove_tag_from_product(&mut self, user_id: AccountId, product_id: String, tag: String) {
        if let Some(mut product) = self.get_product(user_id.clone(), product_id.clone()) {
            product.tags.retain(|t| t != &tag);
            self.add_product(user_id, product_id, product);
        }
    }

    // Create a new collection
    pub fn create_collection(&mut self, collection_name: String) {
        self.collections.insert(&collection_name, &Vec::new());
    }

    // Add a product to a collection
    pub fn add_product_to_collection(&mut self, collection_name: String, product_id: String) {
        if let Some(mut product_ids) = self.collections.get(&collection_name) {
            product_ids.push(product_id);
            self.collections.insert(&collection_name, &product_ids);
        }
    }

    // Retrieve products in a collection
    pub fn get_collection(&self, collection_name: String) -> Option<Vec<String>> {
        self.collections.get(&collection_name)
    }

    // Add media to a product
    pub fn add_media_to_product(&mut self, user_id: AccountId, product_id: String, media: Media) {
        if let Some(mut product) = self.get_product(user_id.clone(), product_id.clone()) {
            product.media.push(media);
            self.add_product(user_id, product_id, product);
        }
    }

    // Retrieve media for a product
    pub fn get_product_media(&self, user_id: AccountId, product_id: String) -> Option<Vec<Media>> {
        if let Some(product) = self.get_product(user_id, product_id) {
            return Some(product.media);
        }
        None
    }
}