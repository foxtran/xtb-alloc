use crate::free::REAL_FREE;
use crate::malloc::REAL_MALLOC;

use std::alloc::{GlobalAlloc, Layout};
use std::ffi::c_void;

struct RustAlloc;

unsafe impl GlobalAlloc for RustAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { (REAL_MALLOC)(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { (REAL_FREE)(ptr as *mut c_void) }
    }
}

#[global_allocator]
static GLOBAL: RustAlloc = RustAlloc;
