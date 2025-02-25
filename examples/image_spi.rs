//! Draw a 1 bit per pixel black and white image. On a 128x64 sh1106 display over SPI.
//!
//! Image was created with ImageMagick:
//!
//! ```bash
//! convert rust.png -depth 1 gray:rust.raw
//! ```
//!
//! This example is for the STM32F103 "Blue Pill" board using SPI.
//!
//! Wiring connections are as follows:
//!
//! ```
//!      Display -> Blue Pill
//!          GND -> GND
//!          VCC -> 3.3V or 5V (check your module's input voltage)
//!          SCK -> PA5
//!         MOSI -> PA7
//!           DC -> PA2
//!           CS -> PA1 (optional, connect to ground on module if unused)
//! ```
//!
//! Run on a Blue Pill with `cargo run --example image`.

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::{image::Image, pixelcolor::BinaryColor, prelude::*};
use embedded_hal::spi;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32;
use sh1106::prelude::*;
use sh1106::Builder;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let dc = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let cs = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        spi::MODE_0,
        400.khz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut disp: GraphicsMode<_> = Builder::new().with_spi_cs(cs).connect_spi(spi, dc).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let im: Image<BinaryColor> =
        Image::new(include_bytes!("./rust.raw"), 64, 64).translate(Point::new(32, 0));

    disp.draw(im.into_iter());

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
