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

use crate::automatic_type_renames::*;
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
        println!("You need to set path to windows-rs library(https://github.com/microsoft/windows-rs) - 0.33.0 is supported at this moment(example run)");
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

    let place_of_mod_rs = load_place_of_mod_rs();
    // for (class, _FF, vec) in things.clone() {
    //     for (function_name, type_of_problem) in vec {
    //         println!("(\"{}\",\"{}\",{}),", class, function_name, type_of_problem.to_string());
    //     }
    // }
    // sort_settings(&things);

    let mut basic_renames: HashMap<&str, &str> = HashMap::new();
    for i in BASIC_RENAMES {
        basic_renames.insert(i.0, i.1);
    }

    let mut advanced_renames: HashMap<&str, String> = Default::default();
    for i in ADVANCED_RENAMES {
        advanced_renames.insert(*i, format!("get_strange_{}", i));
    }

    let mut non_creatable_arguments: HashSet<&str> = Default::default();
    for i in NON_CREATABLE_ARGUMENTS {
        non_creatable_arguments.insert(*i);
    }

    let mut strange_renames: HashMap<&str, &str> = Default::default();
    for i in STRANGE_RENAMES {
        strange_renames.insert(i.0, i.1);
    }

    find_things(&place_of_mod_rs);
    create_main_file(&place_of_mod_rs);

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
    for (class_name, path) in place_of_mod_rs {
        if DISABLED_CLASSES.contains(&class_name) {
            continue;
        }
        let mut exceptions: HashMap<_, _> = Default::default();
        EXCEPTIONS
            .iter()
            .filter_map(|(cla, fun, problem)| if *cla == class_name { Some((fun, problem)) } else { None })
            .for_each(|(e, problem)| {
                exceptions.insert(e.to_string(), problem.clone());
            });
        let mut file_data = FileData::new();
        parse_file(&mut file_data, &format!("{}{}", get_windows_rs_folder(), path));
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
            &basic_renames,
            &advanced_renames,
            &exceptions,
            &mut ignored_arguments,
            &non_creatable_arguments,
            &strange_renames,
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
                TypeOfProblem::InvalidNumberOfArguments | TypeOfProblem::Mismatch3264BitFunctions => {
                    continue; //"Not checked function(not Wine bug, just problem with parsing)"
                }
            };
            println!("\t{}   -   {}", function, pro);
        }
    }
}
