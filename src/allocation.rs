extern {
    fn malloc(size: usize) -> *mut u8;
    fn calloc(count: usize, size: usize) -> *mut u8;
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[allocator]
#[allow(unused_variables)]
#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe { malloc(size) as *mut u8 }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { free(ptr) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                                align: usize) -> *mut u8 {
    unsafe { realloc(ptr, size) as *mut u8 }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn __rust_reallocate_inplace(ptr: *mut u8, old_size: usize,
                                        size: usize, align: usize) -> usize {
    old_size
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
    size
}
