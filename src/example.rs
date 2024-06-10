use crate::debug;

pub fn test() {
    // starting the logger.

    // pass the executable path
    match std::env::current_exe() {
        Ok(path_buf) => debug::set_path(path_buf),
        _ => {}
    }

    // set some flags
    debug::file_output_enable();
    debug::file_timestamp_enable();
    // debug::file_history_enable();
    // debug::terminal_output_enable();
    // debug::terminal_timestamp_enable();

    // globally enable logging
    debug::enable();

    // logging arguments
    for (i, arg) in std::env::args().enumerate() {
        debug::log!("received argument: {} => {}", i, arg);
    }

    // some random data
    let alice = Person {
        name: String::from("Alice"),
        age: 25,
        height: 1.65,
    };

    let bob = Person {
        name: String::from("Bob"),
        age: 30,
        height: 1.80,
    };

    // log data
    debug::log!(
        "Person 1 => Name: {}, Age: {}, Height: {:.2}",
        alice.name,
        alice.age,
        alice.height,
    );
    debug::log!(
        "Person 2 => Name: {}, Age: {}, Height: {:.2}",
        bob.name,
        bob.age,
        bob.height,
    );

    // end
}

//------------------------------------------------------------------------------

struct Person {
    name: String,
    age: i32,
    height: f32,
}
