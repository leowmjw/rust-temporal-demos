/*
 * Copyright 2025 Simon Emms <simon@simonemms.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{env, str::FromStr};
use temporal_client::{ClientOptionsBuilder, WorkflowOptions, WorkflowClientTrait};
use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
use temporal_sdk_core::Url;
use tracing::info;


/// This script upserts a schedule into Temporal that is designed to run indefinitely.
/// This might be created by a CI/CD action, a Kubernetes Job or any other method
/// of running a script to completion.
///
/// This means this schedule will remain in your Temporal instance for its life
/// and deletion is out of the scope of this demo. You MUST manually delete it if
/// you are using a long-running Temporal service (eg, Temporal Cloud).
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get Temporal server address from environment
    let temporal_address = env::var("TEMPORAL_ADDRESS").unwrap_or_else(|_| "http://localhost:7233".to_string());

    // Create client
    let client_options = ClientOptionsBuilder::default()
        .target_url(Url::from_str(&temporal_address)?)
        .build()?;
    let client = client_options.connect("default", None).await?;

    // For now, just start a single workflow execution
    // In a real implementation, you would use the schedule client to create recurring schedules
    let workflow_id = format!("scheduled-payment-{}", uuid::Uuid::new_v4());
    let workflow_options = WorkflowOptions {
        id_reuse_policy: WorkflowIdReusePolicy::AllowDuplicate,
        ..Default::default()
    };

    info!("Starting scheduled workflow: {}", workflow_id);
    
    let handle = client
        .start_workflow(
            vec![], // input payloads
            workflow_id.clone(),
            "find_due_payments_workflow".to_string(),
            "payments".to_string(),
            None, // workflow_id_reuse_policy
            workflow_options,
        )
        .await?;

    info!("Scheduled workflow started: {} with run ID: {}", workflow_id, handle.run_id);
    info!("Note: This is a single execution. For recurring schedules, use the schedule client API.");
    
    Ok(())
}
