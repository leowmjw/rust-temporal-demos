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

use crate::data::PaymentData;
use anyhow::Result;
use chrono::Utc;
use temporal_sdk::{WfContext, WfExitValue, ActivityOptions, ChildWorkflowOptions, TimerOptions};
use tokio::time::Duration;
use tracing::info;

/// Find payments due today
pub async fn find_due_payments_workflow(ctx: WfContext) -> Result<WfExitValue<()>, anyhow::Error> {
    let now = Utc::now();
    let start_time = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let end_time = start_time + chrono::Duration::days(1);

    info!("Find payments due: {} to {}", start_time, end_time);

    // Sleep for 5 seconds to simulate processing time
    ctx.timer(TimerOptions {
        duration: Duration::from_secs(5),
        summary: None,
    }).await;

    let payments = ctx
        .activity(ActivityOptions {
            activity_type: "find_payments_for_day".to_string(),
            input: serde_json::to_vec(&(start_time, end_time))?.into(),
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await
        .unwrap_ok_payload();
    
    let payments: Vec<PaymentData> = serde_json::from_slice(&payments.data)?;

    info!("Making {} payments", payments.len());

    // Sleep for another 5 seconds
    ctx.timer(TimerOptions {
        duration: Duration::from_secs(5),
        summary: None,
    }).await;

    // Execute child workflows in parallel
    let mut child_futures = Vec::new();
    
    for (i, payment) in payments.into_iter().enumerate() {
        let workflow_id = format!("payment_{}", i);
        let child_future = ctx
            .child_workflow(ChildWorkflowOptions {
                workflow_id: workflow_id,
                workflow_type: "make_payment".to_string(),
                input: vec![serde_json::to_vec(&payment)?.into()],
                ..Default::default()
            })
            .start(&ctx);
        child_futures.push(child_future);
    }

    // Wait for all child workflows to complete
    for future in child_futures {
        let pending = future.await;
        if let Some(started) = pending.into_started() {
            started.result().await;
        }
    }

    info!("All payments completed successfully");
    Ok(WfExitValue::Normal(()))
}

pub async fn make_payment(ctx: WfContext, payment: PaymentData) -> Result<WfExitValue<()>, anyhow::Error> {
    info!("Making payment for amount: {} pence", payment.amount_in_pence);

    let _result = ctx
        .activity(ActivityOptions {
            activity_type: "send_payment".to_string(),
            input: serde_json::to_vec(&payment)?.into(),
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await
        .unwrap_ok_payload();

    info!("Payment completed successfully");
    Ok(WfExitValue::Normal(()))
}
