use serde::{Deserialize, Serialize};

/// Input for the IP location workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInput {
    pub name: String,
    pub seconds: u64,
}

/// Output from the IP location workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOutput {
    pub ip_addr: String,
    pub location: String,
}

/// Task queue name for the IP location workflow
pub const TASK_QUEUE_NAME: &str = "ip-address-rust";
