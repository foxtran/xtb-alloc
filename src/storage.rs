use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct DataPtr(*mut c_void);

unsafe impl Send for DataPtr {}
unsafe impl Sync for DataPtr {}

static ALLOCATED: Lazy<Mutex<HashMap<DataPtr, usize>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static FREED: Lazy<Mutex<HashMap<usize, Vec<DataPtr>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_allocated(block_size: usize) -> Option<*mut c_void> {
    let mut freed = FREED.lock().unwrap();
    let mut allocated = ALLOCATED.lock().unwrap();

    if let Some(vec) = freed.get_mut(&block_size) {
        if let Some(ptr) = vec.pop() {
            allocated.insert(ptr, block_size);
            return Some(ptr.0);
        }
    }

    None
}

pub fn move_to_freed(data: *mut c_void) {
    let data_ptr = DataPtr(data);

    let mut allocated = ALLOCATED.lock().unwrap();
    if let Some(block_size) = allocated.remove(&data_ptr) {
        drop(allocated);

        let mut freed = FREED.lock().unwrap();
        freed.entry(block_size).or_default().push(data_ptr);
    }
}

pub fn add_to_allocated(data: *mut c_void, block_size: usize) {
    let data_ptr = DataPtr(data);
    let mut allocated = ALLOCATED.lock().unwrap();
    allocated.insert(data_ptr, block_size);
}
