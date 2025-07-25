use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Room {
    pub room_pk: i32,
    pub name: Option<String>,
    pub capacity: Option<i32>,
    pub max_capacity: Option<i32>,
    pub is_apartment: Option<bool>,
    pub has_kitchen: Option<bool>,
    pub bedrooms: Option<i32>
}