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

use chrono::{Utc, Datelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Schedule {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentData {
    /// Ignored if daily, weekly is day of week (1=monday), monthly is day of month
    pub schedule_time: u32,
    pub schedule: Schedule,
    pub amount_in_pence: u32,
    pub sender_id: String,
    pub recipient_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPaymentResult {
    pub amount_in_pence: u32,
    pub transaction_id: Uuid,
}

/// This generates the data. This would ordinarily be a database
/// connection
pub fn generate_data() -> Vec<PaymentData> {
    let today = Utc::now();
    let tomorrow = today + chrono::Duration::days(1);
    let yesterday = today - chrono::Duration::days(1);

    let data = vec![
        PaymentData {
            // Daily - due today
            schedule: Schedule::Daily,
            schedule_time: 0, // Not used for daily
            amount_in_pence: 10000,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
        PaymentData {
            // Weekly - due yesterday
            schedule: Schedule::Weekly,
            schedule_time: yesterday.weekday().num_days_from_monday() + 1,
            amount_in_pence: 10100,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
        PaymentData {
            // Weekly - due today
            schedule: Schedule::Weekly,
            schedule_time: today.weekday().num_days_from_monday() + 1,
            amount_in_pence: 10200,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
        PaymentData {
            // Weekly - due tomorrow
            schedule: Schedule::Weekly,
            schedule_time: tomorrow.weekday().num_days_from_monday() + 1,
            amount_in_pence: 10300,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
        PaymentData {
            // Monthly - due yesterday
            schedule: Schedule::Monthly,
            schedule_time: yesterday.day(),
            amount_in_pence: 10400,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
        PaymentData {
            // Monthly - due today
            schedule: Schedule::Monthly,
            schedule_time: today.day(),
            amount_in_pence: 10000,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
        PaymentData {
            // Monthly - due tomorrow
            schedule: Schedule::Monthly,
            schedule_time: tomorrow.day(),
            amount_in_pence: 10000,
            sender_id: Uuid::new_v4().to_string(),
            recipient_id: Uuid::new_v4().to_string(),
        },
    ];

    data
}
