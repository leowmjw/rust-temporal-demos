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

use crate::types::OrderState;
use anyhow::Result;
use temporal_sdk::{ActContext, ActivityError};
use tokio::time::{sleep, Duration};
use tracing::info;

pub async fn refund_payment(_ctx: ActContext, _: ()) -> Result<(), ActivityError> {
    info!("Refund payment activity started");

    // Simulate payment processing time
    sleep(Duration::from_secs(5)).await;

    info!("Refund payment activity finished");
    Ok(())
}

pub async fn send_text_message(_ctx: ActContext, status: OrderState) -> Result<(), ActivityError> {
    info!("Send text message activity started for status: {}", status.status);

    // Simulate sending text message
    sleep(Duration::from_secs(1)).await;

    info!("Send text message activity finished");
    Ok(())
}

pub async fn take_payment(_ctx: ActContext, _: ()) -> Result<(), ActivityError> {
    info!("Take payment activity started");

    // Simulate payment processing time
    sleep(Duration::from_secs(5)).await;

    info!("Take payment activity finished");
    Ok(())
}
