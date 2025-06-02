use std::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    if let Some(data) = crate::storage::get_allocated(size) {
        return data;
    }

    let data = malloc(size);
    if !data.is_null() {
        crate::storage::add_to_allocated(data, size);
    }

    data
}
