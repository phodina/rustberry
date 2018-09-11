#![no_std]
#![no_main]

use core::panic::PanicInfo;

fn main () {}

pub extern "C" fn _start() -> !{

  main();
  loop{}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
