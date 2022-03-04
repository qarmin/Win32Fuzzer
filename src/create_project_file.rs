use crate::FileData;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;

pub fn create_project_file(
    file_data: &FileData,
    file_path: &str,
    create_renames: &HashMap<&str, &str>,
) {
    let header = r###"use windows::{Win32::Foundation::*, Win32::Graphics::Printing::*};
use windows::core::{GUID, PCSTR, PCWSTR};
use crate::basic_data::*;
use crate::more_bad_data::*;

pub fn z_printer()  {
    unsafe {
    "###;
    let footer = r###"    }
}"###;

    let file_name = format!("WinProject/src/z_{}.rs", file_path.to_lowercase());

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_name)
        .unwrap();
    let mut file = BufWriter::new(file);

    writeln!(file, "{}", header).unwrap();

    for (function_name, arguments) in &file_data.functions {
        // Check if function is supported
        let mut is_supported: bool = true;
        let mut function_names: Vec<String> = Vec::new();
        let mut additional_arguments: Vec<ArgumentAdditional> = Vec::new();

        let mut function_info = FunctionInfo::new();

        for arg in arguments {
            if arg.contains("*") {
                println!("Not supported '''{}''' due *", arg);
                is_supported = false;
            } else {
                if let Some(function_creator) = create_renames.get(arg.as_str()) {
                    // println!("Supported '''{}'''",arg);
                    function_names.push(function_creator.to_string());

                    let mut add_arg = ArgumentAdditional::new();
                    add_arg.original_argument = arg.clone();

                    let ss  = arg.s
                    (add_arg.argument_before, add_arg.argument_type) = ;


                    function_info.arguments.push(add_arg);
                } else {
                    println!("Not supported '''{}''' due missing function", arg);
                    is_supported = false;
                }
            }
        }
        if !is_supported {
            continue;
        }
        assert_eq!(arguments.len(), function_names.len());
        let zipped = arguments.iter().zip(function_names.iter());

        let mut execute_arguments = "".to_string();
        for (index, (arg, create_function_name)) in zipped.enumerate() {
            writeln!(
                file,
                "\t\tlet argument_{} = {}();",
                index, create_function_name
            )
            .unwrap();
            execute_arguments += &format!("argument_{}.0", index);
            if arguments.len() - 1 != index {
                execute_arguments += ",";
            }
        }
        writeln!(file, "\t\t{}({});", function_name, execute_arguments).unwrap();
    }

    writeln!(file, "{}", footer).unwrap();
}

struct FunctionInfo {
    pub function_name: String,
    pub arguments: Vec<ArgumentAdditional>,
}

impl FunctionInfo {
    pub fn new() -> Self {
        FunctionInfo {
            function_name: "".to_string(),
            arguments: Vec::new(),
        }
    }
}

struct ArgumentAdditional {
    pub original_argument: String, // E.g. *mut AABB
    pub argument_type: String,     // Like AABB from *AABB
    pub argument_before: String,   // Like *mut from *mut AABB
}
impl ArgumentAdditional {
    pub fn new() -> Self {
        ArgumentAdditional {
            original_argument: "".to_string(),
            argument_type: "".to_string(),
            argument_before: "".to_string(),
        }
    }
}
