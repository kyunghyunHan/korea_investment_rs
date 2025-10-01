use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CustType {
    B, //법인
    P, //개인
}

#[derive(Debug, Clone, Copy)]
pub enum MarketType {
    Domestic,
    Overseas,
}