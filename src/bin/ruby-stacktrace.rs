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
pub struct blah {
    pub byte: [::std::os::raw::c_char; 80usize],
}

fn get_cfps() -> Vec<blah> {
    let mut ret = Vec::with_capacity(560);
    for i in 0..560 {
        ret.push(i);
    }

    let p = ret.as_mut_ptr();
    let cap = ret.capacity();

    // mem::size_of of blah is 80
    // so make a 7-element vector of blahs instead
    let rebuilt: Vec<blah> = unsafe { 
        mem::forget(ret);
        Vec::from_raw_parts(
            p as *mut blah,
            7,
            560,
            )
    };

    rebuilt
}

fn main() {
    println!("size of blah: {}", mem::size_of::<blah >());
    for i in 0..10 {
        println!("attempt {}", i);
        get_stack_trace();
    }
}
