# rust-debug

Simple log-to-file library for debugging Rust 0apps.


## Why Use This?

Regardless of the fact that logging to a file with precise timestamps can generally be more practical and useful than printing to the screen...

There are times when you need to debug issues:
- that only manifest during runtime.
- when access to a debugger isn't available.
- when halting the process isn't feasible.
- when the stdout/stderr isn't visible.
- when dealing with a plug-in or a child process.

In such scenarios, logging to a file can be invaluable.

This repository offers a straightforward logger, complete with detailed timestamps, enabling quick setup for file logging.

This library can also be used for simple terminal printing.  
It adds the following features:
- a global switch to disable all debug_log() calls.
- timestamp to the output (can be easily disabled).

<div style="text-align: right;">
    <a href="#rust-debug">Back to top</a>
</div>


## Usage

1. Copy `debug.rs` into the src folder of your rust project.
2. Insert `mod debug` in the `main.rs` file of your project.
3. Insert `use crate::debug` in every file you need to access logging functions.
4. At the start of your main, set the path, set some flags and call `debug::enable()` like so:
```rust
// pass the executable path
debug::set_path(std::env::current_exe().unwrap());

// set some flags
debug::file_output_enable();
debug::file_timestamp_enable();

// globally enable logging
debug::enable();
```
5. Every time you need to log something, call `debug::log!(SOMETHING);` just like you would call `println!(SOMETHING);`.
5. When you have finished debugging, comment or delete call to enable() like this `// debug::enable();` to disable logging.
    - No need to comment or delete any `debug::log();` line. Without `debug::enable();`, they will do nothing.

<div style="text-align: right;">
    <a href="#rust-debug">Back to top</a>
</div>


## FLAGS options

There are some flags you can use to customize how `debug::log()` works.


### debug::terminal_output_enable()

Why forgo terminal output while logging to file?  
Call this in order to see everything you log to file also on the process stdout.
Do you want to disable this at some point?  
Simply comment or delete this call.


### debug::terminal_timestamp_enable()

If you want timestamps for every line in your terminal, call this.


### debug::file_output_enable()

Call this to enable file logging.  
Need to disable logging to file?  
Just don't call it.

### debug::file_timestamp_enable()

If you want timestamps for every line in your file, call this.


### debug::file_history_enable

**default mode**: only one file is written and it is overwritten over and over everytime the executable executes.
```
root
├── your_executable.exe
└── debug
    └── your_executable.exe.log
```

**debug::file_history_enable();**: it generates a new timestamped file each time the executable is executed. 
```
root
root
├── your_executable.exe
└── debug
    ├── 20240531_1234_000_your_executable.exe.log
    ├── 20240531_1234_123_your_executable.exe.log
    ├── 20240531_1256_012_your_executable.exe.log
    └── 20240531_1345_987_your_executable.exe.log
```

<div style="text-align: right;">
    <a href="#rust-debug">Back to top</a>
</div>


## Testing and compiling the example

- Check `main.rs` and `example.rs`.
- Execute the example with `cargo run`.
- Check the log file that has been written in `target/debug/debug/` folder.
- Experiment with the `example.rs` file (disable debugging, change FLAGS, log other types of data, etc.).

<div style="text-align: right;">
    <a href="#rust-debug">Back to top</a>
</div>


## Links

- [Debug logger for Python.](https://github.com/RobertoPorpora/python-debug)
- [Debug logger for C/C++.](https://github.com/RobertoPorpora/c-debug)
- [Debug logger for Node.js.](https://github.com/RobertoPorpora/node-debug)
- [Debug logger for Rust.](https://github.com/RobertoPorpora/rust-debug)


<div style="text-align: right;">
    <a href="#rust-debug">Back to top</a>
</div>
