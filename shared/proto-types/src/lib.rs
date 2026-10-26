use serde::{Deserialize, Serialize}
use uuid::Uuid;
use chrono::{Datetime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSide {
    Buy, Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub user_id : Uuid,
    pub symbol : String,
    pub side : OrderSide,
    pub price : f64,
    pub quantity : f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTicket {
    pub symbol: String,
    pub price: f64,
    pub timestamp: Datetime<Utc>,
}