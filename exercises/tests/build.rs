use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Set the timestamp for TEST_FOO (for the previous exercise)
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Check if we're building tests8 (this exercise)
    if env::var("CARGO_PKG_NAME").unwrap() == "tests8" {
        // Set the "pass" feature for tests8
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }

    // Always rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}