use iplocate_rust::{WorkflowInput, TASK_QUEUE_NAME};
use std::{env, str::FromStr};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::Url;
use temporal_client::{WorkflowOptions, WorkflowClientTrait};
use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get Temporal server address from environment
    let temporal_address = env::var("TEMPORAL_ADDRESS").unwrap_or_else(|_| "http://localhost:7233".to_string());

    // Get client identity from environment or use default
    let client_identity = env::var("CLIENT_IDENTITY").unwrap_or_else(|_| "iplocate-rust-client".to_string());

    // Create client
    let server_options = sdk_client_options(Url::from_str(&temporal_address)?)
        .identity(client_identity)
        .build()?;
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
            vec![input.as_json_payload()?],           // input payloads
            TASK_QUEUE_NAME.to_string(),              // task_queue
            workflow_id.clone(),                       // workflow_id
            "get_address_from_ip".to_string(),        // workflow_type
            None,                                      // request_id
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
