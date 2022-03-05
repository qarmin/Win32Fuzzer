use crate::FileData;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

pub fn create_project_file(
    file_data: &FileData,
    file_path: &str,
    create_renames: &HashMap<&str, &str>,
    ignored_functions: &HashSet<String>,
    ignored_arguments: &mut BTreeMap<String, u32>,
) {
    let header = r###"use windows::{Win32::Foundation::*, Win32::Graphics::Printing::*};
use windows::Win32::Gaming::*;
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::Dwm::*;
use windows::Win32::Graphics::Dxgi::*;

use windows::core::{GUID, PCSTR, PCWSTR};
use crate::basic_data::*;
use crate::more_bad_data::*;

pub fn z_<<<function_name>>>()  {
    unsafe {
    "###;
    let footer = r###"    }
}"###;

    let file_name = format!("WinProject/src/z_{}.rs", file_path.to_lowercase());

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_name).unwrap();
    let mut file = BufWriter::new(file);

    writeln!(file, "{}", header.replace("<<<function_name>>>", &file_path.to_lowercase())).unwrap();

    for (function_name, arguments) in &file_data.functions {
        // Check if function is supported
        let mut is_supported: bool = true;

        let mut function_info = FunctionInfo::new();

        if ignored_functions.contains(function_name) {
            continue;
        }

        for arg in arguments {
            let mut add_arg = ArgumentAdditional::new();
            add_arg.original_argument = arg.clone();

            if let Some(right_space) = arg.rfind(" ") {
                add_arg.argument_before = arg[..right_space].to_string();
                add_arg.argument_type = arg[right_space + 1..].to_string();
            } else {
                add_arg.argument_type = arg.clone();
            }

            if arg.contains("*const") {
                // println!("Not supported '''{}''' const", arg); // Maybe TODO
                // add_to_ignore_arguments(ignored_arguments, arg); // TODO Renable this later
                is_supported = false;
            } else if arg.contains("*mut *mut ") {
                // println!("Not supported '''{}''' twice *mut", arg); // Maybe TODO
                add_to_ignore_arguments(ignored_arguments, arg);
                is_supported = false;
            } else if let Some(function_creator) = create_renames.get(add_arg.argument_type.as_str()) {
                // println!("Supported '''{}'''", arg);
                add_arg.function_name = function_creator.to_string();
                function_info.arguments.push(add_arg);
            } else {
                // println!("Not supported '''{}''' due missing function", arg);
                add_to_ignore_arguments(ignored_arguments, arg);
                is_supported = false;
            }
        }
        if !is_supported {
            continue;
        }

        let mut execute_arguments = "".to_string();
        let mut creation_of_arguments = "".to_string();
        for (index, additional_arguments) in function_info.arguments.iter().enumerate() {
            if additional_arguments.argument_before.contains("mut") {
                creation_of_arguments += &format!("\t\tlet mut argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\t\tlet argument_{} = (&mut argument_{}.0,argument_{}.1);\n", index, index, index);
            } else {
                creation_of_arguments += &format!("\t\tlet argument_{} = {}();\n", index, additional_arguments.function_name);
            }

            execute_arguments += &format!("argument_{}.0", index);
            if arguments.len() - 1 != index {
                execute_arguments += ",";
            }
        }
        write!(file, "{}", creation_of_arguments).unwrap();
        writeln!(file, "\t\t{}({});\n", function_name, execute_arguments).unwrap();
    }
    writeln!(file, "{}", footer).unwrap();
}
pub fn add_to_ignore_arguments(btreemap: &mut BTreeMap<String, u32>, key: &String) {
    if btreemap.contains_key(key) {
        *btreemap.get_mut(key).unwrap() += 1;
    } else {
        btreemap.insert(key.clone(), 1);
    }
}

struct FunctionInfo {
    pub arguments: Vec<ArgumentAdditional>,
}

impl FunctionInfo {
    pub fn new() -> Self {
        FunctionInfo { arguments: Vec::new() }
    }
}

struct ArgumentAdditional {
    pub function_name: String,     // e.g. take_AABB
    pub original_argument: String, // E.g. *mut AABB
    pub argument_type: String,     // Like AABB from *AABB
    pub argument_before: String,   // Like *mut from *mut AABB
}
impl ArgumentAdditional {
    pub fn new() -> Self {
        ArgumentAdditional {
            function_name: "".to_string(),
            original_argument: "".to_string(),
            argument_type: "".to_string(),
            argument_before: "".to_string(),
        }
    }
}
