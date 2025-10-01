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

use crate::types::{OrderState, OrderStatus};
use anyhow::Result;
use temporal_sdk::{WfContext, WfExitValue, ActivityOptions};
use tokio::time::Duration;
use tracing::info;

pub async fn order_workflow(ctx: WfContext, mut state: OrderState) -> Result<WfExitValue<()>, anyhow::Error> {
    // Force to be default state - payment not taken yet
    state.status = OrderStatus::Default;

    // Take payment
    ctx.activity(ActivityOptions {
        activity_type: "take_payment".to_string(),
        input: serde_json::to_vec(&())?.into(),
        start_to_close_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    })
    .await
    .unwrap_ok_payload();

    // Set order status to pending
    state.status = OrderStatus::Pending;

    // Send notification
    ctx.activity(ActivityOptions {
        activity_type: "send_text_message".to_string(),
        input: serde_json::to_vec(&state.clone())?.into(),
        start_to_close_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    })
    .await
    .unwrap_ok_payload();

    // For now, just wait a bit and then complete
    // In a real implementation, this would wait for restaurant updates
    ctx.timer(temporal_sdk::TimerOptions {
        duration: Duration::from_secs(10),
        summary: None,
    })
    .await;

    // Simulate order completion
    state.status = OrderStatus::Completed;

    // Send final notification
    ctx.activity(ActivityOptions {
        activity_type: "send_text_message".to_string(),
        input: serde_json::to_vec(&state.clone())?.into(),
        start_to_close_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    })
    .await
    .unwrap_ok_payload();

    info!("Order workflow completed successfully");
    Ok(WfExitValue::Normal(()))
}
