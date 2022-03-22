use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    log, near_bindgen, AccountId, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!(format!(
            "predecessor: {}, gas: {}T, deposit: {}yocto",
            env::predecessor_account_id(),
            u64::from(env::prepaid_gas()) as f64 / 1e12,
            env::attached_deposit()
        ));
        log!(format!(
            "sender: {}, amount: {}, msg: {}",
            sender_id, amount.0, msg
        ));
        PromiseOrValue::Value(U128(0))
    }
}
