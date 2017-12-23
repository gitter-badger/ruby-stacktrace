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

extern crate regex;
extern crate libc;
extern crate byteorder;
extern crate env_logger;
extern crate read_process_memory;

pub mod bindings;

use std::time::Duration;
use std::thread;
use std::mem;

use bindings::ruby_2_2_0::*;

pub fn get_stack_trace() -> u64 {
    get_cfps();
    3
}

fn get_cfps() -> Vec<rb_control_frame_struct> {
    let mut ret = vec![0, 0, 0, 0, 0, 0, 0, 0, 200, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 248, 234, 192, 160, 0, 86, 0, 0, 192, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 249, 207, 160, 0, 86, 0, 0, 48, 67, 220, 160, 0, 86, 0, 0, 168, 208, 47, 222, 94, 127, 0, 0, 128, 248, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 160, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 93, 220, 160, 0, 86, 0, 0, 136, 50, 210, 160, 0, 86, 0, 0, 144, 208, 47, 222, 94, 127, 0, 0, 144, 249, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 136, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 94, 220, 160, 0, 86, 0, 0, 40, 71, 220, 160, 0, 86, 0, 0, 120, 208, 47, 222, 94, 127, 0, 0, 160, 250, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 112, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 94, 220, 160, 0, 86, 0, 0, 88, 74, 220, 160, 0, 86, 0, 0, 96, 208, 47, 222, 94, 127, 0, 0, 176, 251, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 88, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 94, 220, 160, 0, 86, 0, 0, 216, 80, 220, 160, 0, 86, 0, 0, 72, 208, 47, 222, 94, 127, 0, 0, 112, 77, 220, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 64, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 95, 220, 160, 0, 86, 0, 0, 0, 84, 220, 160, 0, 86, 0, 0, 48, 208, 47, 222, 94, 127, 0, 0, 0, 244, 219, 160, 0, 86, 0, 0, 145, 2, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 160, 93, 220, 160, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let p = ret.as_mut_ptr();
    let cap = ret.capacity();
    let cfp_size = mem::size_of::<rb_control_frame_struct>() as u64;

    let rebuilt: Vec<rb_control_frame_struct> = unsafe {
        // Cast `v` into the void: no destructor run, so we are in
        // complete control of the allocation to which `p` points.
        // Put everything back together into a Vec
        mem::forget(ret);
        debug!("unsafe p: {:x}, len: {}, cap: {}, cfp_size: {}", p as u64, cap/(cfp_size as usize), cap, cfp_size);
        Vec::from_raw_parts(
            p as *mut rb_control_frame_struct,
            cap / (cfp_size as usize),
            cap,
            )
    };

    rebuilt
}

fn main() {
    loop {
        get_stack_trace();
        debug!("before print");
        thread::sleep(Duration::from_millis(100));
        debug!("done sleeping");
    }
}
