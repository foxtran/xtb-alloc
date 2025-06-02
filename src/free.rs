use libc::{RTLD_NEXT, dlsym};
use std::ffi::c_void;
use std::sync::LazyLock;

type FreeFn = unsafe extern "C" fn(*mut c_void);

pub static REAL_FREE: LazyLock<FreeFn> = LazyLock::new(|| unsafe {
    let sym = dlsym(RTLD_NEXT, b"free\0".as_ptr() as *const _);
    std::mem::transmute(sym)
});

#[unsafe(no_mangle)]
pub extern "C" fn free(data: *mut c_void) {
    if data.is_null() {
        return;
    }
    crate::storage::move_to_freed(data);
}
