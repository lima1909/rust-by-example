mod http;
mod logging;

use log::info;
use serde_json::Value;

#[tokio::main]
async fn main() {
    logging::init();

    let r = http::get_portfolio(vec![
        "ABEC.DE", "NFC.DE", "SIE.DE", "AMZ.DE", "O1BC.F", "DWWD.SG", "2B76.F", "EXS2.F", "OE7A.SG",
    ])
    .await
    .unwrap();

    info!("Result: {}", r.bodies.len());
    info!("Elapsed time: {}", r.elapsed_time);

    for json in r.bodies {
        let jr: Value = serde_json::from_str(&json).unwrap();
        if let Some(r) = jr["quoteResponse"]["result"].as_array() {
            if r.len() > 0 {
                info!(
                    "{:?}: {:?}",
                    r[0]["symbol"].as_str().unwrap(),
                    // r[0]["longName"].as_str(),
                    r[0]["regularMarketPrice"],
                );
            }
        }
    }
}

/*
// QuoteResponse ...
type QuoteResponse struct {
    QR Output `json:"quoteResponse"`
}

// Output ...
type Output struct {
    Res   []Result    `json:"result"`
    Error interface{} `json:"error"`
}

type Result struct {
    Cur    string  `json:"currency"`
    Name   string  `json:"longName"`
    Close  float32 `json:"regularMarketPreviousClose"`
    Price  float32 `json:"regularMarketPrice"`
    Symbol string  `json:"symbol"`
    Time   int64   `json:"regularMarketTime"`
}
*/
