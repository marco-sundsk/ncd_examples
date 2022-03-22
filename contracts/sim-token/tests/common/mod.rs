use near_sdk_sim::{
    call, deploy, ContractAccount, ExecutionResult, UserAccount,
};
use near_sdk::serde::{Deserialize, Serialize};
use sim_token::ContractContract as SimToken;
use mock_dex::ContractContract as MockDex;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    SIM_TOKEN_WASM_BYTES => "../../res/sim_token.wasm",
    MOCK_DEX_WASM_BYTES => "../../res/mock_dex.wasm",
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub version: String,
    pub owner_id: String,
    pub operators: Vec<String>,
}

#[allow(dead_code)]
pub fn show_promises(r: &ExecutionResult) {
    for promise in r.promise_results() {
        println!("{:?}", promise);
    }
}

#[allow(dead_code)]
pub fn get_logs(r: &ExecutionResult) -> Vec<String> {
    let mut logs: Vec<String> = vec![];
    r.promise_results().iter().map(
        |ex| ex.as_ref().unwrap().logs().iter().map(
            |x| logs.push(x.clone())
        ).for_each(drop)
    ).for_each(drop);
    logs
}

#[allow(dead_code)]
pub fn get_error_count(r: &ExecutionResult) -> u32 {
    r.promise_errors().len() as u32
}

#[allow(dead_code)]
pub fn get_error_status(r: &ExecutionResult) -> String {
    format!("{:?}", r.promise_errors()[0].as_ref().unwrap().status())
}

#[allow(dead_code)]
pub fn deploy_token(
    root: &UserAccount,
) -> ContractAccount<SimToken> {
    let t = deploy!(
        contract: SimToken,
        contract_id: "token_id",
        bytes: &SIM_TOKEN_WASM_BYTES,
        signer_account: root
    );
    call!(root, t.new(root.account_id(), "token_name".to_string(), "token_symbol".to_string(), 24)).assert_success();
    t
}

#[allow(dead_code)]
pub fn deploy_dex(
    root: &UserAccount,
) -> ContractAccount<MockDex> {
    let t = deploy!(
        contract: MockDex,
        contract_id: "dex_id",
        bytes: &MOCK_DEX_WASM_BYTES,
        signer_account: root
    );
    t
}