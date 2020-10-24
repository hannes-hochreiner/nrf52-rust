#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
use nrf52810_hal as hal;
use nrf52810_hal::gpio::Level;
use nrf52810_hal::prelude::OutputPin;
use nrf52810_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let p = hal::pac::Peripherals::take().unwrap();
    let cp = hal::pac::CorePeripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_14.into_push_pull_output(Level::Low);
    let mut delay = hal::delay::Delay::new(cp.SYST);
    let pins = hal::uarte::Pins {
        rxd: port0.p0_16.into_floating_input().degrade(),
        txd: port0.p0_18.into_push_pull_output(Level::Low).degrade(),
        cts: None,
        rts: None
    };
    let mut uart = hal::uarte::Uarte::new(p.UARTE0, pins, hal::uarte::Parity::EXCLUDED, hal::uarte::Baudrate::BAUD115200);

    loop {
        // your code goes here
        led.set_high().unwrap();
        delay.delay_ms(1000u32);
        led.set_low().unwrap();
        delay.delay_ms(1000u32);
        uart.write_str(&"test\n").unwrap();
    }
}
