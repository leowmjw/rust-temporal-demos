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
use temporal_sdk::{sdk_client_options};
use temporal_sdk_core::{Url};
use temporal_client::{WorkflowOptions, WorkflowClientTrait};
use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
use tracing::{error, info};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get Temporal server address from environment
    let temporal_address = env::var("TEMPORAL_ADDRESS").unwrap_or_else(|_| "http://localhost:7233".to_string());

    // Create client
    let server_options = sdk_client_options(Url::from_str(&temporal_address)?).build()?;
    let client = server_options.connect("default", None).await?;

    // Start workflow
    let workflow_id = format!("find-due-payments-{}", Uuid::new_v4());
    let workflow_options = WorkflowOptions {
        id_reuse_policy: WorkflowIdReusePolicy::AllowDuplicate,
        ..Default::default()
    };

    info!("Starting workflow with ID: {}", workflow_id);

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

    info!("Started workflow: {} with run ID: {}", workflow_id, handle.run_id);

    // Wait for workflow completion
    let result = client.get_workflow_execution_history(workflow_id, Some(handle.run_id.clone()), vec![]).await;
    match result {
        Ok(_) => {
            info!("Workflow completed successfully");
        }
        Err(e) => {
            error!("Workflow failed: {}", e);
            return Err(e.into());
        }
    }

    info!("Triggered");
    Ok(())
}
