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

use crate::data::{PaymentData, Schedule, SendPaymentResult, generate_data};
use anyhow::Result;
use chrono::{DateTime, Utc, Datelike};
use temporal_sdk::{ActContext, ActivityError};
use tokio::time::{sleep, Duration};
use tracing::info;
use uuid::Uuid;

/// Simulate a database call that returns all the payments due today
pub async fn find_payments_for_day(
    _ctx: ActContext,
    (start_time, end_time): (DateTime<Utc>, DateTime<Utc>),
) -> Result<Vec<PaymentData>, ActivityError> {
    info!("Finding payments for day: {} to {}", start_time, end_time);

    let data = generate_data();
    let now = Utc::now();
    let mut payments = Vec::new();

    for item in &data {
        let include = match item.schedule {
            Schedule::Weekly => {
                // Check if it's today's weekday (1=monday)
                (now.weekday().num_days_from_monday() + 1) == item.schedule_time
            }
            Schedule::Monthly => {
                // Check if it's today's day of month
                now.day() == item.schedule_time
            }
            Schedule::Daily => {
                // Daily - add to list
                true
            }
        };

        if include {
            payments.push(item.clone());
        }
    }

    info!("Found {} payments due today", payments.len());
    Ok(payments)
}

pub async fn send_payment(
    _ctx: ActContext,
    payment: PaymentData,
) -> Result<SendPaymentResult, ActivityError> {
    info!("Sending payment for amount: {} pence", payment.amount_in_pence);

    // Simulate payment processing time
    sleep(Duration::from_secs(2)).await;

    let result = SendPaymentResult {
        amount_in_pence: payment.amount_in_pence,
        transaction_id: Uuid::new_v4(),
    };

    info!("Payment sent successfully with transaction ID: {}", result.transaction_id);
    Ok(result)
}
