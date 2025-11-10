use core::panic::PanicInfo;

#[cfg(feature = "uefi")]
pub use uefi::{print, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "PANIC at line \"{}\", in file \"{}\"",
            location.file(),
            location.line(),
        );
    } else {
        println!("PANIC?: {info}");
    }

    loop {}
}
