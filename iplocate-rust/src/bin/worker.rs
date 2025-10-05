use iplocate_rust::activities::{get_ip, get_location_info};
use iplocate_rust::workflows::get_address_from_ip;
use iplocate_rust::TASK_QUEUE_NAME;
use std::{env, str::FromStr};
use temporal_sdk::{sdk_client_options, Worker};
use temporal_sdk_core::{init_worker, Url, CoreRuntime};
use temporal_sdk_core_api::{
    worker::{WorkerConfigBuilder, WorkerVersioningStrategy},
    telemetry::TelemetryOptionsBuilder
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get Temporal server address from environment
    let temporal_address = env::var("TEMPORAL_ADDRESS").unwrap_or_else(|_| "http://localhost:7233".to_string());

    // Create client options
    let server_options = sdk_client_options(Url::from_str(&temporal_address)?).build()?;
    let client = server_options.connect("default", None).await?;

    // Create telemetry options and runtime
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    // Create worker config
    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(TASK_QUEUE_NAME)
        .versioning_strategy(WorkerVersioningStrategy::None {
            build_id: "rust-sdk".to_owned()
        })
        .build()?;

    // Initialize core worker
    let core_worker = init_worker(&runtime, worker_config, client)?;

    // Create Rust SDK worker
    let mut worker = Worker::new_from_core(std::sync::Arc::new(core_worker), TASK_QUEUE_NAME);

    // Register workflow with a closure that deserializes input
    worker.register_wf("get_address_from_ip", |ctx: temporal_sdk::WfContext| async move {
        // Extract input from context
        let input: iplocate_rust::WorkflowInput = serde_json::from_slice(
            &ctx.get_args().first().expect("Missing input").data
        ).expect("Failed to deserialize input");

        get_address_from_ip(ctx, input).await
    });

    // Register activities
    worker.register_activity("get_ip", |ctx, _: ()| get_ip(ctx));
    worker.register_activity("get_location_info", |ctx, ip: String| get_location_info(ctx, ip));

    info!("Starting worker for task queue: {}", TASK_QUEUE_NAME);

    // Run worker
    if let Err(e) = worker.run().await {
        error!("Worker failed: {}", e);
        return Err(e.into());
    }

    Ok(())
}
