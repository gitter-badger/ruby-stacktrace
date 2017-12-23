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
use std::collections::HashMap;
use std::collections::HashSet;
use read_process_memory::*;

use ruby_stacktrace::*;

fn parse_args() -> ArgMatches<'static> {
    App::new("ruby-stacktrace")
        .version("0.1")
        .about("Sampling profiler for Ruby programs")
        .arg(
            Arg::with_name("COMMAND")
                .help(
                    "Subcommand you want to run. Options: top, stackcollapse.\n          top \
                   prints a top-like output of what the Ruby process is doing right now\n          \
                   stackcollapse prints out output suitable for piping to stackcollapse.pl \
                   (https://github.com/brendangregg/FlameGraph)",
                )
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PID")
                .help("PID of the Ruby process you want to profile")
                .required(true)
                .index(2),
        )
        .get_matches()
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
