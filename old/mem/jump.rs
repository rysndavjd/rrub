use core::ptr::copy_nonoverlapping;

unsafe extern "C" {
    unsafe static relocator_start: u8;
    unsafe static relocator_end: u8;

    unsafe static mut target_rax: u64;
    unsafe static mut target_rbx: u64;
    unsafe static mut target_rcx: u64;
    unsafe static mut target_rdx: u64;
    unsafe static mut target_rsi: u64;
    unsafe static mut target_rip: u64;
}

pub struct Relocator {
    start: *const u8,
    size: usize,
}

impl Relocator {
    fn get() -> Relocator {
        unsafe {
            let start = relocator_start as *const u8;
            let size = (relocator_end as *const u8).offset_from(start) as usize;
            return Relocator { start, size };
        }
    }

    unsafe fn copy_to(&self, dst: *mut u8) {
        unsafe {
            copy_nonoverlapping(self.start, dst, self.size);
        }
    }

    fn set_registors(&self, rax: u64, rbx: u64, rcx: u64, rdx: u64, rip: u64, rsi: u64) {
        unsafe {
            target_rax = rax;
            target_rbx = rbx;
            target_rcx = rcx;
            target_rdx = rdx;
            target_rip = rip;
            target_rsi = rsi;
        }
    }

    fn jump(self, ) {
        
    }
}
