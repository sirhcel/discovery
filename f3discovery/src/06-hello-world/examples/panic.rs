#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux6::entry;

#[entry]
fn main() -> ! {
    panic!("Hello, world!");
}
