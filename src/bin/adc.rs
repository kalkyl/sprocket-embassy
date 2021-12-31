// $ DEFMT_LOG=info cargo run --bin adc --release
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use sprocket_embassy as _; // global logger + panicking-behavior + memory layout

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::Delay;
use embassy::time::{Duration, Timer};
use embassy_stm32::Peripherals;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let mut adc = embassy_stm32::adc::Adc::new(p.ADC1, &mut Delay);
    let mut cal1 = p.PA6;
    let mut cal2 = p.PA7;

    loop {
        let voltage = adc.read(&mut cal1);
        info!("CAL 1: {}", voltage);
        let voltage = adc.read(&mut cal2);
        info!("CAL 2: {}", voltage);
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
