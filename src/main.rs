#![deny(unsafe_code)]
#![no_std]
#![no_main]
#[allow(unused_extern_crates)]
extern crate panic_halt; // panic hnadler
use cortex_m_rt::entry;

extern crate hal;
use hal::pac; //mcu select

use hal::{prelude::*, timer::Timer};
//use pac::{TIM6};
#[allow(unused_imports)]
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use embedded_hal::digital::v2::OutputPin;
use nb::block;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    //for stm32f303
    // Конфигурируем пин c13 как двухтактный выход.
    // Регистр "crh" передаётся в функцию для настройки порта.
    // Для пинов 0-7, необходимо передавать регистр "crl".
    let mut gpio = dp.GPIOC.split(&mut rcc.ahb);
    let mut led = gpio
        .pc13
        .into_push_pull_output(&mut gpio.moder, &mut gpio.otyper);
    let mut timer = Timer::tim6(dp.TIM6, 1.khz(), clocks, &mut rcc.apb1);
    //for stm32f103
    /*let mut gpio = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpio.pc13.into_push_pull_output(&mut gpio.crh);
    //let mut timer = Timer::syst(cp.SYST,&clocks).start_count_down(1.hz());
    let mut timer = Timer::tim1(dp.TIM1,&clocks,&mut rcc.apb2).start_count_down(1.hz());
    */
    let mut itm = cp.ITM;
    iprintln!(&mut itm.stim[0], "hello wordl!");
    // Ждём пока таймер запустит обновление
    // и изменит состояние светодиода.
    loop {
        timer.start(1.hz());
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        timer.start(1.hz());
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
