use crate::DISABLED_CLASSES;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

pub fn create_main_file(things: &Vec<(&str, String, Vec<&str>)>) {
    let file_name = "WinProject/src/main.rs";

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_name).unwrap();
    let mut file = BufWriter::new(file);

    let allowed_bad_things = r###"
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]
#![allow(unused_must_use)]
#![allow(dead_code)]

mod basic_data;
mod more_bad_data;
"###;
    let mut mod_to_add = "".to_string();
    let mut use_crate_to_add = "".to_string();
    let mut function_execution = "".to_string();
    for (class_name, _, _) in things {
        if DISABLED_CLASSES.contains(class_name) {
            continue;
        }
        mod_to_add += &format!("mod z_{};\n", class_name.to_lowercase());
        use_crate_to_add += &format!("use crate::z_{}::*;\n", class_name.to_lowercase());
        function_execution += &format!("z_{}();\n", class_name.to_lowercase());
    }
    writeln!(file, "{}", allowed_bad_things).unwrap();
    writeln!(file, "{}", mod_to_add).unwrap();
    writeln!(file, "{}", use_crate_to_add).unwrap();

    writeln!(file, "fn main() {{").unwrap();

    writeln!(file, "{}", function_execution).unwrap();
    writeln!(file, "}}").unwrap();
}

// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_unsafe)]
// #![allow(unused_must_use)]
// #![allow(dead_code)]
//
// mod basic_data;
// mod more_bad_data;
// mod z_printer;
// mod z_gaming;
//
// use crate::z_printer::*;
// use crate::z_gaming::*;
//
// fn main() {
//     z_gaming();
//     z_printer();
// }
