#[macro_export]
macro_rules! write {
    ($path: literal, $content: expr) => {{
        use std::io::Write;
        use std::fs::File;
        let mut __file = File::create($crate::build_path!($path)).unwrap();
        __file.write_all($content).unwrap()
    }};
}

#[macro_export]
macro_rules! build_path {
    ($path: literal) => {{
        std::env::var("OUT_DIR").unwrap() + "/" + $path
    }};
}