use near_sdk::env;

#[derive(Default)]
pub struct ResourceTracker {
    pub total_gas_used: u64,
}

impl ResourceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track_gas_usage(&mut self) {
        let gas_used = env::used_gas();
        self.total_gas_used += gas_used;
    }

    pub fn get_total_gas_used(&self) -> u64 {
        self.total_gas_used
    }
}