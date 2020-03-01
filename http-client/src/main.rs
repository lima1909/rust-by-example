use simplelog::*;
use log::info;

extern crate tokio;

mod http;
 
fn inti_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
    ]).unwrap();
}


#[tokio::main]
async fn main() {
    inti_logger();

    info!("start request:");
    
    http::get("https://query1.finance.yahoo.com/v7/finance/quote?lang=en-US&region=US&corsDomain=finance.yahoo.com&symbols=ABEC.DE")
    .await
    .unwrap();
}
