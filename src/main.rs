#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;


use cortex_m_rt::entry;
use stm32f4xx_hal::{
    adc::{self, Adc, config::AdcConfig}, gpio::{self, AF5, AF15, Alternate, Edge, PB6, PB10, PB11, Pin, alt::fmc::A5, gpiob, gpioc}, hal::spi, nb, pac::{self, ADC1, UART4, USART1, USART3, adc1, dcmi::mis, rcc::cfgr}, prelude::*, rcc::Config as RccConfig, serial::{self, Serial, config::Config}, spi::{Mode, Phase, Polarity, Spi}, time::Hertz
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
    let gpioa = p.GPIOA.split(&mut rcc);

    //init SPI bus
    let mut  sck = gpioa.pa5.into_alternate::<5>() ; // We have to tell the pin to register as an SPI pin instead of GPIO
    // AF 5 (Alt Func 5) is the func register for SPI
    let mut mosi: Pin<'A', 7, Alternate<_>> = gpioa.pa7.into_alternate::<5>() ; // We have to tell the pin to register as an SPI pin instead of GPIO
    let  mut miso: Pin<'A', 6, Alternate<_>> = gpioa.pa6.into_alternate::<5>() ; // We have to tell the pin to register as an SPI pin instead of GPIO
    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };
    let mut spiBus = Spi::new(p.SPI1, (core::prelude::v1::Some(sck),core::prelude::v1::Some(miso),core::prelude::v1::Some(mosi)), spi_mode,Hertz::MHz(8), &mut rcc);

    let mut keyLeft = gpiob.pb10.into_pull_down_input(); // The diagram has this as a pull-down'd input

    let mut clocks: stm32f4xx_hal::rcc::Rcc = rcc.freeze(RccConfig::hse(8.MHz()).sysclk(48.MHz())); // the "freeze" method applies a configuration to the clock config registry
    // the clock is configured for an 8MHz external oscilator on the OSC pins, caled the HSE. This is chained with a sysclk of 48MHz, which will be achieved through internal multiplication of thre clock's signal.
    let mut serial: Serial<UART4> = Serial::new(p.UART4, (gpioc.pc10,gpioc.pc11), Config::default().baudrate(3200.bps()), &mut clocks).unwrap(); // value is moved to clocks.
    

    writeln!(serial, "about to instance adc").ok();

    

    /// yeahhh so you deadass can't use the ADC when emulating with renode since renode emulates the individual ADC registries/peripherals
    // but DOESNT emulate the common registries (CADC_CRR etc) 
    // if you look at the init behavior for the adc stuff in the hal it'll loop/hang until the adc gets a calibrtion bit and an enable bit / ready bit 
    // which obv we are not getting and no matter how I try to mess with the memory directly the area that would be the CADC_CRR is inaccessible
    // since it doesn't start at a new page (renode limits memory creation to align to the pages) and there's no way to 
    // allias memory
    #[cfg(feature = "emulation")]
    let mut adcDriver0 = (); // dummy placeholder for ADC

    #[cfg(not(feature = "emulation"))] 
    let mut adcDriver0 = Adc::new( // value is moved to clocks. instance and configure an ADC.
        p.ADC1,
        false,
        AdcConfig::default().scan(adc::config::Scan::Disabled),
        &mut clocks
    );    

    writeln!(serial, "loop").ok();

    #[cfg(not(feature = "emulation"))]
    adcDriver0.enable();

    #[cfg(not(feature = "emulation"))]
    let mut p0 = gpiob.pb0.into_analog();
    

    loop { 


        //Read the data received on pc11 (UART4_RX)
        if let Ok(byte) = nb::block!(serial.read()){
            // echo the received byte back to the sender
            nb::block!(serial.write(byte)).ok();
        }
        let spi_res = spiBus.write(b"hey").ok();
        
        // let adc_result: Result<u16, nb::Error<()>>= (adcDriver0.read( &mut p0));

        if keyLeft.is_high() {
            
            writeln!(serial, "hello world").ok();
            // writeln!(serial, "{}", adc_result.unwrap()).ok();
        }

    }
}
