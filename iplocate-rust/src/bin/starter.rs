use iplocate_rust::{WorkflowInput, TASK_QUEUE_NAME};
use std::{env, str::FromStr};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::Url;
use temporal_client::{WorkflowOptions, WorkflowClientTrait};
use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
use tracing::info;
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

    // Create workflow input
    let input = WorkflowInput {
        name: "User".to_string(),
        seconds: 0,
    };

    // Start workflow
    let workflow_id = format!("get-address-from-ip-{}", Uuid::new_v4());
    let workflow_options = WorkflowOptions {
        id_reuse_policy: WorkflowIdReusePolicy::AllowDuplicate,
        ..Default::default()
    };

    info!("Starting IP location workflow with ID: {}", workflow_id);

    let handle = client
        .start_workflow(
            vec![serde_json::to_vec(&input)?.into()], // input payloads
            workflow_id.clone(),
            "get_address_from_ip".to_string(),
            TASK_QUEUE_NAME.to_string(),
            None, // workflow_id_reuse_policy
            workflow_options,
        )
        .await?;

    info!("Started workflow: {} with run ID: {}", workflow_id, handle.run_id);

    // Wait for workflow to complete by polling history
    info!("Waiting for workflow to complete...");

    // Note: The actual result extraction would require polling the workflow history
    // For now, we'll just indicate the workflow was started successfully
    println!("\nWorkflow started successfully!");
    println!("Workflow ID: {}", workflow_id);
    println!("Run ID: {}", handle.run_id);
    println!("\nTo view the workflow result, check the Temporal Web UI at http://localhost:8233");

    Ok(())
}
