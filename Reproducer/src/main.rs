mod classes_origin;

use classes_origin::*;
use std::collections::BTreeMap;
// use std::fs;
use rayon::prelude::*;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::SystemTime;

// Needs to execute - winetricks nocrashdialog - to hide crash dialog

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
enum TypeOfProblem {
    CrashesLinux,        // Crashes Linux
    NotImplementedLinux, // not implemented
    Other,               // This error should not exists, so when it happen, needs to
    NoProblem,           // No error,
}
impl TypeOfProblem {
    pub fn to_string(&self) -> &'static str {
        match self {
            TypeOfProblem::CrashesLinux => "CrashesLinux",
            TypeOfProblem::NotImplementedLinux => "NotImplementedLinux",
            TypeOfProblem::Other => "Other",
            TypeOfProblem::NoProblem => "NoProblem",
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

fn main() {
    let start_time = SystemTime::now();

    // Później dodać to pole konfigurowalne
    let path_to_fuzzer = "/home/rafal/Projekty/Rust/Win32Fuzzer/WinProject/target/debug/win_project.exe";

    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    // Zczytywać te dane z innego
    let functions_classes = DATA_TO_USE;

    // let _max_executed_in_loop; // Będzie to możliwe tylko przy dodaniu ścieżki bezwzględnej do projektu

    // for (index, (class_name, function_name)) in functions_classes.iter().enumerate() {

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

            // let status_file_name = format!("{}_{}.txt", class_name, function_name);
            // .arg(&status_file_name)
            let handler = Command::new("wine")
                .arg(path_to_fuzzer)
                .arg(format!("BBB{}", class_name))
                .arg(format!("CCC{}", function_name))
                .arg(format!("DDD{}", 10))
                .arg("DISABLE_PRINTING")
                .arg("REPRODUCIBLE")
                // .arg("2>&1")
                // .arg("|")
                // .arg("tee")
                // .arg("/home/rafal/tt.txt")
                .output()
                .unwrap();

            // println!("Error {}", String::from_utf8_lossy(&handler.stderr));
            // println!("Output {}", String::from_utf8_lossy(&handler.stdout));
            // println!("STATUS {:?}", &handler.status);

            let command_output = String::from_utf8_lossy(&handler.stdout).to_string();
            let status = handler.status.code().unwrap();
            println!("Output {}", command_output);

            match status {
                0 => {
                    // println!("Just fine");
                }
                _ => {
                    // Failed to execute command, bug/crash when running

                    if command_output.contains("unimplemented function") {
                        function_info.type_of_problem = TypeOfProblem::NotImplementedLinux;
                    } else if command_output.contains("crashes") || command_output.contains("page fault on read access") {
                        function_info.type_of_problem = TypeOfProblem::CrashesLinux;
                    } else {
                        function_info.type_of_problem = TypeOfProblem::Other;
                    }
                }
            }

            // function_infos.push(function_info);
            function_info
            // let handler = OS.execute(vec!["wineserver","-k"]); // Should not be used when using multithreading
        })
        .collect();

    println!("ENDING CHECKING");

    // Consider to save this to file, because output may be polluted by wine messages

    let mut btreemap: BTreeMap<TypeOfProblem, u32> = Default::default();
    for function_info in function_infos {
        println!(
            "{}.{} ___ result {}",
            function_info.class_name,
            function_info.function_name,
            function_info.type_of_problem.to_string()
        );

        if let Some(key) = btreemap.get_mut(&function_info.type_of_problem) {
            *key += 1;
        } else {
            btreemap.insert(function_info.type_of_problem, 1);
        }
    }
    for (key, value) in btreemap {
        println!("{} - {}", key.to_string(), value);
    }

    let end_time = SystemTime::now();
    println!("Operation took {:?}", end_time.duration_since(start_time).unwrap());
}
