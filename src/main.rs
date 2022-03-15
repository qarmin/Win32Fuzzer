#![allow(clippy::type_complexity)]
#![allow(unused_imports)]
#![allow(dead_code)]
mod automatic_type_renames;
mod create_main_file;
mod create_project_file;
mod find_things;
mod generate_reproducer_file;
mod parse_file;
mod settings;
mod sort_settings;

use crate::automatic_type_renames::ADVANCED_RENAMES;
use crate::find_things::*;
use crate::parse_file::*;
use crate::settings::*;
use crate::sort_settings::*;
use crate::TypeOfProblem::InvalidNumberOfArguments;
use create_main_file::*;
use create_project_file::*;
use generate_reproducer_file::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!(
            "You need to set path to windows-rs library(https://github.com/microsoft/windows-rs) - 0.33.0 is supported at this moment(example run)"
        );
        return;
    }
    let mut path = args[1].clone();
    if !path.ends_with('/') {
        path.push('/');
    }

    if !Path::new(&path).exists() {
        println!(
            "Provided path {} not exists, please provide proper path to windows-rs library(https://github.com/microsoft/windows-rs)",
            path
        );
    }

    set_windows_rs_folder(path);

    let things = load_settings();
    // sort_settings(&things);

    let create_renames2 = [
        // Basic
        ("u32", "take_u32"),
        ("usize", "take_usize"),
        ("u8", "take_u8"),
        ("u16", "take_u16"),
        ("i16", "take_i16"),
        ("u64", "take_u64"),
        ("f32", "take_f32"),
        ("f64", "take_f64"),
        ("i32", "take_i32"),
        ("i8", "take_i8"),
        ("i64", "take_i64"),
        ("isize", "take_isize"),
        ("char", "take_char"),
        ("string", "take_string"),
    ];

    let mut create_renames: HashMap<&str, &str> = HashMap::new();
    for i in create_renames2 {
        create_renames.insert(i.0, i.1);
    }

    let mut advanced_renames: HashMap<&str, String> = Default::default();
    for i in ADVANCED_RENAMES {
        advanced_renames.insert(*i, format!("get_strange_{}", i));
    }

    find_things(&things);
    create_main_file(&things);

    let mut ignored_arguments: BTreeMap<String, u32> = Default::default(); // List of ignored arguments

    for entry in WalkDir::new("WinProject") {
        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().to_string();
        if path.contains("z_") {
            // println!("Path to remove {}", path);
            let _ = fs::remove_file(path);
        }
    }

    let mut number_of_parsed_classes = 0;
    let mut number_of_parsed_functions = 0;
    let mut number_of_parsed_arguments = 0;
    let mut using_functions = 0;
    let mut functions_classes: BTreeMap<String, Vec<String>> = Default::default();
    for (class_name, path, exceptions) in things {
        if DISABLED_CLASSES.contains(&class_name) {
            continue;
        }
        let exceptions: HashSet<_> = exceptions.iter().map(|e| e.0.to_string()).collect();
        let mut file_data = FileData::new();
        parse_file(&mut file_data, &path);
        {
            number_of_parsed_classes += 1;
            number_of_parsed_functions += file_data.functions.len();
            for arguments in file_data.functions.values() {
                number_of_parsed_arguments += arguments.len();
            }
        }
        let used_functions = create_project_file(
            &file_data,
            class_name,
            &create_renames,
            &advanced_renames,
            &exceptions,
            &mut ignored_arguments,
        );
        using_functions += used_functions.len();
        functions_classes.insert(class_name.to_string(), used_functions);
    }

    generate_reproducer_file(functions_classes);

    println!(
        "Parsed - Classes: \"{}\", Functions: \"{}\", Arguments: \"{}\"",
        number_of_parsed_classes, number_of_parsed_functions, number_of_parsed_arguments
    );
    println!("Using - Functions: \"{}\"", using_functions);

    {
        let mut new_vec: Vec<(u32, String)> = Default::default();
        for (key, value) in ignored_arguments {
            new_vec.push((value, key));
        }
        new_vec.sort_by(|e, f| if e.0 > f.0 { Ordering::Greater } else { Ordering::Less });
        for (key, value) in new_vec {
            println!("Not supported '''{}''', found {} occurences", value, key);
        }
    }
}

pub fn print_excluded_things(things: &[(&'static str, String, Vec<(&'static str, TypeOfProblem)>)]) {
    let mut things = things.to_vec();
    things.sort_by(|e, f| if e.0 > f.0 { Ordering::Greater } else { Ordering::Less });
    for (class, _path, disallowed_functions) in things {
        if disallowed_functions.iter().filter(|e| e.1 != InvalidNumberOfArguments).count() == 0 {
            continue;
        }
        println!("{}", class);

        let mut disallowed_functions = disallowed_functions.clone();
        disallowed_functions.sort_by(|e, f| if e.1 > f.1 { Ordering::Less } else { Ordering::Greater });
        for (function, problem) in disallowed_functions {
            let pro = match problem {
                TypeOfProblem::CrashesLinux => "Crashes in Linux",
                TypeOfProblem::CrashesWindows => "Crashes in Windows",
                TypeOfProblem::NotImplementedLinux => "Not implemented in Wine Staging 7.3",
                TypeOfProblem::NotImplementedWindows => "Not implemented in Windows 10 20H2",
                TypeOfProblem::ShowsDialogWindows => "Shows dialog in Windows(stop execution until user close it)",
                TypeOfProblem::ShowsDialogLinux => "Shows dialog in Linux(stop execution until user close it)",
                TypeOfProblem::CrashAutomatic => "Functions which close app(this is expected behaviour)",
                TypeOfProblem::Freeze => "Freeze app",
                TypeOfProblem::Other => "Other",
                TypeOfProblem::InvalidNumberOfArguments => {
                    continue; //"Not checked function(not Wine bug, just problem with parsing)"
                }
            };
            println!("\t{}   -   {}", function, pro);
        }
    }
}
