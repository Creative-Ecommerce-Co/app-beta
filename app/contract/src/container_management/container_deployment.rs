use near_sdk::{env, AccountId};

#[derive(Default)]
pub struct ContainerDeployment {
    pub containers: Vec<Container>, // List of deployed containers
}

#[derive(Clone)]
pub struct Container {
    pub id: String,
    pub owner_id: AccountId,
    pub status: ContainerStatus,
}

#[derive(Clone)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Error,
}

impl ContainerDeployment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn deploy_container(&mut self, owner_id: AccountId, container_id: String) {
        let container = Container {
            id: container_id,
            owner_id,
            status: ContainerStatus::Running,
        };
        self.containers.push(container);
        env::log(format!("Container {} deployed successfully.", container_id).as_bytes());
    }

    pub fn stop_container(&mut self, container_id: &str) {
        if let Some(container) = self.containers.iter_mut().find(|c| c.id == container_id) {
            container.status = ContainerStatus::Stopped;
            env::log(format!("Container {} stopped successfully.", container_id).as_bytes());
        } else {
            env::panic(b"Container not found");
        }
    }

    pub fn get_container_status(&self, container_id: &str) -> Option<&ContainerStatus> {
        self.containers.iter().find(|c| c.id == container_id).map(|c| &c.status)
    }
}