#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, serial, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        let tx_pin = gpioa.pa2.into_alternate_af7();
        let rx_pin = gpioa.pa3.into_alternate_af7();

        let serial = serial::Serial::usart2(
            dp.USART2,
            (tx_pin, rx_pin),
            serial::config::Config::default().baudrate(9600.bps()),
            clocks,
        )
        .unwrap();

        let (mut tx, mut _rx) = serial.split();

        let mut value: u8 = 0;
        loop {
            writeln!(tx, "hello {}\r", value).unwrap();
            value += 1;

            led.set_high().unwrap();
            delay.delay_ms(100_u32);
            led.set_low().unwrap();
            delay.delay_ms(100_u32);
        }
    }

    loop {}
}
