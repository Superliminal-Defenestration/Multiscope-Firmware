#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;


use cortex_m_rt::entry;
use stm32f4xx_hal::{
    adc::{self, Adc, config::AdcConfig}, gpio::{self, Edge, PB6, PB10, PB11, gpiob, gpioc}, nb, pac::{self, ADC1, UART4, USART1, USART3, adc1, rcc::cfgr}, prelude::*, rcc::Config as RccConfig, serial::{self, Serial, config::Config}
};
use core::fmt::Write; // for pretty formatting of the serial output
#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {

    let mut p=pac::Peripherals::take().unwrap(); // Access the peripherals through the PAC
    let mut cp =  cortex_m::peripheral::Peripherals::take().unwrap(); // Access peripherals through hardware
    
    let mut rcc = p.RCC.constrain(); // Access the Reset + Clock Controller, and constrain it. 

    //ALl peripherals are either constrained or split. Split splits into individual registries with full access. Constrain provides a restricted, hardware-safe view.
  
    let mut syscfg = p.SYSCFG.constrain(&mut rcc); // Access the syscfg and constrain it!
    
    let gpiob = p.GPIOB.split(&mut rcc);   // GPIO-B port is a low-risk peripheral to access, so we'll just split it.
    let gpioc = p.GPIOC.split(&mut rcc);   // GPIO-B port is a low-risk peripheral to access, so we'll just split it.
    
    let mut keyLeft = gpiob.pb10.into_pull_down_input(); // The diagram has this as a pull-down'd input

    let mut clocks: stm32f4xx_hal::rcc::Rcc = rcc.freeze(RccConfig::hse(8.MHz()).sysclk(48.MHz())); // the "freeze" method applies a configuration to the clock config registry
    // the clock is configured for an 8MHz external oscilator on the OSC pins, caled the HSE. This is chained with a sysclk of 48MHz, which will be achieved through internal multiplication of thre clock's signal.
    let mut serial: Serial<UART4> = Serial::new(p.UART4, (gpioc.pc10,gpioc.pc11), Config::default().baudrate(3200.bps()), &mut clocks).unwrap();

    // let mut adcDriver0 = Adc::new(p.ADC1, true, AdcConfig::default(),&mut clocks );
    // value is moved to clocks. instance and configure an ADC.
    let mut p0 = gpiob.pb0.into_analog();
    

    loop { 


        //Read the data received on pc11 (UART4_RX)
        if let Ok(byte) = nb::block!(serial.read()){
            // echo the received byte back to the sender
            nb::block!(serial.write(byte)).ok();
        }


        // let adcResult    = adcDriver0.read( &mut p0);
        // if keyLeft.is_high() {
            
        writeln!(serial, "hello world").ok();

        // }

    }
}
