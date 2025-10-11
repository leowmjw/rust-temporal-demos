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

use schedule_payments_rust::data::{generate_data, PaymentData, Schedule};
use tracing::info;

#[tokio::test]
async fn test_data_generation() {
    let _ = tracing_subscriber::fmt::try_init();
    
    let data = generate_data();
    assert!(!data.is_empty(), "Generated data should not be empty");
    
    // Check that we have different types of payments
    let has_daily = data.iter().any(|p| matches!(p.schedule, Schedule::Daily));
    let has_weekly = data.iter().any(|p| matches!(p.schedule, Schedule::Weekly));
    let has_monthly = data.iter().any(|p| matches!(p.schedule, Schedule::Monthly));
    
    assert!(has_daily, "Should have daily payments");
    assert!(has_weekly, "Should have weekly payments");
    assert!(has_monthly, "Should have monthly payments");
    
    info!("Generated {} payment records", data.len());
    for payment in &data {
        info!("Payment: {} pence, schedule: {:?}", payment.amount_in_pence, payment.schedule);
    }
}

#[tokio::test]
async fn test_payment_data_serialization() {
    let _ = tracing_subscriber::fmt::try_init();
    
    let payment = PaymentData {
        amount_in_pence: 1000,
        schedule: Schedule::Daily,
        schedule_time: 1,
        sender_id: "test_sender".to_string(),
        recipient_id: "test_recipient".to_string(),
    };
    
    // Test serialization
    let serialized = serde_json::to_string(&payment).unwrap();
    assert!(!serialized.is_empty());
    
    // Test deserialization
    let deserialized: PaymentData = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.amount_in_pence, payment.amount_in_pence);
    assert_eq!(deserialized.schedule, payment.schedule);
    assert_eq!(deserialized.sender_id, payment.sender_id);
    assert_eq!(deserialized.recipient_id, payment.recipient_id);
    
    info!("Payment data serialization test passed");
}

#[tokio::test]
async fn test_schedule_enum() {
    let _ = tracing_subscriber::fmt::try_init();
    
    // Test all schedule variants
    let daily = Schedule::Daily;
    let weekly = Schedule::Weekly;
    let monthly = Schedule::Monthly;
    
    // Test serialization
    assert_eq!(serde_json::to_string(&daily).unwrap(), "\"Daily\"");
    assert_eq!(serde_json::to_string(&weekly).unwrap(), "\"Weekly\"");
    assert_eq!(serde_json::to_string(&monthly).unwrap(), "\"Monthly\"");
    
    // Test deserialization
    assert_eq!(serde_json::from_str::<Schedule>("\"Daily\"").unwrap(), Schedule::Daily);
    assert_eq!(serde_json::from_str::<Schedule>("\"Weekly\"").unwrap(), Schedule::Weekly);
    assert_eq!(serde_json::from_str::<Schedule>("\"Monthly\"").unwrap(), Schedule::Monthly);
    
    info!("Schedule enum test passed");
}
