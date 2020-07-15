#![deny(unsafe_code)]
#![no_std]
#![no_main]

//use panic_halt as _;
extern crate panic_itm; // panic handler
use nb::block;

#[allow(unused_imports)]
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
// Определяем входную функцию.

#[entry]
fn main() -> ! {
    // Получаем управление над аппаратными средствами
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpio_c = dp.GPIOC.split(&mut rcc.apb2);

    // Конфигурируем пин c13 как двухтактный выход.
    // Регистр "crh" передаётся в функцию для настройки порта.
    // Для пинов 0-7, необходимо передавать регистр "crl".
    let mut led = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);
    // Конфигурируем системный таймер на запуск обновления каждую секунду.
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    let mut itm = cp.ITM;

    // Ждём пока таймер запустит обновление
    // и изменит состояние светодиода.
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
        iprintln!(&mut itm.stim[0], "hello wordl!")
    }
}
