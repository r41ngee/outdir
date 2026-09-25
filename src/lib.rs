#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! outdir {
    ($x: literal) => {
        include!(concat!(env!("OUT_DIR"), $x))
    };
}

#[macro_export]
macro_rules! outdir_bytes {
    ($x: literal) => {
        include_bytes!(concat!(env!("OUT_DIR"), $x))
    };
}

#[macro_export]
macro_rules! outdir_str {
    ($x: literal) => {
        include_str!(concat!(env!("OUT_DIR"), $x))
    };
}