//! Template

#![no_std]
#![no_main]

use core::usize;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::clocks::ClockConfig;
use embassy_rp::config;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(config::Config::new(
        ClockConfig::system_freq(200_000_000).unwrap(),
    ));
    info!("Hello World!");
}
