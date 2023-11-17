use ethers::core::types::Address;
use ethers::prelude::*;
use futures::future::join_all;
use rand::Rng;
use std::env;
use tokio::time::{sleep, Duration};

mod mint;
mod value_incrementer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let neonevm_url = env::var("NEONEVM_URL")?;
    let contract_address = env::var("CONTRACT_ADDRESS")?;
    let contract_address = Address::from_slice(&hex::decode(contract_address)?);

    let priv_keys = tokio::fs::read("privkeys.json").await?;
    let accounts: Vec<LocalWallet> = serde_json::from_slice::<Vec<String>>(&priv_keys)?
        .iter()
        .map(|key| key.parse::<LocalWallet>())
        .collect::<Result<Vec<_>, _>>()?;

    let provider = Provider::<Http>::try_from(neonevm_url)?;

    println!("{:?}", accounts[0].address());

    let tasks = accounts.into_iter().enumerate().map(|(id, a)| {
        handle(
            provider.clone(),
            id,
            a.with_chain_id(245022926_u64),
            contract_address.clone(),
        )
    });
    join_all(tasks).await;

    Ok(())
}

async fn handle(
    provider: Provider<Http>,
    id: usize,
    account: LocalWallet,
    contract_address: Address,
) {
    let client = SignerMiddleware::new(provider.clone(), account.clone());
    let contract = value_incrementer::ValueIncrementer::new(contract_address, client.into());
    let mut rng = rand::thread_rng();
    let mut i = 0;
    loop {
        let prepared_tx = contract.increment_value().legacy();
        let pending_tx = prepared_tx.send().await;
        match pending_tx {
            Ok(tx) => {
                match tx.await {
                    Ok(_) => {
                        println!("OK {} {} {}", id, account.address(), i);
                    }
                    Err(err) => {
                        println!("ERROR: {} {:?}", id, err);
                        let mint_result =
                            mint::send_request(hex::encode(account.address().as_bytes())).await;
                        println!("MINT RESULT: {} {:?}", id, mint_result);
                        if let Err(_) = mint_result {
                            sleep(Duration::from_secs(rng.gen_range(60..=600))).await;
                        }
                    }
                };
            }
            Err(err) => {
                println!("ERROR: {} {:?}", id, err);
                let mint_result =
                    mint::send_request(hex::encode(account.address().as_bytes())).await;
                println!("MINT RESULT: {} {:?}", id, mint_result);
                if let Err(_) = mint_result {
                    sleep(Duration::from_secs(rng.gen_range(60..=600))).await;
                }
            }
        };
        i += 1;
    }
}
