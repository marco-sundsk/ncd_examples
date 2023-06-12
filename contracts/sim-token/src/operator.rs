use crate::*;
use near_sdk::{
    near_bindgen, log, ext_contract, AccountId, Gas, Balance
};

pub const NO_DEPOSIT: Balance = 0;
pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_nep_141)]
pub trait ExtNep141 {
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

// #[ext_contract(ext_self)]
// trait DemoCallback {
//     fn get_balance_callback(&mut self, account_id: AccountId);
// }

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

    #[payable]
    pub fn peek_user_balance(&mut self, account_id: AccountId) {
        assert_one_yocto();
        require!(self.is_owner_or_operators(), "NOT ALLOWED");
        ext_nep_141::ext(env::current_account_id())
            .with_static_gas(env::prepaid_gas() - Gas(20 * TGAS) - Gas(5 * TGAS))
            .ft_balance_of(account_id.clone())
        .then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .get_balance_callback(account_id)
            );
    }


    #[private]
    pub fn get_balance_callback(
        &mut self,
        account_id: AccountId,
        #[callback] balance: U128,
    ) {
        log!(format!(
            "account: {}, balance: {}",
            account_id, balance.0
        ));
    }
}