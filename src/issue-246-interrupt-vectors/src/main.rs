#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 40;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
