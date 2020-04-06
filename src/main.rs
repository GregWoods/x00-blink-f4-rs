//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

use panic_halt as _;

//use nb::block;


use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f1::stm32f103 as pac;

//use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC;

    // Set up the SysTick peripheral.
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(2_100_000); //internal in clock ticks
    syst.enable_counter();

    // APB Adv.Periph.Bus 2 port enable register. Enable port A
    rcc.apb2enr.write(|w| w.iopaen().set_bit());        
    let gpioa = dp.GPIOA;

    // Set pin A7 as general purpose output, with push/pull mode, 2MHz max
    gpioa.crl.write(|w| w.cnf7().bits(0b00).mode7().bits(0b10));
    
    // Restart the SysTick counter.
    syst.clear_current();

    loop {
        // Toggle the LED every SysTick tick.
        while !syst.has_wrapped() {};
        gpioa.odr.write(|w| w.odr7().set_bit());
        while !syst.has_wrapped() {};
        gpioa.odr.write(|w| w.odr7().clear_bit());
    }
}