# stm32f1-00-blink

A microcontroller version of "hello, world", for the STM32F103 Blue Pill board.
Blinks the onboard LED.

## Steps

* clone this repository
* Plug your STLink-v2 clone into you PC, with the headers connected correctly to the Blue Pill
* `cargo build` from the VS Code built-in Terminal (Ctrl+')
* Start debugging from within VS Code (Ctrl+Shift+D)

## Learning Rust

I hope this repo is useful as part of your embedded Rust journey. 

It intentionally keeps things simple, and avoids the use a HAL (Hardware abstraction layer).

My limited experience of microcontrollers tells me that you need to understand the chip's Reference Manual in order to use it. 
By using the Peripheral Access Crate (stm32f1) instead of a HAL, you can more easily see how your code directly relates to 
the registers in the microcontroller.


