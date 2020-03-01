mod http;
mod logging;
 
#[tokio::main]
async fn main() {
    logging::init();
    
    http::get("https://query1.finance.yahoo.com/v7/finance/quote?lang=en-US&region=US&corsDomain=finance.yahoo.com&symbols=ABEC.DE")
    .await
    .unwrap();
}