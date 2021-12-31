// $ DEFMT_LOG=info cargo run --bin pwm --release
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use sprocket_embassy as _; // global logger + panicking-behavior + memory layout

use defmt::info;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::gpio::NoPin;
use embassy_stm32::pwm::{Channel, Pwm};
use embassy_stm32::time::U32Ext;
use embassy_stm32::Peripherals;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let mut pwm = Pwm::new(p.TIM2, p.PA0, NoPin, NoPin, NoPin, 10.khz());
    let max = pwm.get_max_duty();
    pwm.enable(Channel::Ch1);

    info!("PWM max duty {}", max);

    loop {
        pwm.set_duty(Channel::Ch1, 0);
        Timer::after(Duration::from_millis(300)).await;
        pwm.set_duty(Channel::Ch1, max - 300);
        Timer::after(Duration::from_millis(300)).await;
        pwm.set_duty(Channel::Ch1, max - 50);
        Timer::after(Duration::from_millis(300)).await;
        pwm.set_duty(Channel::Ch1, max - 1);
        Timer::after(Duration::from_millis(300)).await;
    }
}
