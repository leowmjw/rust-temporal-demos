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

pub const ORDER_FOOD_TASK_QUEUE: &str = "order-food";

pub struct Queries;
impl Queries {
    pub const GET_STATUS: &str = "GET_STATUS";
}

pub struct Signals;
impl Signals {
    pub const CHECKOUT: &str = "CHECKOUT";
}

pub struct Updates;
impl Updates {
    pub const ADD_ITEM: &str = "ADD_ITEM";
    pub const REMOVE_ITEM: &str = "REMOVE_ITEM";
    pub const UPDATE_STATUS: &str = "UPDATE_STATUS";
}
