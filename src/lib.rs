#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(compiler_builtins_lib)]

extern crate rlibc;
extern crate compiler_builtins;

use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// but raspi1 has peripheral base address 0x20000000. Ensure
// you are using the correct peripheral address for your
// hardware.
const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

#[no_mangle]
pub extern fn kernel_main() {
    write("There is no place like Rust!\r\n");
    loop {
        writec(getc())
    }
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]                                                                     
pub extern fn rust_begin_unwind(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    unsafe { abort() }                                         
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() { loop {} }

#[no_mangle]
pub extern "C" fn world(val: u32) -> u32 {
	val * val + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
