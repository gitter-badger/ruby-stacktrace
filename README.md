<a href="https://travis-ci.org/jvns/ruby-stacktrace"><img src="https://travis-ci.org/jvns/ruby-stacktrace.svg"></a>

# rbspy

Have you ever wanted to know what functions Ruby program is doing? `rbspy` can tell you!

**this is alpha software**.

## Requirements

1. Linux (OS X support is planned)
2. The most recent pre-release of `rbspy` (download from [the github releases page](https://github.com/jvns/ruby-stacktrace/releases))

rbspy supports Ruby 1.9.1 to 2.5.0.

## How to get rbspy

1. Download recent release of `rbspy` (download from [the github releases page](https://github.com/jvns/ruby-stacktrace/releases))
2. Unpack it
3. Move the `rbspy` binary to `/usr/local/bin`

## Using rbspy

rbspy currently has 2 features: snapshot and record.

**Snapshot**

Snapshot takes a single stack trace from the specified process and exits. 

```
rbspy snapshot --pid $PID
```

**Record**

Record records stack traces from your process for displaying as a flamegraph

```
rbspy record --file stacks.txt --pid $PID
rbspy record --file stacks.txt ruby myprogram.rb
```

## Generating flamegraphs

You can use this tool to generate flamegraphs for a running Ruby process. 

1. Get the [FlameGraph repository](https://github.com/brendangregg/FlameGraph) and add it to your PATH
1. run `stackcollapse.pl < stacks | flamegraph.pl > output.svg`
1. Open output.svg! You should get a beautiful graph like this: (click
   to enlarge)

<a href="http://jvns.ca/images/sampling.png"><img src="http://jvns.ca/images/sampling.png" width="400px"></a>

## How it works

## Missing features

* Mac support 


## Developing ruby-stacktrace

It's written in Rust.

1. Install cargo from [crates.io](https://crates.io/)
1. `cargo build` to build
1. `cargo test` to test

The build artifacts will end up in `target/debug`

## Authors

* Julia Evans
* Kamal Marhubi
