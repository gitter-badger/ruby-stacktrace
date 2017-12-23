// #![cfg_attr(rustc_nightly, feature(test))]
// #![feature(alloc_system)]
// 
// #![feature(alloc_system)]
// #![feature(global_allocator, allocator_api)]
// 
// extern crate alloc_system;
// 
// use alloc_system::System;
// 
// #[global_allocator]
// static A: System = System;

use std::mem;

fn get_stack_trace() -> u64 {
    get_cfps();
    3
}

#[repr(C)]
pub struct size_80_struct {
    pub byte: [::std::os::raw::c_char; 80usize],
}

fn get_cfps() -> Vec<size_80_struct> {
    let mut ret = Vec::with_capacity(560);
    for i in 0..560 {
        ret.push(i);
    }

    let p = ret.as_mut_ptr();

    // so make a 7-element vector of size_80_structs instead
    let rebuilt: Vec<size_80_struct> = unsafe { 
        mem::forget(ret);
        Vec::from_raw_parts(
            p as *mut size_80_struct,
            7,
            560,
            )
    };

    rebuilt
}

fn main() {
    for i in 0..10 {
        println!("attempt {}", i);
        get_stack_trace();
    }
}
