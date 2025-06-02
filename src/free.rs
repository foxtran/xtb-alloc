use std::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn free(data: *mut c_void) {
    if data.is_null() {
        return;
    }
    crate::storage::move_to_freed(data);
}
