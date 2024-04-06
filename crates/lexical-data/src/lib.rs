#![no_std]
#![forbid(unsafe_code)]

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/wordlists.rs"));
