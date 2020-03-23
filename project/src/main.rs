#![cfg_attr(not(test), no_std)]
#![no_main]
#![allow(deprecated)]


extern crate panic_semihosting;
extern crate stm32l0xx_hal;

use hal::{
    exti::TriggerEdge,
    delay::Delay,
    gpio::*,
    pac,
    prelude::*,
    rcc::Config,
    spi::{self, Mode, NoMiso, Phase, Polarity},
    syscfg,
};
use stm32l0xx_hal as hal;
// use communicator::{Message, Channel};
use cortex_m_semihosting::hprintln;

use ssd1306::{mode::TerminalMode, prelude::*, Builder};

#[rtfm::app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {

    struct Resources {
        INT: pac::EXTI,
        SX1276_DIO0: gpiob::PB2<Input<PullUp>>,
        LED: gpiob::PB6<Output<PushPull>>,
        
        #[init(false)]
        STATE: bool,
    }

    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        // Configure the clock.
        let mut rcc = cx.device.RCC.freeze(Config::hsi16());
        let mut syscfg = syscfg::SYSCFG::new(cx.device.SYSCFG, &mut rcc);

        // Acquire the GPIOB peripheral. This also enables the clock for GPIOB in
        // the RCC register.
        let gpioa = cx.device.GPIOA.split(&mut rcc);
        let gpiob = cx.device.GPIOB.split(&mut rcc);
        let gpioc = cx.device.GPIOC.split(&mut rcc);

        let exti = cx.device.EXTI;
        

        // Configure PB4 as input.
        let sx1276_dio0 = gpiob.pb2.into_pull_up_input();
        // Configure the external interrupt on the falling edge for the pin 2.
        exti.listen(
            &mut syscfg,
            sx1276_dio0.port(),
            sx1276_dio0.pin_number(),
            TriggerEdge::Rising,
        );

        let sck = gpiob.pb13;
        let mosi = gpiob.pb15;
        let _nss = gpiob.pb12.into_push_pull_output();
        let dc = gpiob.pb9.into_push_pull_output();
        let mut res = gpiob.pb8.into_push_pull_output();

        // Initialise the SPI peripheral.   
        let spi = cx.device
                        .SPI2
                        .spi((sck, NoMiso, mosi), 
                        spi::MODE_0, 1_000_000.hz(), &mut rcc);



        let mut delay = Delay::new(cx.core.SYST, rcc.clocks);

        let mut disp: TerminalMode<_> = Builder::new().connect_spi(spi, dc).into();

        disp.reset(&mut res, &mut delay).unwrap();
        disp.init().unwrap();

        disp.clear().unwrap();

        disp.print_char('G').unwrap();
        disp.print_char('o').unwrap();
        disp.print_char('d').unwrap();
        disp.print_char('s').unwrap();
        disp.print_char('p').unwrap();
        disp.print_char('e').unwrap();
        disp.print_char('e').unwrap();
        disp.print_char('d').unwrap();
        disp.print_char('!').unwrap();

        disp.flush().unwrap();

        // Configure PB5 as output.
        let mut led = gpiob.pb6.into_push_pull_output();
        led.set_low().ok();

        hprintln!("Hello, world!").unwrap();

        // Return the initialised resources.
        init::LateResources {
            INT: exti,
            SX1276_DIO0: sx1276_dio0,
            LED: led,
        }
    }

    #[task(binds = EXTI2_3, priority = 1, resources = [SX1276_DIO0, INT], spawn = [button_event])]
    fn exti2_3(cx: exti2_3::Context) {
        hprintln!("{}", cx.resources.SX1276_DIO0.pin_number()).unwrap();
        cx.resources.INT.clear_irq(cx.resources.SX1276_DIO0.pin_number());
        cx.spawn.button_event().unwrap();
    }

    #[task(capacity = 4, priority = 2, resources = [LED, STATE])]
    fn button_event(cx: button_event::Context) {

        if(*cx.resources.STATE) {
            cx.resources.LED.set_high().unwrap();
            *cx.resources.STATE = false;
        } else {
            cx.resources.LED.set_low().unwrap();
            *cx.resources.STATE = true;
        }

        hprintln!("button event").unwrap();
    }
    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART4_USART5();
    }
};
