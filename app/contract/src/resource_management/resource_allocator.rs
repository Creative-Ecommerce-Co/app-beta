use near_sdk::collections::LookupMap;
use near_sdk::AccountId;

#[derive(Default)]
pub struct ResourceAllocator {
    pub resource_limits: LookupMap<AccountId, u32>, // Maps user accounts to their resource limits
}

impl ResourceAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_resource_limit(&mut self, user_id: AccountId, limit: u32) {
        self.resource_limits.insert(&user_id, &limit);
    }

    pub fn get_resource_limit(&self, user_id: AccountId) -> Option<u32> {
        self.resource_limits.get(&user_id)
    }

    pub fn allocate_resources(&self, user_id: AccountId, required_resources: u32) -> bool {
        if let Some(limit) = self.get_resource_limit(user_id) {
            return required_resources <= limit;
        }
        false
    }
}