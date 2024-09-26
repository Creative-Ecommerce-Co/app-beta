use near_sdk::{env, AccountId};

#[derive(Default)]
pub struct ContainerScaling {
    pub scaling_factors: Vec<ScalingFactor>, // List of scaling factors for containers
}

#[derive(Clone)]
pub struct ScalingFactor {
    pub container_id: String,
    pub scale_up: u32,
    pub scale_down: u32,
}

impl ContainerScaling {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_scaling_factor(&mut self, container_id: String, scale_up: u32, scale_down: u32) {
        let scaling_factor = ScalingFactor {
            container_id,
            scale_up,
            scale_down,
        };
        self.scaling_factors.push(scaling_factor);
        env::log(format!("Scaling factors set for container {}: scale up {}, scale down {}.", container_id, scale_up, scale_down).as_bytes());
    }

    pub fn scale_container(&self, container_id: &str, current_load: u32) {
        if let Some(scaling_factor) = self.scaling_factors.iter().find(|sf| sf.container_id == container_id) {
            if current_load > scaling_factor.scale_up {
                env::log(format!("Scaling up container {}.", container_id).as_bytes());
                // Logic to scale up the container
            } else if current_load < scaling_factor.scale_down {
                env::log(format!("Scaling down container {}.", container_id).as_bytes());
                // Logic to scale down the container
            }
        } else {
            env::panic(b"Scaling factor not found for container");
        }
    }
}