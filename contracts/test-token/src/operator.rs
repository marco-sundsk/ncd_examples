use crate::*;
use near_sdk::{
    near_bindgen,
};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn set_token_name(&mut self, name: String, symbol: String) {
        assert_one_yocto();
        require!(self.is_owner_or_operators(), "NOT ALLOWED");
        self.name = name;
        self.symbol = symbol;
    }

    #[payable]
    pub fn set_icon(&mut self, icon: String) {
        assert_one_yocto();
        require!(self.is_owner_or_operators(), "NOT ALLOWED");
        self.icon = Some(icon);
    }

    #[payable]
    pub fn set_decimals(&mut self, dec: u8) {
        assert_one_yocto();
        require!(self.is_owner_or_operators(), "NOT ALLOWED");
        self.decimals = dec;
    }
}