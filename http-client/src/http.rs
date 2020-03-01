use std::time::Instant;
use reqwest;
use log::info;

// https://www.youtube.com/watch?v=xnIDyMJZ4ws

pub async fn get(url: &str) -> Result<(), reqwest::Error> {
    let now = Instant::now();

    let res = reqwest::get(url).await?;
    let status_code = res.status();
    let body = res.text().await?;

    info!("Body: \n{}", body);
    info!("Status: {}", status_code);
    info!("elapsed time: {} ms", now.elapsed().as_millis());

    Ok(())
}