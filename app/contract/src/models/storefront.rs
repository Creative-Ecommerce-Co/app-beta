// app/contract/src/models/storefront.rs

use near_sdk::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use crate::models::{Product, Media, Variation, Review, Discount};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Storefront {
    pub owner_id: AccountId, // ID of the user who owns the storefront
    pub products: UnorderedMap<String, Product>, // Map of product IDs to products
    pub collections: UnorderedMap<String, Vec<String>>, // Maps collection names to product IDs
}

impl Storefront {
    /// Creates a new storefront for the owner.
    ///
    /// # Arguments
    ///
    /// * `owner_id` - The ID of the user who owns the storefront.
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            products: UnorderedMap::new(b"p".to_vec()),
            collections: UnorderedMap::new(b"c".to_vec()),
        }
    }

    /// Adds a product to the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to add.
    /// * `product` - The product to add.
    pub fn add_product(&mut self, product_id: String, product: Product) {
        self.products.insert(&product_id, &product);
    }

    /// Retrieves a product from the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to retrieve.
    pub fn get_product(&self, product_id: String) -> Option<Product> {
        self.products.get(&product_id)
    }

    /// Updates a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to update.
    /// * `product` - The updated product.
    pub fn update_product(&mut self, product_id: String, product: Product) {
        self.products.insert(&product_id, &product);
    }

    /// Removes a product from the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to remove.
    pub fn remove_product(&mut self, product_id: String) {
        self.products.remove(&product_id);
    }

    /// Updates the inventory count for a product.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to update.
    /// * `new_stock` - The new stock level for the product.
    pub fn update_product_inventory(&mut self, product_id: String, new_stock: u32) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.stock = new_stock; // Assuming Product has a stock field
            self.products.insert(&product_id, &product);
        }
    }

    /// Retrieves all products in the storefront.
    pub fn get_all_products(&self) -> Vec<Product> {
        self.products.values().collect()
    }

    /// Searches for products by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to search for.
    pub fn search_products_by_name(&self, name: String) -> Vec<Product> {
        self.products.values()
            .filter(|product| product.name.contains(&name))
            .cloned()
            .collect()
    }

    /// Filters products by status.
    ///
    /// # Arguments
    ///
    /// * `status` - The status to filter by.
    pub fn filter_products_by_status(&self, status: ProductStatus) -> Vec<Product> {
        self.products.values()
            .filter(|product| product.status == status)
            .cloned()
            .collect()
    }

    /// Activates a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to activate.
    pub fn activate_product(&mut self, product_id: String) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.status = ProductStatus::Active;
            self.products.insert(&product_id, &product);
        }
    }

    /// Deactivates a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to deactivate.
    pub fn deactivate_product(&mut self, product_id: String) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.status = ProductStatus::Inactive;
            self.products.insert(&product_id, &product);
        }
    }

    /// Updates a collection by replacing its product IDs.
    ///
    /// # Arguments
    ///
    /// * `collection_name` - The name of the collection to update.
    /// * `product_ids` - The new list of product IDs for the collection.
    pub fn update_collection(&mut self, collection_name: String, product_ids: Vec<String>) {
        self.collections.insert(&collection_name, &product_ids);
    }

    /// Removes a collection from the storefront.
    ///
    /// # Arguments
    ///
    /// * `collection_name` - The name of the collection to remove.
    pub fn remove_collection(&mut self, collection_name: String) {
        self.collections.remove(&collection_name);
    }

    /// Adds media to a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to which media will be added.
    /// * `media` - The media to add.
    pub fn add_media_to_product(&mut self, product_id: String, media: Media) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.media.push(media);
            self.products.insert(&product_id, &product);
        }
    }

    /// Retrieves media for a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product for which to retrieve media.
    pub fn get_product_media(&self, product_id: String) -> Option<Vec<Media>> {
        if let Some(product) = self.products.get(&product_id) {
            return Some(product.media);
        }
        None
    }

    /// Adds a variation to a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to which the variation will be added.
    /// * `variation` - The variation to add.
    pub fn add_variation_to_product(&mut self, product_id: String, variation: Variation) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.variations.push(variation);
            self.products.insert(&product_id, &product);
        }
    }

    /// Adds a review to a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to which the review will be added.
    /// * `review` - The review to add.
    pub fn add_review_to_product(&mut self, product_id: String, review: Review) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.reviews.push(review);
            self.products.insert(&product_id, &product);
        }
    }

    /// Adds a discount to a product in the storefront.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product to which the discount will be added.
    /// * `discount` - The discount to add.
    pub fn add_discount_to_product(&mut self, product_id: String, discount: Discount) {
        if let Some(mut product) = self.products.get_mut(&product_id) {
            product.discount = Some(discount);
            self.products.insert(&product_id, &product);
        }
    }
}