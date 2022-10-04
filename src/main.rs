// src/main.rs

#![no_std] // don't link Rust std lib
#![no_main] // disable all Rust-level entry points

mod vga;

use core::panic::PanicInfo;

// this fn is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    println!( "Hello world! {} {}\n", 2, 3.4);
    println!("Hello world! {} {}", 2, 3.4);
    panic!("some panic msg");

    loop {}

}
