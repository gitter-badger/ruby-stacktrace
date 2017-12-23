#[macro_use]
extern crate log;

extern crate regex;
extern crate libc;
extern crate ruby_stacktrace;
extern crate byteorder;
extern crate clap;
extern crate env_logger;
extern crate read_process_memory;

use clap::{Arg, App, ArgMatches};
use libc::*;
use std::process;
use std::time::Duration;
use std::thread;

use ruby_stacktrace::*;
use bindings::ruby_2_2_0::*;

pub fn get_stack_trace() -> u64 {
    let cfps = get_cfps();
    3
}

fn get_cfps() -> Vec<rb_control_frame_struct> {
    let mut ret = vec![0, 0, 0, 0, 0, 0, 0, 0, 200, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 248, 234, 192, 160, 0, 86, 0, 0, 192, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 249, 207, 160, 0, 86, 0, 0, 48, 67, 220, 160, 0, 86, 0, 0, 168, 208, 47, 222, 94, 127, 0, 0, 128, 248, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 160, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 93, 220, 160, 0, 86, 0, 0, 136, 50, 210, 160, 0, 86, 0, 0, 144, 208, 47, 222, 94, 127, 0, 0, 144, 249, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 136, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 94, 220, 160, 0, 86, 0, 0, 40, 71, 220, 160, 0, 86, 0, 0, 120, 208, 47, 222, 94, 127, 0, 0, 160, 250, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 112, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 94, 220, 160, 0, 86, 0, 0, 88, 74, 220, 160, 0, 86, 0, 0, 96, 208, 47, 222, 94, 127, 0, 0, 176, 251, 219, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 88, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 94, 220, 160, 0, 86, 0, 0, 216, 80, 220, 160, 0, 86, 0, 0, 72, 208, 47, 222, 94, 127, 0, 0, 112, 77, 220, 160, 0, 86, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 96, 236, 192, 160, 0, 86, 0, 0, 64, 208, 47, 222, 94, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 95, 220, 160, 0, 86, 0, 0, 0, 84, 220, 160, 0, 86, 0, 0, 48, 208, 47, 222, 94, 127, 0, 0, 0, 244, 219, 160, 0, 86, 0, 0, 145, 2, 0, 0, 0, 0, 0, 0, 112, 220, 192, 160, 0, 86, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 160, 93, 220, 160, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let p = ret.as_mut_ptr();
    let cap = ret.capacity();

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
    env_logger::init().unwrap();

    let matches = parse_args();
    let pid: pid_t = matches.value_of("PID").unwrap().parse().unwrap();
    let command = matches.value_of("COMMAND").unwrap();
    let source = pid.try_into_process_handle().unwrap();
    if command.clone() != "top" && command.clone() != "stackcollapse" &&
        command.clone() != "parse"
        {
            println!("COMMAND must be 'top' or 'stackcollapse. Try again!");
            process::exit(1);
        }


    let ruby_current_thread_address_location: u64 = get_ruby_current_thread_address(pid);

    if command == "stackcollapse" {
        // This gets a stack trace and then just prints it out
        // in a format that Brendan Gregg's stackcollapse.pl script understands
        loop {
            let trace = stack_trace::get_stack_trace(ruby_current_thread_address_location, &source);
            debug!("before print");
            println!("{:?}", trace);
            thread::sleep(Duration::from_millis(100));
            debug!("done sleeping");
        }
    }
}
