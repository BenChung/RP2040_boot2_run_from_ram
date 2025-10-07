//! Template

#![no_std]
#![no_main]

use core::{usize};
use embassy_executor::Spawner;
use embassy_rp::clocks::ClockConfig;
use embassy_rp::config;
use {defmt_rtt as _, panic_probe as _};

#[cfg_attr(not(target_os = "macos"), link_section = ".data.example")]
static EXAMPLE:[u32; 1] = [1];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(config::Config::new(
        ClockConfig::system_freq(200_000_000).unwrap(),
    ));
    if unsafe { EXAMPLE.as_ptr().read_volatile() != 1 } {
        panic!();
    }
}
