use near_sdk::{
    AccountId, log,
    serde::{Serialize},
    serde_json::{json},
    json_types::U128,
};

const EVENT_STANDARD: &str = "test-token";
const EVENT_STANDARD_VERSION: &str = "1.0.0";

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum Event<'a> {
    TokenMint {
        caller_id: &'a AccountId,
        receiver_id: &'a AccountId,
        mint_amount: &'a U128,
        cur_supply: &'a U128,
    },
    TokenBurn {
        caller_id: &'a AccountId,
        target_id: &'a AccountId,
        burn_amount: &'a U128,
        cur_supply: &'a U128,
    },
}

impl Event<'_> {
    pub fn emit(&self) {
        emit_event(&self);
    }
}

// Emit event that follows NEP-297 standard: https://nomicon.io/Standards/EventsFormat
// Arguments
// * `standard`: name of standard, e.g. nep171
// * `version`: e.g. 1.0.0
// * `event`: type of the event, e.g. nft_mint
// * `data`: associate event data. Strictly typed for each set {standard, version, event} inside corresponding NEP
pub (crate) fn emit_event<T: ?Sized + Serialize>(data: &T) {
    let result = json!(data);
    let event_json = json!({
        "standard": EVENT_STANDARD,
        "version": EVENT_STANDARD_VERSION,
        "event": result["event"],
        "data": [result["data"]]
    })
    .to_string();
    log!(format!("EVENT_JSON:{}", event_json));
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils, AccountId};

    fn alice() -> AccountId {
        AccountId::new_unchecked("alice".to_string())
    }

    fn bob() -> AccountId {
        AccountId::new_unchecked("bob".to_string())
    }

    #[test]
    fn event_token_mint() {
        let caller_id = &alice();
        let receiver_id = &bob();
        let mint_amount = &U128(100);
        let cur_supply = &U128(10000);
        Event::TokenMint { caller_id, receiver_id, mint_amount, cur_supply }.emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"test-token","version":"1.0.0","event":"token_mint","data":[{"caller_id":"alice","receiver_id":"bob","mint_amount":"100","cur_supply":"10000"}]}"#
        );
    }

    #[test]
    fn event_token_burn() {
        let caller_id = &alice();
        let target_id = &bob();
        let burn_amount = &U128(100);
        let cur_supply = &U128(10000);
        Event::TokenBurn { caller_id, target_id, burn_amount, cur_supply }.emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"test-token","version":"1.0.0","event":"token_burn","data":[{"caller_id":"alice","target_id":"bob","burn_amount":"100","cur_supply":"10000"}]}"#
        );
    }
}