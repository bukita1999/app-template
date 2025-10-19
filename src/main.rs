#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our HAL crate
use stm32f1xx_hal::{pac, prelude::*};

// Same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[entry]
fn main() -> ! {
    defmt::println!("LED Blink Demo - STM32F103C8T6 Blue Pill");

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock (unused but required for HAL)
    let _rcc = dp.RCC.constrain();

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output (Blue Pill onboard LED)
    // Note: Blue Pill LED is active low (LED lights when pin is low)
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    defmt::println!("Starting LED blink loop...");

    loop {
        // LED ON for ~1 second (Blue Pill LED is active low)
        led.set_low();
        defmt::println!("LED ON");

        // Simple delay loop for ~1 second at 8MHz (approximate)
        // STM32F103 runs at 8MHz by default with this setup
        cortex_m::asm::delay(8_000_000);

        // LED OFF for ~1 second
        led.set_high();
        defmt::println!("LED OFF");

        // Simple delay loop for ~1 second at 8MHz (approximate)
        cortex_m::asm::delay(8_000_000);
    }
}
