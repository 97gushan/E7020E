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
    adc,
    pwm,
    timer::Timer,
};
use stm32l0xx_hal as hal;
// use communicator::{Message, Channel};
use cortex_m_semihosting::hprintln;
use ssd1306::{mode::TerminalMode, interface::*, prelude::*, Builder};

#[rtfm::app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {

    struct Resources {
        INT: pac::EXTI,
        SX1276_DIO0: gpiob::PB2<Input<PullUp>>,
        LED: gpiob::PB6<Output<PushPull>>,
        POT: gpioa::PA4<Analog>,
        ADC: adc::Adc,
        TIMER: Timer<pac::TIM2>,
        TIMER2: Timer<pac::TIM3>,
        TIMER3: Timer<pac::TIM21>,
        TIMER4: Timer<pac::TIM22>,
        BUZZER: gpiob::PB5<Output<PushPull>>,
        #[init(false)]
        STATE: bool,
        #[init(false)]
        ON: bool,
        TV: TerminalMode<SpiInterface<spi::Spi<pac::SPI2,(gpiob::PB13<Input<Floating>>, NoMiso, gpiob::PB15<Input<Floating>>)>,gpiob::PB9<Output<PushPull>>>>,

    }

    #[init(spawn = [write_to_screen])]
    fn init(cx: init::Context) -> init::LateResources {
        // Configure the clock.
        let mut rcc = cx.device.RCC.freeze(Config::hsi16());
        let mut syscfg = syscfg::SYSCFG::new(cx.device.SYSCFG, &mut rcc);

        // Acquire the GPIOB peripheral. This also enables the clock for GPIOB in
        // the RCC register.
        let gpioa = cx.device.GPIOA.split(&mut rcc);
        let gpiob = cx.device.GPIOB.split(&mut rcc);

        let exti = cx.device.EXTI;

        let mut timer = cx.device.TIM2.timer(440.hz(), &mut rcc);
        timer.listen();

        let timer2 = cx.device.TIM3.timer(1.hz(), &mut rcc);
        let timer3 = cx.device.TIM21.timer(6.hz(), &mut rcc);
        let timer4 = cx.device.TIM22.timer(20.hz(), &mut rcc);

        
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
        let mut tv: TerminalMode<_> = Builder::new().connect_spi(spi, dc).into();
        tv.reset(&mut res, &mut delay);
        tv.init().unwrap();
        cx.spawn.write_to_screen().unwrap();
 
        // Configure PB5 as output.
        let mut led = gpiob.pb6.into_push_pull_output();
        led.set_low().ok();

        let pot = gpioa.pa4.into_analog();
        let buzzer = gpiob.pb5.into_push_pull_output();
        let adc = cx.device.ADC.constrain(&mut rcc);
        // Return the initialised resources.
        init::LateResources {
            INT: exti,
            SX1276_DIO0: sx1276_dio0,
            LED: led,
            POT: pot,
            ADC: adc,
            TIMER: timer,
            TIMER2: timer2,
            TIMER3: timer3,
            TIMER4: timer4,
            BUZZER: buzzer,
            TV: tv,
        }
    }

    #[task(binds = EXTI2_3, priority = 3, resources = [SX1276_DIO0, INT], spawn = [button_event])]
    fn exti2_3(cx: exti2_3::Context) {
        cx.resources.INT.clear_irq(cx.resources.SX1276_DIO0.pin_number());
        cx.spawn.button_event().unwrap();
    }

    #[task(capacity = 4, priority = 3, resources = [ON, LED])]
    fn button_event(cx: button_event::Context) {

            let mut on = cx.resources.ON;
            let mut led = cx.resources.LED;

            if *on {
                led.set_low().unwrap();
                *on = false;
            } else {
                led.set_high().unwrap();
                *on = true;
            }
        }

    #[task(capacity = 4, priority = 2, resources = [POT, ADC, TIMER, TIMER2, TIMER3, TIMER4, ON, STATE])]
    fn buzzer_control(cx: buzzer_control::Context) {

        let val: u16 = cx.resources.ADC.read(cx.resources.POT).unwrap();        //let mut rcc = device.RCC.freeze(Config::hsi16());
        let tim2 = cx.resources.TIMER2;
        let tim3 = cx.resources.TIMER3;
        let tim4 = cx.resources.TIMER4;
        let mut state = cx.resources.STATE;

        let mut on = cx.resources.ON;
        
        on.lock(|on| {
            if !*on{ 
                state.lock(|state| {
                    *state = false;
                });
                tim2.unlisten();
                tim3.unlisten();
                tim4.unlisten();
            }else if val <= 1500{
                tim2.listen();
                tim3.unlisten();
                tim4.unlisten();
            }
            else if val <= 3000{
                tim2.unlisten();
                tim3.listen();
                tim4.unlisten();
            }
            else{
                tim2.unlisten();
                tim3.unlisten();
                tim4.listen();
            }
        });
    }
        
    #[task(capacity = 4, priority = 1, spawn = [buzzer_control], resources = [POT, ADC, TV])]
    fn write_to_screen(cx: write_to_screen::Context) {

        let mut state: u16 = 0;
        let mut updateScreen = true;
        let mut tv = cx.resources.TV;
        let mut adc = cx.resources.ADC;
        let mut val: u16 = 0;
        let mut pot = cx.resources.POT;
        loop {
            cx.spawn.buzzer_control().unwrap();

            adc.lock(|adc| {
                pot.lock(|pot| {
                    val = adc.read(pot).unwrap(); 

                });
                
            });

            if val <= 1500 {
                if state != 1 {
                    updateScreen = true;
                }
                
                state = 1;
            } else if val <= 3000 {
                if state != 2 {
                    updateScreen = true;
                }
                state = 2;
            } else {
                if state != 3 {
                    updateScreen = true;
                }
                state = 3;
            }
            
            if updateScreen {
                tv.clear().unwrap();
                
                if state == 1 {
                    tv.print_char(' ').unwrap();
                    tv.print_char('F').unwrap();
                    tv.print_char('A').unwrap();
                    tv.print_char('R').unwrap();
                } else if state == 2 {
                    tv.print_char(' ').unwrap();
                    tv.print_char('N').unwrap();
                    tv.print_char('E').unwrap();
                    tv.print_char('A').unwrap();
                    tv.print_char('R').unwrap();
                } else if state == 3 {
                    tv.print_char(' ').unwrap();
                    tv.print_char('C').unwrap();
                    tv.print_char('R').unwrap();
                    tv.print_char('A').unwrap();
                    tv.print_char('S').unwrap();
                    tv.print_char('H').unwrap();
                    tv.print_char('!').unwrap();
                    tv.print_char('!').unwrap();
                }
                updateScreen = false;
                tv.flush().unwrap();
            }
        }
    }
    
    #[task(binds = TIM2, priority= 4, resources = [BUZZER, TIMER, STATE])]
    fn tim2(cx: tim2::Context) {
        // Clear the interrupt flag.
        cx.resources.TIMER.clear_irq();
        if *cx.resources.STATE {
            if cx.resources.BUZZER.is_set_high().unwrap(){
                cx.resources.BUZZER.set_low().unwrap();
            } else {
                cx.resources.BUZZER.set_high().unwrap();
            }
        }
    }
    

    #[task(binds = TIM3, priority= 2, resources = [TIMER2], spawn = [toggle])]
    fn tim3(cx: tim3::Context) {
        // Clear the interrupt flag.
        cx.resources.TIMER2.clear_irq();  
        cx.spawn.toggle().unwrap();       
    }

    #[task(binds = TIM21, priority= 2, resources = [TIMER3], spawn = [toggle])]
    fn tim21(cx: tim21::Context) {
        // Clear the interrupt flag.
        cx.resources.TIMER3.clear_irq();
        cx.spawn.toggle().unwrap();           
    }

    #[task(binds = TIM22, priority= 2, resources = [TIMER4], spawn = [toggle])]
    fn tim22(cx: tim22::Context) {
        // Clear the interrupt flag.
        cx.resources.TIMER4.clear_irq();  
        cx.spawn.toggle().unwrap();   
    }

    #[task(capacity = 4, priority = 3, resources = [STATE])]
    fn toggle(cx: toggle::Context) {
        let mut state = cx.resources.STATE;
        state.lock(|state| {
            if *state {
                *state = false;
            } else {
                *state = true;
            }
        });
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART4_USART5();
        fn USART1();
        fn USART2();
    }
};
