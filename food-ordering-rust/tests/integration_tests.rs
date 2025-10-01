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

use food_ordering_rust::types::{Address, OrderProduct, OrderState, OrderStatus};
use tracing::info;

#[tokio::test]
async fn test_order_state_operations() {
    let _ = tracing_subscriber::fmt::try_init();
    
    let mut order_state = OrderState::new();
    order_state.email = "test@example.com".to_string();
    order_state.collection = false;
    
    // Test adding items
    order_state.add_item(OrderProduct {
        product_id: 1,
        quantity: 2,
    });
    
    order_state.add_item(OrderProduct {
        product_id: 2,
        quantity: 1,
    });
    
    // Test adding same item (should increase quantity)
    order_state.add_item(OrderProduct {
        product_id: 1,
        quantity: 1,
    });
    
    assert_eq!(order_state.products.len(), 2);
    assert_eq!(order_state.products[0].quantity, 3); // 2 + 1
    assert_eq!(order_state.products[1].quantity, 1);
    
    // Test removing items
    order_state.remove_item(OrderProduct {
        product_id: 1,
        quantity: 1,
    });
    
    assert_eq!(order_state.products[0].quantity, 2); // 3 - 1
    
    // Test removing all of an item
    order_state.remove_item(OrderProduct {
        product_id: 1,
        quantity: 2,
    });
    
    assert_eq!(order_state.products.len(), 1); // Should only have product_id 2 left
    
    info!("Order state operations test completed successfully");
}

#[tokio::test]
async fn test_order_status_parsing() {
    let _ = tracing_subscriber::fmt::try_init();
    
    // Test parsing from string
    assert_eq!("DEFAULT".parse::<OrderStatus>().unwrap(), OrderStatus::Default);
    assert_eq!("PENDING".parse::<OrderStatus>().unwrap(), OrderStatus::Pending);
    assert_eq!("ACCEPTED".parse::<OrderStatus>().unwrap(), OrderStatus::Accepted);
    assert_eq!("PREPARING".parse::<OrderStatus>().unwrap(), OrderStatus::Preparing);
    assert_eq!("READY".parse::<OrderStatus>().unwrap(), OrderStatus::Ready);
    assert_eq!("COMPLETED".parse::<OrderStatus>().unwrap(), OrderStatus::Completed);
    assert_eq!("REJECTED".parse::<OrderStatus>().unwrap(), OrderStatus::Rejected);
    
    // Test case insensitive parsing
    assert_eq!("default".parse::<OrderStatus>().unwrap(), OrderStatus::Default);
    assert_eq!("pending".parse::<OrderStatus>().unwrap(), OrderStatus::Pending);
    
    // Test invalid status
    assert!("INVALID".parse::<OrderStatus>().is_err());
    
    // Test display formatting
    assert_eq!(format!("{}", OrderStatus::Default), "DEFAULT");
    assert_eq!(format!("{}", OrderStatus::Pending), "PENDING");
    
    info!("Order status parsing test completed successfully");
}

#[tokio::test]
async fn test_order_state_serialization() {
    let _ = tracing_subscriber::fmt::try_init();
    
    let order_state = OrderState {
        collection: false,
        delivery_address: Some(Address {
            line1: "123 Test St".to_string(),
            line2: None,
            line3: None,
            town: "Test City".to_string(),
            county: None,
            post_code: "12345".to_string(),
        }),
        email: "test@example.com".to_string(),
        products: vec![OrderProduct {
            product_id: 1,
            quantity: 2,
        }],
        status: OrderStatus::Pending,
    };
    
    // Test serialization
    let serialized = serde_json::to_string(&order_state).unwrap();
    assert!(!serialized.is_empty());
    
    // Test deserialization
    let deserialized: OrderState = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.email, order_state.email);
    assert_eq!(deserialized.collection, order_state.collection);
    assert_eq!(deserialized.status, order_state.status);
    assert_eq!(deserialized.products.len(), order_state.products.len());
    
    info!("Order state serialization test passed");
}
