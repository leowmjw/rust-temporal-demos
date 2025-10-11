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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Default,   // Order not paid yet
    Pending,   // Order paid and waiting for restaurant to accept
    Accepted,  // Restaurant accepted order, but not started work yet
    Preparing, // Restaurant is cooking your food
    Ready,     // Food is ready for collection/out for delivery
    Completed, // Food given to a hungry person
    Rejected,  // Kitchen has rejected the order
}

impl std::str::FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEFAULT" => Ok(OrderStatus::Default),
            "PENDING" => Ok(OrderStatus::Pending),
            "ACCEPTED" => Ok(OrderStatus::Accepted),
            "PREPARING" => Ok(OrderStatus::Preparing),
            "READY" => Ok(OrderStatus::Ready),
            "REJECTED" => Ok(OrderStatus::Rejected),
            "COMPLETED" => Ok(OrderStatus::Completed),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Default => write!(f, "DEFAULT"),
            OrderStatus::Pending => write!(f, "PENDING"),
            OrderStatus::Accepted => write!(f, "ACCEPTED"),
            OrderStatus::Preparing => write!(f, "PREPARING"),
            OrderStatus::Ready => write!(f, "READY"),
            OrderStatus::Rejected => write!(f, "REJECTED"),
            OrderStatus::Completed => write!(f, "COMPLETED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub town: String,
    pub county: Option<String>,
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderState {
    pub collection: bool,
    pub delivery_address: Option<Address>,
    pub email: String,
    pub products: Vec<OrderProduct>,
    pub status: OrderStatus,
}

impl OrderState {
    pub fn new() -> Self {
        Self {
            collection: false,
            delivery_address: None,
            email: String::new(),
            products: Vec::new(),
            status: OrderStatus::Default,
        }
    }

    pub fn add_item(&mut self, item: OrderProduct) {
        // Check if we're updating existing products
        for existing_item in &mut self.products {
            if existing_item.product_id == item.product_id {
                existing_item.quantity += item.quantity;
                return;
            }
        }

        // Otherwise, add new product
        self.products.push(item);
    }

    pub fn remove_item(&mut self, item: OrderProduct) {
        self.products.retain_mut(|existing_item| {
            if existing_item.product_id == item.product_id {
                existing_item.quantity = existing_item.quantity.saturating_sub(item.quantity);
                existing_item.quantity > 0
            } else {
                true
            }
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderProduct {
    pub product_id: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub product_id: u32,
    pub name: String,
    pub price: f64,
}

// Sample products data
pub fn get_sample_products() -> Vec<Product> {
    vec![
        Product {
            product_id: 1,
            name: "Margherita Pizza".to_string(),
            price: 12.99,
        },
        Product {
            product_id: 2,
            name: "Pepperoni Pizza".to_string(),
            price: 14.99,
        },
        Product {
            product_id: 3,
            name: "Caesar Salad".to_string(),
            price: 8.99,
        },
        Product {
            product_id: 4,
            name: "Chicken Wings".to_string(),
            price: 9.99,
        },
        Product {
            product_id: 5,
            name: "Coca Cola".to_string(),
            price: 2.99,
        },
    ]
}
