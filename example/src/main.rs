#![no_std]
#![no_main]

use embassy_executor::{ main, Spawner };
use pico_cdc;
use { defmt_rtt as _, panic_probe as _ };


#[main]
async fn main(spawner: Spawner) {
    spawner.spawn(pico_cdc::init()).unwrap();
}
