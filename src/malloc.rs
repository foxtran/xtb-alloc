use libc::{RTLD_NEXT, dlsym};
use std::ffi::c_void;
use std::sync::LazyLock;

type MallocFn = unsafe extern "C" fn(usize) -> *mut c_void;

pub static REAL_MALLOC: LazyLock<MallocFn> = LazyLock::new(|| unsafe {
    let sym = dlsym(RTLD_NEXT, b"malloc\0".as_ptr() as *const _);
    std::mem::transmute(sym)
});

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    if let Some(data) = crate::storage::get_allocated(size) {
        return data;
    }

    let data = unsafe { (REAL_MALLOC)(size) };
    if !data.is_null() {
        crate::storage::add_to_allocated(data, size);
    }

    data
}
