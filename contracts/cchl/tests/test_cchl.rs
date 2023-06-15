use serde_json::json;
pub use near_units::parse_near;
pub use workspaces::{
    network::Sandbox, result::ExecutionFinalResult, Account, AccountId, Contract, Worker,
};

pub async fn initialize_contracts_and_users(
    worker: &Worker<Sandbox>,
) -> anyhow::Result<(Account, Contract)> {
    let root = worker.root_account()?;

    let cchl = root
        .create_subaccount("cchl")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();

    let cchl = cchl
        .deploy(&std::fs::read(format!("../../res/cross_contract_high_level.wasm"))?)
        .await?
        .unwrap();

    // cchl.call("new")
    // .args_json(json!({}))?
    // .gas(300_000_000_000_000)
    // .transact()
    // .await?;

    Ok((root, cchl))
}

#[tokio::test]
async fn test_cchl() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let (root, cchl) = initialize_contracts_and_users(&worker).await?;
    println!("Contract deployed at {}.", cchl.id());

    let outcome = root
        .call(cchl.id(), "demo")
        .args_json(json!({
            "n": 5_u32,
        }))
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    println!("called demo.");
    let ret = outcome.json::<u32>().unwrap();
    println!("We got a return value of {}.", ret);

    let outcome = root
        .call(cchl.id(), "factorial")
        .args_json(json!({
            "n": 5_u32,
        }))
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    println!("called factorial.");

    // let ret = outcome.json::<u32>().unwrap();
    let ret: u32 = outcome.clone().json()?;
    println!("We got a return value of {}.", ret);

    // show details of the call
    println!("{:#?}", outcome);

    // show all logs in the call
    // println!("logs = {:#?}", outcome.logs());

    // total gas burnt
    println!("total_gas_burnt: {:?}", outcome.total_gas_burnt);

    // break down gas burnt
    // println!(
    //     "initial transaction gas_burnt: {:?}",
    //     outcome.outcome().gas_burnt
    // );
    // println!("Following is the break down of all sub receipts in the call:");
    // for receipt in outcome.receipt_outcomes() {
    //     println!("sub receipt gas burnt: {:?}", receipt.gas_burnt);
    // }

    println!("Following is the break down of all receipts in the call:");
    for receipt in outcome.outcomes() {
        println!("receipt gas burnt: {:?}", receipt.gas_burnt);
    }

    Ok(())
}
