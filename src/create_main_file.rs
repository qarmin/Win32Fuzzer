use crate::DISABLED_CLASSES;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

pub fn create_main_file(things: &[(&str, String, Vec<&str>)]) {
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
#![allow(non_snake_case)]

mod basic_data;
mod more_bad_data;

use std::fs::{File, OpenOptions};
use crate::basic_data::SettingsTaker;
use crate::basic_data::read_from_file;
"###;

    // <<number_of_functions>> - number of functions
    // <<functions>> -
    let main_start = r###"
fn main() {
    let st = read_from_file();

    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("results.txt").unwrap();
    let all_classes: [(fn(&mut File, &SettingsTaker) -> (), &str); <<number_of_functions>>] = [<<functions>>];


    for _i in 0..st.all_repeating_number {
        if st.allowed_classes.is_empty() {
            for (function, name) in all_classes {
                if !st.ignored_classes.contains(&name.to_string()) {
                    function(&mut file, &st);
                }
            }
        } else {
            for (function, name) in all_classes {
                if st.allowed_classes.contains(&name.to_string()) {
                    function(&mut file, &st);
                }
            }
        }
    }
    "###;

    let mut mod_to_add = "".to_string();
    let mut use_crate_to_add = "".to_string();
    let mut function_execution = "".to_string();
    let mut number_of_functions = 0;
    for (class_name, _, _) in things {
        if DISABLED_CLASSES.contains(class_name) {
            continue;
        }
        mod_to_add += &format!("mod z_{};\n", class_name.to_lowercase());
        use_crate_to_add += &format!("use crate::z_{}::*;\n", class_name.to_lowercase());
        function_execution += &format!("(z_{},\"{}\"),", class_name.to_lowercase(), class_name);
        number_of_functions += 1;
    }
    if function_execution.ends_with(',') {
        function_execution.pop();
    }

    let main_start = main_start
        .replace("<<functions>>", &function_execution)
        .replace("<<number_of_functions>>", &number_of_functions.to_string());

    writeln!(file, "{}", allowed_bad_things).unwrap();
    writeln!(file, "{}", mod_to_add).unwrap();
    writeln!(file, "{}", use_crate_to_add).unwrap();

    writeln!(file, "{}", main_start).unwrap();
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
