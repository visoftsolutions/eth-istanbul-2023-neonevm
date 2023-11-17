use reqwest;
use serde::Serialize;
use tracing::{error, info};

#[derive(Debug, Serialize)]
struct Request {
    wallet: String,
    amount: u64,
}

pub async fn send_request(wallet_address: String) {
    let client = reqwest::Client::new();
    let data = Request {
        wallet: wallet_address.clone(),
        amount: 100,
    };

    info!("Sending request to Neon Faucet for wallet: {:?}", data);

    match client
        .post("https://api.neonfaucet.org/request_neon")
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await
    {
        Ok(res) => {
            info!("Status: {}", res.status());

            if res.status() != 200 {
                let error_message = format!("HTTP Error: {}", res.status());
                error!("{}", error_message);
            } else {
                info!("Headers:\n{:#?}", res.headers());

                match res.text().await {
                    Ok(body) => {
                        info!("Body:\n{}", body);
                    }
                    Err(err) => {
                        error!("Failed to read response body: {:?}", err);
                    }
                }
            }
        }
        Err(err) => {
            error!("Request failed: {:?}", err);
        }
    }
}
