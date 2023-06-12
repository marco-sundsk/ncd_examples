pub use near_sdk::{
    serde_json::json, 
    serde::{Deserialize, Serialize},
};
pub use near_units::parse_near;
pub use workspaces::{
    network::Sandbox, result::CallExecutionDetails, Account, AccountId, Contract, Worker,
};

pub async fn initialize_contracts_and_users(
    worker: &Worker<Sandbox>,
) -> anyhow::Result<(Account, Contract)> {
    let root = worker.root_account();

    let cchl = root
        .create_subaccount(worker, "cchl")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();

    let cchl = cchl
        .deploy(
            &worker,
            &std::fs::read(format!("../../res/cross_contract_high_level.wasm"))?,
        )
        .await?
        .unwrap();

    Ok((root, cchl))
}

#[tokio::test]
async fn test_cchl() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let (root, cchl) = initialize_contracts_and_users(&worker).await?;
    println!("Contract deployed at {}.", cchl.id());

    let outcome = root
        .call(&worker, cchl.id(), "demo2")
        .args_json(())?
        .max_gas()
        .transact()
        .await?;
    println!("called demo2.");
    let ret = outcome.json::<u32>().unwrap();
    println!("We got a return value of {}.", ret);

    // let outcome = root
    //     .call(&worker, cchl.id(), "demo")
    //     .args_json(json!({
    //         "n": 5_u32,
    //     }))?
    //     .gas(300_000_000_000_000)
    //     .transact()
    //     .await?;
    // println!("called demo.");
    // let ret = outcome.json::<u32>().unwrap();
    // println!("We got a return value of {}.", ret);

    Ok(())
}
