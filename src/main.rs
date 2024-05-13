#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::CStr;
use flip_ui::flip_ui;
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Flipper Zero Rust",
    app_version = 1,
    has_icon = false,
);

// Define the entry function
entry!(main);

// Getting UI data && events
flip_ui! {
    App,
    "src/main.json",
    next => next,
    close => close,
    back => back,
}

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    let mut app = App::create();

    app.show();

    0
}
