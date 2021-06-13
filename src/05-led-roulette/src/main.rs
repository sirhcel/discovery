#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::entry;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    defmt::info!("Greetrings from 05-led-roulette!");

    // Gracefully terminate the application and make `probe-run` exit.
    aux5::exit();
}
