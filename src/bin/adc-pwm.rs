// $ DEFMT_LOG=info cargo run --bin adc-pwm --release
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use sprocket_embassy as _; // global logger + panicking-behavior + memory layout

use defmt::info;
use embassy::executor::Spawner;
use embassy::time::Delay;
use embassy::time::{Duration, Timer};
use embassy_stm32::gpio::NoPin;
use embassy_stm32::pwm::{Channel, Pwm};
use embassy_stm32::time::U32Ext;
use embassy_stm32::Peripherals;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let mut adc = embassy_stm32::adc::Adc::new(p.ADC1, &mut Delay);
    let mut cal1 = p.PA6;
    let mut cal2 = p.PA7;

    let mut pwm = Pwm::new(p.TIM2, p.PA0, NoPin, NoPin, NoPin, 10.khz());
    let max = pwm.get_max_duty();
    pwm.enable(Channel::Ch1);

    info!("PWM initialized");
    info!("PWM max duty {}", max);

    loop {
        let voltage = adc.read(&mut cal1);
        info!("CAL 1: {}", voltage);
        let voltage = adc.read(&mut cal2);
        info!("CAL 2: {}", voltage);
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
