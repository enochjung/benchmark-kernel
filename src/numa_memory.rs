use std::os::raw::{c_int, c_void};

#[link(name = "numa")]
unsafe extern "C" {
    fn numa_available() -> c_int;
    fn numa_alloc_onnode(size: usize, node: c_int) -> *mut c_void;
    fn numa_free(start: *mut c_void, size: usize);
}

pub struct NumaMemory<T> {
    ptr: *mut T,
    size: usize,
}

impl<T> NumaMemory<T> {
    pub fn alloc_on_node(len: usize, node: i32) -> Result<Self, &'static str> {
        unsafe {
            if numa_available() < 0 {
                return Err("numa not supported");
            }
            let size = size_of::<T>() * len;
            let ptr = numa_alloc_onnode(size, node) as *mut T;
            if ptr.is_null() {
                return Err("failed to allocate numa memory");
            }
            Ok(Self { ptr, size })
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}

impl<T> Drop for NumaMemory<T> {
    fn drop(&mut self) {
        unsafe { numa_free(self.ptr as *mut c_void, self.size) };
    }
}
