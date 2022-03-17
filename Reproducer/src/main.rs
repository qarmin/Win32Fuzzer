mod classes_origin;

use crate::OsThing::{Linux, Windows};
use classes_origin::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

// Needs to execute - winetricks nocrashdialog - to hide crash dialog

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
enum TypeOfProblem {
    CrashesLinux,          // Crashes Linux
    CrashesLinux0Info,     // Crashes Linux, but without any message
    NotImplementedLinux,   // not implemented
    CrashesWindows,        // Crashes Linux
    CrashesWindows0Info,   // Crashes Linux, but without any message
    NotImplementedWindows, // not implemented
    Other,                 // This error should not exists, so when it happen, needs to checked
    NoProblem,             // No error,
                           // MissingError,          //
}
impl TypeOfProblem {
    pub fn to_string(&self) -> &'static str {
        match self {
            TypeOfProblem::CrashesLinux => "CrashesLinux",
            TypeOfProblem::CrashesLinux0Info => "CrashesLinux0Info",
            TypeOfProblem::NotImplementedLinux => "NotImplementedLinux",
            TypeOfProblem::CrashesWindows => "CrashesWindows",
            TypeOfProblem::CrashesWindows0Info => "CrashesWindows0Info",
            TypeOfProblem::NotImplementedWindows => "NotImplementedWindows",
            TypeOfProblem::Other => "Other",
            TypeOfProblem::NoProblem => "NoProblem",
            // TypeOfProblem::MissingError => "MissingError",
        }
    }
}

struct FunctionInfo {
    class_name: String,
    function_name: String,
    type_of_problem: TypeOfProblem,
}

impl FunctionInfo {
    pub fn new() -> Self {
        FunctionInfo {
            class_name: "XXXXXXXXXXXXX".to_string(),
            function_name: "YYYYYYYYYYYYYY".to_string(),
            type_of_problem: TypeOfProblem::NoProblem,
        }
    }
}

#[derive(Eq, PartialEq)]
enum OsThing {
    Windows,
    Linux,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!(
            "Provided too small amount of arguments {}, at least 2 are required - path to fuzzer and system(linux,windows)",
            args.len() - 1
        );
        return;
    }

    let lowercase = args[2].to_lowercase();
    let os_thing = match lowercase.as_ref() {
        "linux" => Linux,
        "windows" => Windows,
        _ => {
            println!("Expected Linux/Windows, found {}", lowercase);
            return;
        }
    };

    let path_to_fuzzer = args[1].clone();

    let start_time = SystemTime::now();

    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    // Zczytywać te dane z innego
    let functions_classes = DATA_TO_USE;

    let beginning_file = Arc::new(Mutex::new(File::create("used.txt").unwrap()));

    let atomic_uint: AtomicU32 = AtomicU32::new(0);
    let function_infos: Vec<_> = functions_classes
        .par_iter()
        .enumerate()
        .map(|(_index, (class_name, function_name))| {
            let mut function_info: FunctionInfo = FunctionInfo::new();
            function_info.function_name = function_name.to_string();
            function_info.class_name = class_name.to_string();
            function_info.type_of_problem = TypeOfProblem::NoProblem;

            println!(
                "Checking {}/{} item {} - {}",
                atomic_uint.fetch_add(1, Ordering::Release),
                functions_classes.len(),
                class_name,
                function_name
            );

            {
                let mut file = beginning_file.lock().unwrap();
                writeln!(file, "{} START", function_info.function_name).unwrap();
            }

            let mut comm;
            let mut comm2;

            let handler;

            if os_thing == Linux {
                comm = Command::new("timeout");
                // comm = Command::new("wine"); // when disabling timeout
            } else {
                comm = Command::new(&path_to_fuzzer);
            }
            comm2 = &mut comm;
            if os_thing == Linux {
                comm2 = comm2.arg("30"); // 30 second timeout
                comm2 = comm2.arg("wine");
                comm2 = comm2.arg(&path_to_fuzzer);
            }
            handler = comm2
                .arg(format!("BBB{}", class_name))
                .arg(format!("CCC{}", function_name))
                .arg(format!("DDD{}", 10))
                .arg("DISABLE_PRINTING")
                .arg("REPRODUCIBLE")
                .output()
                .unwrap();

            // println!("Error {}", String::from_utf8_lossy(&handler.stderr));
            // println!("Output {}", String::from_utf8_lossy(&handler.stdout));
            // println!("STATUS {:?}", &handler.status);

            let command_output = String::from_utf8_lossy(&handler.stdout).to_string();
            let command_error = String::from_utf8_lossy(&handler.stderr).to_string();
            let status = handler.status.code().unwrap();
            println!("Output {}", command_output);

            match status {
                0 => {
                    if !command_output.contains("Ending fuzzing.") {
                        if os_thing == Linux {
                            function_info.type_of_problem = TypeOfProblem::CrashesLinux0Info;
                        } else {
                            function_info.type_of_problem = TypeOfProblem::CrashesWindows0Info;
                        }
                    }
                }
                _ => {
                    // Failed to execute command, bug/crash when running
                    if command_output.contains("unimplemented function") {
                        if os_thing == Linux {
                            function_info.type_of_problem = TypeOfProblem::NotImplementedLinux;
                        } else {
                            function_info.type_of_problem = TypeOfProblem::NotImplementedWindows;
                        }
                    } else if command_output.contains("crashes")
                        || command_output.contains("page fault on read access")
                        || command_output.contains("page fault on write access")
                        || command_output.contains("Unhandled exception")
                    {
                        if os_thing == Linux {
                            function_info.type_of_problem = TypeOfProblem::CrashesLinux;
                        } else {
                            function_info.type_of_problem = TypeOfProblem::CrashesWindows;
                        }
                    } else if command_error.contains("Assertion") {
                        if os_thing == Linux {
                            function_info.type_of_problem = TypeOfProblem::CrashesLinux;
                        } else {
                            function_info.type_of_problem = TypeOfProblem::CrashesWindows;
                        }
                    } else {
                        println!(
                            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n {}\n, {}",
                            command_output, command_error
                        );

                        if os_thing == Linux {
                            function_info.type_of_problem = TypeOfProblem::Other;
                        } else {
                            function_info.type_of_problem = TypeOfProblem::CrashesWindows;
                            // On Windows, there is no error messages
                        }
                    }
                }
            }

            {
                let mut file = beginning_file.lock().unwrap();
                writeln!(file, "{} END", function_info.function_name).unwrap();
            }

            function_info
        })
        .collect();

    println!("ENDING CHECKING");

    // Consider to save this to file, because output may be polluted by wine messages
    let mut file_csv = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("csv_results.csv")
        .unwrap();

    let mut btreemap: BTreeMap<TypeOfProblem, u32> = Default::default();
    for function_info in function_infos {
        let to_print = format!(
            "{},{},{}",
            function_info.class_name,
            function_info.function_name,
            function_info.type_of_problem.to_string()
        );

        println!("{}", to_print);
        writeln!(file_csv, "{}", to_print).unwrap();

        if let Some(key) = btreemap.get_mut(&function_info.type_of_problem) {
            *key += 1;
        } else {
            btreemap.insert(function_info.type_of_problem, 1);
        }
    }

    println!("Checked {} functions", btreemap.values().sum::<u32>());

    for (key, value) in btreemap {
        println!("{} - {}", key.to_string(), value);
    }

    let end_time = SystemTime::now();
    println!("Operation took {:?}", end_time.duration_since(start_time).unwrap());
}
