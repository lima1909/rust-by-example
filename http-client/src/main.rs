// extern crate simplelog;

use simplelog::*;
use log::{error, info};

fn inti_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
    ]).unwrap();
}

fn main() {
    inti_logger();

    info!("Hello, world!");
    error!("yay ...")
}
