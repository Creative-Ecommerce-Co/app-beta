// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, env, AccountId};
use crate::actions::{product_actions, storefront_actions};
use crate::storage::Storage;
use crate::models::{Product, Storefront};

mod actions;
mod models;
mod storage;
mod utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    storage: Storage,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            storage: Storage::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    // Storefront actions
    pub fn add_storefront(&mut self, user_id: AccountId, storefront: Storefront) {
        storefront_actions::add_storefront(&mut self.storage, user_id, storefront);
    }

    pub fn update_storefront(&mut self, user_id: AccountId, storefront: Storefront) {
        storefront_actions::update_storefront(&mut self.storage, user_id, storefront);
    }

    pub fn get_storefront(&self, user_id: AccountId) -> Option<Storefront> {
        storefront_actions::get_storefront(&self.storage, user_id)
    }

    pub fn remove_storefront(&mut self, user_id: AccountId) {
        storefront_actions::remove_storefront(&mut self.storage, user_id);
    }

    pub fn list_storefronts(&self) -> Vec<Storefront> {
        storefront_actions::list_storefronts(&self.storage)
    }

    // Product actions
    pub fn add_product(&mut self, user_id: AccountId, product_id: String, product: Product) {
        product_actions::add_product(&mut self.storage, user_id, product_id, product);
    }

    pub fn update_product(&mut self, user_id: AccountId, product_id: String, product: Product) {
        product_actions::update_product(&mut self.storage, user_id, product_id, product);
    }

    pub fn get_product(&self, user_id: AccountId, product_id: String) -> Option<Product> {
        product_actions::get_product(&self.storage, user_id, product_id)
    }

    pub fn remove_product(&mut self, user_id: AccountId, product_id: String) {
        product_actions::remove_product(&mut self.storage, user_id, product_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "bob.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_add_and_get_storefront() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
        let user_id = AccountId::new_unchecked("user.testnet".to_string());
        let storefront = Storefront::new(user_id.clone());

        contract.add_storefront(user_id.clone(), storefront.clone());
        let retrieved_storefront = contract.get_storefront(user_id.clone()).unwrap();
        assert_eq!(retrieved_storefront, storefront);
    }

    #[test]
    fn test_add_and_get_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
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

        contract.add_storefront(user_id.clone(), Storefront::new(user_id.clone()));
        contract.add_product(user_id.clone(), product_id.clone(), product.clone());
        let retrieved_product = contract.get_product(user_id.clone(), product_id.clone()).unwrap();
        assert_eq!(retrieved_product.name, product.name);
    }
}