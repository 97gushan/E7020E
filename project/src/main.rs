#![cfg_attr(not(test), no_std)]
#![no_main]
#![allow(deprecated)]


extern crate panic_semihosting;
extern crate stm32l0xx_hal;

use hal::{
    exti::TriggerEdge,
    gpio::*,
    pac,
    prelude::*,
    rcc::Config,
    spi,
    syscfg,
    adc,
    pwm,
    timer::Timer,
};
use stm32l0xx_hal as hal;
use communicator::{Message, Channel};
use heapless::consts::*;
use cortex_m_semihosting::hprintln;


#[rtfm::app(device = stm32l0xx_hal::pac)]
const APP: () = {
    static mut INT: pac::EXTI = ();
    static mut SX1276_DIO0: gpiob::PB2<Input<PullUp>> = ();
    static mut LED: gpiob::PB6<Output<PushPull>> = ();
    static mut STATE: bool = false;
    static mut POT: gpioa::PA4<Analog> = ();
    static mut ADC: adc::Adc = ();
    static mut TIMER: Timer<pac::TIM2> = ();
    static mut BUZZER: gpiob::PB5<Output<PushPull>> = ();

    #[init()]
    fn init() -> init::LateResources {
        // Configure the clock.
        let mut rcc = device.RCC.freeze(Config::hsi16());
        let mut syscfg = syscfg::SYSCFG::new(device.SYSCFG, &mut rcc);

        // Acquire the GPIOB peripheral. This also enables the clock for GPIOB in
        // the RCC register.
        let gpioa = device.GPIOA.split(&mut rcc);
        let gpiob = device.GPIOB.split(&mut rcc);
        let gpioc = device.GPIOC.split(&mut rcc);

        let mut timer = device.TIM2.timer(4.hz(), &mut rcc);
        timer.listen();

        let exti = device.EXTI;
        
        // Configure PB4 as input.
        let sx1276_dio0 = gpiob.pb2.into_pull_up_input();
        // Configure the external interrupt on the falling edge for the pin 2.
        exti.listen(
            &mut syscfg,
            sx1276_dio0.port(),
            sx1276_dio0.pin_number(),
            TriggerEdge::Rising,
        );

        let sck = gpiob.pb3;
        let miso = gpioa.pa6;
        let mosi = gpioa.pa7;
        let nss = gpioa.pa15.into_push_pull_output();

        // Initialise the SPI peripheral.
        let mut _spi = device
            .SPI1
            .spi((sck, miso, mosi), spi::MODE_0, 1_000_000.hz(), &mut rcc);


        // Configure PB5 as output.
        let mut led = gpiob.pb6.into_push_pull_output();
        led.set_low().ok();


        let pot = gpioa.pa4.into_analog();
        let buzzer = gpiob.pb5.into_push_pull_output();
        let adc = device.ADC.constrain(&mut rcc);

        hprintln!("Hello, world!").unwrap();

        // Return the initialised resources.
        init::LateResources {
            INT: exti,
            SX1276_DIO0: sx1276_dio0,
            LED: led,
            POT: pot,
            ADC: adc,
            TIMER: timer,
            BUZZER: buzzer,
        }
    }

    #[interrupt(priority = 3, resources = [SX1276_DIO0, INT], spawn = [button_event])]
    fn EXTI2_3() {
        hprintln!("{}", resources.SX1276_DIO0.pin_number()).unwrap();
        
        let int = resources.INT;
        let led = resources.SX1276_DIO0;

        int.clear_irq(led.pin_number());
        spawn.button_event().unwrap();
    }

    #[task(capacity = 4, priority = 2, resources = [LED, STATE, POT, ADC])]
    fn button_event() {

        if *resources.STATE {
            resources.LED.set_high().unwrap();
            *resources.STATE = false;
        } else {
            resources.LED.set_low().unwrap();
            *resources.STATE = true;
        }

        let val: u16 = resources.ADC.read(resources.POT).unwrap();
        hprintln!("addc: {:?}", val).unwrap();

        hprintln!("button event").unwrap();
    }

    #[interrupt(priority= 2, resources = [BUZZER, TIMER])]
    fn TIM2() {
        // Clear the interrupt flag.
        resources.TIMER.clear_irq();

        // Change the LED state on each interrupt.
        if resources.BUZZER.is_set_high().unwrap() {
            resources.BUZZER.set_low().unwrap();
        } else {
            resources.BUZZER.set_high().unwrap();
        }
    }
    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART4_USART5();
    }
};
