use core::{panic::PanicInfo, sync::atomic::Ordering};

use uefi::println;

use crate::firmware::u_efi::BOOT_SERVICES_EXITED;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if !(BOOT_SERVICES_EXITED.load(Ordering::SeqCst)) {
        match info.location() {
            Some(l) => println!(
                "Panicked '{}', {}:{}:{}",
                info.message(),
                l.file(),
                l.line(),
                l.column(),
            ),
            None => println!(
                "Panicked '{}', Unable to get location information",
                info.message()
            ),
        }
    }
    loop {}
}
