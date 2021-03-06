#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod txtjob;

use txtjob::TxtJobApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = TxtJobApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}



