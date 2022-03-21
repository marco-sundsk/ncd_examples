use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,},
    FungibleToken
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize,},
    json_types::U128,
    env, near_bindgen, require, AccountId, PanicOnDefault, PromiseOrValue
};
use crate::events::Event;

mod events;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    name: String,
    symbol: String,
    icon: Option<String>,
    decimals: u8,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        Self {
            token: FungibleToken::new(b"t".to_vec()),
            name,
            symbol,
            icon: None,
            decimals,
        }
    }

    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        if self.token.storage_balance_of(account_id.clone()).is_none() {
            self.token.internal_register_account(&account_id);
        }

        self.token
            .internal_deposit(&account_id, amount.into());
        
        Event::TokenMint { 
            caller_id: &env::predecessor_account_id(), 
            receiver_id: &account_id, 
            mint_amount: &amount, 
            cur_supply: &self.token.ft_total_supply() 
        }
        .emit();
    }

    pub fn burn(&mut self, account_id: AccountId, amount: U128) {
        self.token
            .internal_withdraw(&account_id, amount.into());
        
        Event::TokenBurn { 
            caller_id: &env::predecessor_account_id(), 
            target_id: &account_id, 
            burn_amount: &amount, 
            cur_supply: &self.token.ft_total_supply() 
        }
        .emit();
    }

    pub fn set_token_name(&mut self, name: String, symbol: String) {
        require!(env::predecessor_account_id() == env::current_account_id(), "NOT ALLOWED");
        self.name = name;
        self.symbol = symbol;
    }

    pub fn set_icon(&mut self, icon: String) {
        require!(env::predecessor_account_id() == env::current_account_id(), "NOT ALLOWED");
        self.icon = Some(icon);
    }

    pub fn set_decimals(&mut self, dec: u8) {
        require!(env::predecessor_account_id() == env::current_account_id(), "NOT ALLOWED");
        self.decimals = dec;
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            icon: self.icon.clone(),
            reference: None,
            reference_hash: None,
            decimals: self.decimals,
        }
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{env, testing_env};

    use super::*;

    #[test]
    fn test_basics() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Contract::new(String::from("TBD"), String::from("TBD"), 24);
        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        // contract.storage_deposit(Some(accounts(0)), None);
        contract.mint(accounts(0), 1_000_000.into());
        assert_eq!(contract.ft_balance_of(accounts(0)), 1_000_000.into());

        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        contract.storage_deposit(Some(accounts(1)), None);
        testing_env!(context
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.ft_transfer(accounts(1), 1_000.into(), None);
        assert_eq!(contract.ft_balance_of(accounts(1)), 1_000.into());

        contract.burn(accounts(1), 500.into());
        assert_eq!(contract.ft_balance_of(accounts(1)), 500.into());
    }
}