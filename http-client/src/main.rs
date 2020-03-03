mod http;
mod logging;

use log::info;
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
}
