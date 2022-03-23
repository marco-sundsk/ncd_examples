use crate::common::*;
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use near_sdk_sim::{call, init_simulator, to_yocto, view, DEFAULT_GAS};

mod common;

#[test]
pub fn test_operator() {
    let root = init_simulator(None);
    let token = deploy_token(&root);

    let op = root.create_user(AccountId::new_unchecked("op".to_string()), to_yocto("100"));
    let user = root.create_user(AccountId::new_unchecked("user".to_string()), to_yocto("100"));

    call!(
        root,
        token.extend_operators(vec![op.account_id()]),
        1,
        DEFAULT_GAS
    ).assert_success();

    let out_come = call!(
        user,
        token.mint(user.account_id(), U128(to_yocto("5")))
    );
    out_come.assert_success();
    assert_eq!(view!(token.ft_balance_of(user.account_id())).unwrap_json::<U128>(), U128(to_yocto("5")));

    let out_come = call!(
        op,
        token.peek_user_balance(user.account_id()),
        1,
        DEFAULT_GAS
    );
    out_come.assert_success();   
    assert!(get_logs(&out_come)[0].contains(r#"account: user, balance: 5000000000000000000000000"#));
    println!("{:#?}", get_logs(&out_come));
    println!("tokens_burnt: {} Near", (out_come.tokens_burnt()) as f64 / 1e24);
    println!("Gas_burnt: {} TGas", u64::from(out_come.gas_burnt()) as f64 / 1e12);

}
