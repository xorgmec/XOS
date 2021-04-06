#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

/// Called on panic, causes abort
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

/// Entry point that starts freestanding binary
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "Devdutt is in control of this terminal ₹, here are {} and {}",
        69,
        4.2 / 6.9
    );

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
