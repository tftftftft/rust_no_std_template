#![no_std]
#![no_main]

// Manually link to msvcrt
#[link(name = "msvcrt")]
extern "C" {}

extern crate alloc;
extern crate libc;
use alloc::vec::Vec;
use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

static mut value: i32 = 0;
#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Since we are passing a C string the final null character is mandatory.
    const HELLO: &'static str = "Hello, world!\n\0";
    unsafe {
        libc::printf(b"%s\0".as_ptr() as *const _, HELLO.as_ptr() as *const _);
    }
    let mut new_vec = Vec::new();
    new_vec.push(1);

    unsafe { value = new_vec[0] };
    0
}

#[cfg(not(test))]
#[panic_handler]
/// Default panic handler
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
