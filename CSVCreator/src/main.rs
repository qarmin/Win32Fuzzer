use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

const BAD_MESSAGES: &[&str] = &["CrashesLinux", "NotImplementedLinux", "CrashesLinux0Info", "CrashesWindows", "Other"];

const GOOD_MESSAGES: &[&str] = &["NoProblem"];

struct FunctionDataOverTime {
    pub maps: BTreeMap<String, String>,
    pub class: String,
}
impl FunctionDataOverTime {
    fn new() -> Self {
        FunctionDataOverTime {
            maps: Default::default(),
            class: "".to_string(),
        }
    }
}

fn main() {
    let mut result_map: BTreeMap<String, FunctionDataOverTime> = Default::default();

    let mut things: BTreeMap<String, String> = Default::default();

    let files = ["_7_04_64.csv", "_f_7_04.csv", "_win10.csv"];

    for name in files {
        let n = name.split('.').next().unwrap().strip_prefix('_').unwrap();
        things.insert(name.to_string(), n.to_string());
    }

    for (file, smaller_file) in things.clone() {
        let content = match fs::read_to_string(&file) {
            Ok(t) => t,
            Err(e) => {
                println!("Failed to read {} due {}", file, e);
                continue;
            }
        };

        let lines = content.lines();

        for line in lines {
            let parts: Vec<_> = line.split(',').collect();
            if parts.len() != 3 {
                println!("Should be prepared 3 parts in row {}", line);
                continue;
            }

            let class = parts[0].to_string();
            let function = parts[1].to_string();
            let what = parts[2].to_string();

            if !result_map.contains_key(&function) {
                let mut f = FunctionDataOverTime::new();
                f.class = class;
                result_map.insert(function.clone(), f);
            }

            let over_time = result_map.get_mut(&function).unwrap();

            if GOOD_MESSAGES.contains(&what.as_str()) {
                over_time.maps.insert(smaller_file.to_string(), what); //  "Works");
            } else if BAD_MESSAGES.contains(&what.as_str()) {
                over_time.maps.insert(smaller_file.to_string(), what); //"Not works");
            } else {
                panic!("Not supported {}", what);
            }
        }
    }

    let mut result_file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("results.csv")
        .unwrap();

    let mut to_print: String = "Class,Function,".to_string();
    for (file, _smaller_file) in &things {
        to_print += &file;
        to_print += ","
    }
    to_print.pop();
    writeln!(result_file, "{}", to_print).unwrap();

    for (function_name, one_time) in result_map {
        let mut to_print = one_time.class.clone();

        to_print += ",";
        to_print += &function_name;
        to_print += ",";

        for (_file, smaller_file) in &things {
            if let Some(result) = one_time.maps.get(smaller_file.as_str()) {
                to_print += result;
            } else {
                to_print += "=";
            }
            to_print += ",";
        }
        to_print.pop();
        writeln!(result_file, "{}", to_print).unwrap();
    }
}
