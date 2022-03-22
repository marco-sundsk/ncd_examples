use near_sdk::{
    serde::{Deserialize, Serialize},
    near_bindgen, AccountId
};
use crate::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub version: String,
    pub owner_id: AccountId,
    pub operators: Vec<AccountId>,
}

#[near_bindgen]
impl Contract {
    pub fn metadata(&self) -> Metadata {
        Metadata {
            owner_id: self.owner_id.clone(),
            operators: self.operators.to_vec(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}