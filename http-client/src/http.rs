use futures::future;
use log::{error, info};
use reqwest;
use std::time::Instant;

const URL: &str = "https://query1.finance.yahoo.com/v7/finance/quote?lang=en-US&region=US&corsDomain=finance.yahoo.com&symbols=";

pub struct GetResult {
    pub bodies: Vec<String>,
    pub elapsed_time: u128,
}

pub async fn get_portfolio(symbols: Vec<&str>) -> Result<GetResult, reqwest::Error> {
    let client = reqwest::Client::new();
    let now = Instant::now();

    let bodies = future::join_all(symbols.into_iter().map(|symbl| {
        let client = &client;
        async move {
            let url = format!("{}{}", URL, symbl);
            let resp = client.get(&url).send().await?;
            resp.text().await
        }
    }))
    .await;

    let mut result = GetResult {
        bodies: Vec::new(),
        elapsed_time: now.elapsed().as_millis(),
    };

    for b in bodies {
        match b {
            Ok(b) => result.bodies.push(b),
            Err(e) => error!("error by get portfolio: {}", e),
        }
    }

    Ok(result)
}

// https://www.youtube.com/watch?v=xnIDyMJZ4ws

pub async fn _get(url: &str) -> Result<(), reqwest::Error> {
    let now = Instant::now();

    let res = reqwest::get(url).await?;
    let status_code = res.status();
    let body = res.text().await?;

    info!("Body: \n{}", body);
    info!("Status: {}", status_code);
    info!("elapsed time: {} ms", now.elapsed().as_millis());

    Ok(())
}
