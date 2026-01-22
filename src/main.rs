#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;


use cortex_m_rt::entry;
use stm32f4xx_hal::{
   
    
    adc::{self, Adc, config::AdcConfig}, gpio::{self, AF10, Edge, PB6, PB10, PB11, alt::{fmc::A11, otg_fs::Dm}, gpiob, gpioc}, nb, 
    otg_fs::{USB, UsbBus, UsbBusType}, pac::{self, ADC1, UART4, USART1, USART3, adc1::{self, ltr}, rcc::cfgr}, prelude::*, rcc::Config as RccConfig, serial::{self, Serial, config::Config}
};
use core::fmt::Write; // for pretty formatting of the serial output

use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {

    let mut p=pac::Peripherals::take().unwrap(); // Access the peripherals through the PAC
    let mut cp =  cortex_m::peripheral::Peripherals::take().unwrap(); // Access peripherals through hardware
    
    let mut rcc = p.RCC.constrain(); // Access the Reset + Clock Controller, and constrain it. 

    //ALl peripherals are either constrained or split. Split splits into individual registries with full access. Constrain provides a restricted, hardware-safe view.
  
    let mut syscfg = p.SYSCFG.constrain(&mut rcc); // Access the syscfg and constrain it!
    
    let gpioa = p.GPIOA.split(&mut rcc);   // GPIO-B port is a low-risk peripheral to access, so we'll just split it.
    let gpiob = p.GPIOB.split(&mut rcc);   // GPIO-B port is a low-risk peripheral to access, so we'll just split it.
    let gpioc = p.GPIOC.split(&mut rcc);   // GPIO-B port is a low-risk peripheral to access, so we'll just split it.
    
    let mut keyLeft = gpiob.pb10.into_pull_down_input(); // The diagram has this as a pull-down'd input

    let mut clocks: stm32f4xx_hal::rcc::Rcc = rcc
        .freeze(RccConfig::hse(8.MHz()).sysclk(48.MHz()).require_pll48clk()); // the "freeze" method applies a configuration to the clock config registry
    // the clock is configured for an 8MHz external oscilator on the OSC pins, caled the HSE. This is chained with a sysclk of 48MHz, which will be achieved through internal multiplication of thre clock's signal.
    let mut serial: Serial<UART4> = Serial::new(p.UART4, (gpioc.pc10,gpioc.pc11), Config::default().baudrate(3200.bps()), &mut clocks).unwrap(); // value is moved to clocks.

    // let dm: gpio::Pin<'A', 11, gpio::Alternate<10>> = gpioa.pa11.into_alternate::<10>();
    let usb = USB::new(
        (p.OTG_FS_GLOBAL, p.OTG_FS_DEVICE, p.OTG_FS_PWRCLK),
        (gpioa.pa11.into_alternate::<10>(), gpioa.pa12.into_alternate::<10>()),
        &clocks.clocks
    );
    
    // let usb = USB {

        // // usb_global: p.OTG_FS_GLOBAL,
        // // usb_device: p.OTG_FS_DEVICE,
        // // usb_pwrclk: p.OTG_FS_PWRCLK,
        // // pin_dm: stm32f4xx_hal::gpio::alt::otg_fs::Dm::PA11(dm),
        // // pin_dp: stm32f4xx_hal::gpio::alt::otg_fs::Dp::PA12(gpioa.pa12.into_alternate::<10>()),
    //     // hclk: 48.MHz(),
    // };



    // Endpoint packet memory (size must be enough for all endpoints)
    static mut EP_MEMORY: [u32; 1024] = [0; 1024];

    
    // static   mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
    let usb_bus = unsafe {

        USB_BUS= Some(UsbBus::new(usb, &mut EP_MEMORY));
        USB_BUS.as_ref().unwrap()
    };
    let mut usb_port = SerialPort::new(usb_bus);


    let mut usb_dev = UsbDeviceBuilder::new(
        usb_bus,
        UsbVidPid(0x5824, 0x27dd), // THIS NEEDS TO BE CHANGED TO A REAL VID/PID IF YOU WANT TO DISTRIBUTE THE FIRMWARE
    ).device_class(USB_CLASS_CDC)
    .strings(&[StringDescriptors::default()
            .manufacturer("Superliminal Defenestration")
            .product("Multiscope Firmware")
            .serial_number("TEST")])
    .unwrap()
    .build();




    // writeln!(serial, "about to instance adc").ok();

    

    /// yeahhh so you deadass can't use the ADC when emulating with renode since renode emulates the individual ADC registries/peripherals
    // but DOESNT emulate the common registries (CADC_CRR etc) 
    // if you look at the init behavior for the adc stuff in the hal it'll loop/hang until the adc gets a calibrtion bit and an enable bit / ready bit 
    // which obv we are not getting and no matter how I try to mess with the memory directly the area that would be the CADC_CRR is inaccessible
    // since it doesn't start at a new page (renode limits memory creation to align to the pages) and there's no way to 
    // allias memory
    // #[cfg(feature = "emulation")]
    // let mut adcDriver0 = (); // dummy placeholder for ADC

    // #[cfg(not(feature = "emulation"))] 
    // let mut adcDriver0 = Adc::new( // value is moved to clocks. instance and configure an ADC.
    //     p.ADC1,
    //     false,
    //     AdcConfig::default().scan(adc::config::Scan::Disabled),
    //     &mut clocks
    // );    

    // writeln!(serial, "loop").ok();

    // #[cfg(not(feature = "emulation"))]
    // adcDriver0.enable();

    // #[cfg(not(feature = "emulation"))]
    // let mut p0 = gpiob.pb0.into_analog();
    

    loop { 

        if usb_dev.poll(&mut [&mut usb_port]) {
            // SerialPort from usbd_serial does not implement core::fmt::Write; use its write method instead.
            usb_port.write(b" hi \n").ok();
            
        }

        /////// IMPORTANT: The below code is commented out to prevent blocking behavior 
        // AS IT TURNS OUT, NO BLOCKING CAN BE DONE ON THE LOOP SINCE BLOCKING WILL CAUSE ENUMERATION FAILURE!
        
        // //Read the data received on pc11 (UART4_RX)
        // if let Ok(byte) = nb::block!(serial.read()){
        //     // echo the received byte back to the sender
        //     nb::block!(serial.write(byte)).ok();
        // }


        // let adc_result: Result<u16, nb::Error<()>>= (adcDriver0.read( &mut p0));

        if keyLeft.is_high() {
            
            writeln!(serial, "hello world").ok();
            // writeln!(serial, "{}", adc_result.unwrap()).ok();
        }
        
    }
}
