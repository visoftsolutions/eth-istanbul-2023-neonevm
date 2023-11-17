use std::io::Error;

use reqwest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Request {
    wallet: String,
    amount: u64,
}

pub async fn send_request(wallet_address: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let data = Request {
        wallet: wallet_address,
        amount: 100,
    };

    println!("{:?}", data);

    let res = client
        .post("https://api.neonfaucet.org/request_neon")
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;

    println!("Status: {}", res.status());
    if res.status() != 200 {
        return Err(Box::new(Error::new(
            std::io::ErrorKind::Other,
            res.status().as_str(),
        )));
    }
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
