use near_sdk_sim::{call, init_simulator, to_yocto, view, DEFAULT_GAS};
use near_sdk::AccountId;
use crate::common::*;

mod common;

#[test]
pub fn test_owner(){
    let root = init_simulator(None);
    let token = deploy_token(&root);

    let owner = root.create_user(AccountId::new_unchecked("owner".to_string()), to_yocto("100"));
    let op1 = root.create_user(AccountId::new_unchecked("op1".to_string()), to_yocto("100"));
    let op2 = root.create_user(AccountId::new_unchecked("op2".to_string()), to_yocto("100"));

    let out_come = call!(
        root,
        token.set_owner(owner.account_id())
    );
    // show_promises(&out_come);
    // println!("{:?}", get_logs(&out_come));
    assert_eq!(get_error_count(&out_come), 1);
    assert!(get_error_status(&out_come).contains("Requires attached deposit of exactly 1 yoctoNEAR"));

    let out_come = call!(
        owner,
        token.set_owner(owner.account_id()),
        deposit = 1
    );
    assert_eq!(get_error_count(&out_come), 1);
    assert!(get_error_status(&out_come).contains("ERR_NOT_OWNER"));

    call!(
        root,
        token.set_owner(owner.account_id()),
        deposit = 1
    ).assert_success();
    let meta = view!(token.metadata()).unwrap_json::<Metadata>();
    assert_eq!(meta.owner_id, "owner".to_string());

    call!(
        owner,
        token.extend_operators(vec![op1.account_id(), op2.account_id()]),
        1,
        DEFAULT_GAS
    ).assert_success();

    let meta = view!(token.metadata()).unwrap_json::<Metadata>();
    assert_eq!(meta.operators, vec!["op1", "op2"]);

    call!(
        owner,
        token.remove_operators(vec![op1.account_id()]),
        deposit = 1
    ).assert_success();
    let meta = view!(token.metadata()).unwrap_json::<Metadata>();
    assert_eq!(meta.operators, vec!["op2"]);
}