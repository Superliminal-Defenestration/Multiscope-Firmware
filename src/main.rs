#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;


use cortex_m_rt::entry;
use stm32f4xx_hal::{
    gpio::{self, Edge, gpiob, gpioc}, pac::{self, rcc::cfgr}, prelude::*, rcc::Config
};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {

    let mut p=pac::Peripherals::take().unwrap();
    let mut cp =  cortex_m::peripheral::Peripherals::take().unwrap();
    
    let mut rcc = p.RCC.constrain();
  
    let mut syscfg = p.SYSCFG.constrain(&mut rcc);
    
    let gpiob = p.GPIOB.split(&mut rcc);  
    let mut keyLeft = gpiob.pb10.into_pull_down_input();

    let clocks = rcc.freeze(Config::hse(8.MHz()).sysclk(48.MHz())); // the "freeze" method applies a configuration to the clock config registry
    // the clock is configured for an 8MHz external oscilator on the OSC pins, caled the HSE. This is chained with a sysclk of 48MHz, which will be achieved through internal multiplication of thre clock's signal.
    

 

    loop {

        if keyLeft.is_high() {

            

        }

    }
}
