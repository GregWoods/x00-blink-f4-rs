//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]


use panic_halt as _;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f4::stm32f411 as pac;


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

    // Enable clock on port C
//  rcc.apb2enr.write(|w| w.iopcen().set_bit());        //stm32f1 
    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());       //stm32f4

    let gpioc = dp.GPIOC;

    // Set pin C13 as general purpose output, with push/pull mode, 2MHz max
    //gpioc.apb2.write(|w| w.cnf13().bits(0b00)
    //                      .mode13().bits(0b10));     //stm32f1

    //gpioc.moder.write(|w| w.moder13().bits(0x01));      //General purpose output mode
    gpioc.moder.write(|w| w.moder13().output());        
    //gpioc.otyper.write(|w| w.ot13().clear_bit());       //Output push-pull
    gpioc.otyper.write(|w| w.ot13().push_pull());
    //gpioc.ospeedr.write(|w| w.ospeedr13().bits(0x00));  //low speed
    gpioc.ospeedr.write(|w| w.ospeedr13().low_speed());
    //gpioc.pupdr.write(|w| w.pupdr13().bits(0x00));      //No pull-up, pull-down (not needed, since we use Output: push-pull)    
    gpioc.pupdr.write(|w| w.pupdr13().floating());        //helper method used instead of .bits() which is marked unsafe
    
    
    // Restart the SysTick counter.
    syst.clear_current();

    loop {
        // Toggle the LED every SysTick tick.
        while !syst.has_wrapped() {};
        gpioc.odr.write(|w| w.odr13().set_bit());
        while !syst.has_wrapped() {};
        gpioc.odr.write(|w| w.odr13().clear_bit());
    }
}