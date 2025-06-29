//! Send signed webhook payloads to a local server with retry support
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Sends the signed payload to the given URL with retry logic
pub async fn send_with_retry(
    url: &str,
    headers: HashMap<String, String>,
    body: String,
) -> Result<()> {
    let client = Client::new();
    let mut attempt = 0;

    loop {
        let mut req = client.post(url);
        for (k, v) in &headers {
            req = req.header(k, v);
        }
        let res = req.body(body.clone()).send().await;

        match res {
            Ok(r) => {
                println!("{}", r.status());
                let text = r.text().await.unwrap_or_default();
                println!("Response: {}", text);
                break;
            }
            Err(e) if attempt < 3 => {
                eprintln!("Retrying after error: {}", e);
                attempt += 1;
                sleep(Duration::from_secs(2u64.pow(attempt))).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
