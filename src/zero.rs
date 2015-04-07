
#[no_stack_check]
#[no_mangle]
pub extern fn abort() -> ! {
    loop {}
}

#[no_stack_check]
#[no_mangle]
pub extern fn get_eit_entry() {
    abort();
}

#[no_stack_check]
#[no_mangle]
pub extern fn __aeabi_memset(dest: *mut u8, size: usize, value: u32) {
  unsafe {
    use core::intrinsics::volatile_set_memory;
    volatile_set_memory(dest, value as u8, size);
  }
}

#[doc(hidden)]
#[no_stack_check]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
  abort();
}

#[doc(hidden)]
#[no_stack_check]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {
  abort();
}

#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}  
#[lang="panic_fmt"] pub fn panic_fmt(_fmt: &::core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! { 
    loop { } 
}
