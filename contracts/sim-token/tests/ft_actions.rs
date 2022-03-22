use near_sdk_sim::{call, init_simulator, to_yocto, view, DEFAULT_GAS};
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::common::*;

mod common;

#[test]
pub fn test_transfer_call(){
    let root = init_simulator(None);
    let token = deploy_token(&root);
    let dex = deploy_dex(&root);

    let user1 = root.create_user(AccountId::new_unchecked("user1".to_string()), to_yocto("100"));

    let out_come = call!(
        user1,
        token.mint(user1.account_id(), U128(to_yocto("3")))
    );
    out_come.assert_success();
    // show_promises(&out_come);
    // println!("{:#?}", get_logs(&out_come));
    // println!("tokens_burnt: {} Near", (out_come.tokens_burnt()) as f64 / 1e24);
    // println!("Gas_burnt: {} TGas", u64::from(out_come.gas_burnt()) as f64 / 1e12);
    assert!(get_logs(&out_come)[0].contains(r#""event":"token_mint""#));
    assert!(get_logs(&out_come)[0].contains(r#""caller_id":"user1","receiver_id":"user1","mint_amount":"3000000000000000000000000","cur_supply":"3000000000000000000000000""#));
    assert_eq!(view!(token.ft_balance_of(user1.account_id())).unwrap_json::<U128>(), U128(to_yocto("3")));

    call!(
        user1,
        token.storage_deposit(Some(dex.account_id()), Some(true)),
        deposit = to_yocto("1")
    )
    .assert_success();

    let out_come = call!(
        user1,
        token.ft_transfer_call(dex.account_id(), U128(to_yocto("2")), Some("This is memo".to_string()), "This is msg".to_string()),
        1,
        DEFAULT_GAS
    );
    out_come.assert_success();
    println!("{:#?}", get_logs(&out_come));
    println!("tokens_burnt: {} Near", (out_come.tokens_burnt()) as f64 / 1e24);
    println!("Gas_burnt: {} TGas", u64::from(out_come.gas_burnt()) as f64 / 1e12);
    assert_eq!(view!(token.ft_balance_of(user1.account_id())).unwrap_json::<U128>(), U128(to_yocto("1")));
}

