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

#[macro_use]
extern crate log;

pub mod bindings;

use std::mem;

use bindings::ruby_2_2_0::*;

pub fn get_stack_trace() -> u64 {
    get_cfps();
    3
}

fn get_cfps() -> Vec<rb_control_frame_struct> {
    let mut ret = Vec::with_capacity(560);
    for i in 0..560 {
        ret.push(i);
    }

    let p = ret.as_mut_ptr();
    let cap = ret.capacity();

    // mem::size_of of rb_control_frame_struct is 80
    // so make a 7-element vector of rb_control_frame_structs instead
    let rebuilt: Vec<rb_control_frame_struct> = unsafe { 
        mem::forget(ret);
        Vec::from_raw_parts(
            p as *mut rb_control_frame_struct,
            7,
            560,
            )
    };

    rebuilt
}

fn main() {
    println!("size of rb_control_frame_struct: {}", mem::size_of::<rb_control_frame_struct >());
    for i in 0..10 {
        println!("attempt {}", i);
        get_stack_trace();
    }
}
