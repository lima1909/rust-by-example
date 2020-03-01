use std::time::Instant;
use reqwest;

pub async fn get(url: &str) -> Result<(), reqwest::Error> {
    let now = Instant::now();

    let res = reqwest::get(url).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);
    println!("elapsed time: {} ms", now.elapsed().as_millis());

    Ok(())
}
