#![no_main]
#![no_std]

mod rtt_logger;

use cortex_m_rt::entry;
use log::info;
use panic_rtt_target as _;
use stm32h7xx_hal::{gpio::PinState, hal::digital::v2::{InputPin, OutputPin}, pac, prelude::*};

fn control_led<In: InputPin<Error = E>, Out: OutputPin<Error = E>, E>(
    input_pin: &In,
    led_pin: &mut Out,
) -> Result<(), E>{
    if input_pin.is_low()? {
        info!("Key is low");
        led_pin.set_high()?;
    } else {
        info!("Key is high");
        led_pin.set_low()?;
    }
    Ok(())
}

#[entry]
fn main() -> ! {
    rtt_logger::init(log::LevelFilter::Debug);
    // Get cortex device peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get the stm32h7 device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Power Configuration
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    
    // RCC Initialization
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(200.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // GPIO Initialization
    // LED GPIO
    let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
    let mut ledr = gpioh.ph10.into_push_pull_output_in_state(PinState::High);
    // let mut ledg = gpioh.ph11.into_push_pull_output_in_state(PinState::High);
    // let mut ledb = gpioh.ph12.into_push_pull_output_in_state(PinState::High);

    // Key GPIO
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let key = gpioc.pc13.into_pull_up_input();

    // Delay Initialization
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        // 点灯
        control_led(&key, &mut ledr).unwrap();
        delay.delay_ms(100_u32);
    }
}