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

use schedule_payments_rust::activities::{find_payments_for_day, send_payment};
use schedule_payments_rust::workflows::{find_due_payments_workflow, make_payment};
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
        .task_queue("payments")
        .versioning_strategy(WorkerVersioningStrategy::None { 
            build_id: "rust-sdk".to_owned() 
        })
        .build()?;

    // Initialize core worker
    let core_worker = init_worker(&runtime, worker_config, client)?;

    // Create Rust SDK worker
    let mut worker = Worker::new_from_core(std::sync::Arc::new(core_worker), "payments");

    // Register workflows
    worker.register_wf("find_due_payments_workflow", find_due_payments_workflow);
    worker.register_wf("make_payment", |ctx: temporal_sdk::WfContext| async move {
        // This is a placeholder - in a real implementation, the payment data would be passed
        // through the workflow input. For now, we'll create a dummy payment.
        let dummy_payment = schedule_payments_rust::data::PaymentData {
            amount_in_pence: 1000,
            schedule: schedule_payments_rust::data::Schedule::Daily,
            schedule_time: 1,
            sender_id: "sender".to_string(),
            recipient_id: "recipient".to_string(),
        };
        make_payment(ctx, dummy_payment).await
    });

    // Register activities
    worker.register_activity("find_payments_for_day", find_payments_for_day);
    worker.register_activity("send_payment", send_payment);

    info!("Starting worker for task queue: payments");

    // Run worker
    if let Err(e) = worker.run().await {
        error!("Worker failed: {}", e);
        return Err(e.into());
    }

    Ok(())
}
