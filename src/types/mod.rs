use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CustType {
    B, //법인
    P, //개인
}

#[derive(Debug, Clone, Copy)]
pub enum MarketType {
    Domestic,
    Overseas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub cano: String,
    pub acnt_prdt_cd: String,
}

impl AccountInfo {
    pub fn new(cano: impl Into<String>, acnt_prdt_cd: impl Into<String>) -> Self {
        Self {
            cano: cano.into(),
            acnt_prdt_cd: acnt_prdt_cd.into(),
        }
    }

    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            cano: env::var("KIS_CANO")
                .or_else(|_| env::var("CANO"))
                .map_err(|_| "KIS_CANO not set in environment")?,
            acnt_prdt_cd: env::var("KIS_ACNT_PRDT_CD")
                .or_else(|_| env::var("ACNT_PRDT_CD"))
                .map_err(|_| "KIS_ACNT_PRDT_CD not set in environment")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContinuationKey {
    pub fk: String,
    pub nk: String,
}
