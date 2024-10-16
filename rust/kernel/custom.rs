use crate::bindings;
use core::ptr::addr_of_mut;

pub unsafe fn btf_range(start: *mut *mut core::ffi::c_void, end: *mut *mut core::ffi::c_void) {
    unsafe { 
        bindings::get_btf_range(start, end)
    }
}

pub unsafe fn assign_get_bpf_program_id_fn(ptr: *mut core::ffi::c_void) {
    unsafe {
        bindings::assign_function_ptr(addr_of_mut!(bindings::get_bpf_program_id_fn), ptr);
    }
}
pub unsafe fn assign_insert_bpf_type_fn(ptr: *mut core::ffi::c_void) {
    unsafe {
        bindings::assign_function_ptr(addr_of_mut!(bindings::insert_bpf_type_fn), ptr);
    }
}

pub unsafe fn assign_get_list_of_types_to_be_checked_fn(ptr: *mut core::ffi::c_void) {
    unsafe {
        bindings::assign_function_ptr(addr_of_mut!(bindings::get_list_of_types_to_be_checked_fn), ptr);
    }
}

pub unsafe fn asssign_remove_bpf_program_id_fn(ptr: *mut core::ffi::c_void) {
    unsafe {
        bindings::assign_function_ptr(addr_of_mut!(bindings::remove_bpf_program_id_fn), ptr);
    }
}