#![feature(trivial_bounds)]

use typed_cfg::*;

cfgs!(feature = { "foo", "bar" });

fn frobnicate() where feature: Is<"foo"> {
    barnicate();
}

fn barnicate() where feature: Is<"bar"> {
    // ...
}

// A function that only works when the feature `foo` is enabled

fn do_foo() where feature: Is<"foo"> {
    println!("Feature `foo` is enabled!");

    do_bar();
}

// A function that only works when the feature `bar` is enabled

fn do_bar() where feature: Is<"bar"> {
    println!("Feature `bar` is enabled!");
}

// Some functions that only work on UNIX-like targets

fn do_unix_thing() where target_family: Is<"unix"> {
    do_other_unix_thing();
}

fn do_other_unix_thing() where target_family: Is<"unix"> {}

// Some functions that only work on Windows targets

fn do_windows_thing() where target_family: Is<"windows">, (): {
    do_other_windows_thing();
}

fn do_other_windows_thing() where target_family: Is<"windows"> {}

fn main() {
    // Error: We're not always running on UNIX here!
    do_unix_thing();

    // Error: `foo` is not always enabled here!
    do_foo();
}
