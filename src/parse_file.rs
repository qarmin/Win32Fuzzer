use crate::FileData;
use std::fs;

pub fn parse_file(file_data: &mut FileData, file_path: &str) {
    let file_content = match fs::read_to_string(file_path) {
        Ok(t) => t,
        Err(e) => panic!("Failed to open {}, due {:?}", file_path, e),
    };

    let mut started_to_read = false;

    let lines = file_content.lines();
    for line in lines {
        let trim_line = line.trim();
        if started_to_read {
            if trim_line.starts_with("pub fn ") {
                let end_line = trim_line.strip_prefix("pub fn ").unwrap();
                if let Some(start_index) = end_line.find("(") {
                    let function_name = &end_line[..start_index];
                    if let Some(end_index) = end_line.find(")") {
                        let arguments_raw = &end_line[start_index + 1..end_index];
                        // println!("DEBUG: Validation {}",arguments_raw);
                        let mut arg = Vec::new();
                        if !arguments_raw.is_empty() {
                            let splits = arguments_raw.split(",");
                            for split in splits {
                                let one_argument = split.split(": ").map(|e| e.to_string()).collect::<Vec<String>>();
                                if one_argument.len() != 2 {
                                    panic!("Found not 2 records '{:?}' in '{}'", one_argument, line);
                                } else {
                                    arg.push(one_argument[1].clone());
                                }
                            }
                        }
                        file_data.functions.insert(function_name.to_string(), arg);
                    } else {
                        panic!("Missing ) in {} (Multiline declaration?)", end_line);
                    }
                } else {
                    panic!("Missing ( in {}", end_line);
                }
            } else if trim_line.starts_with("#") {
            } else if line.starts_with("}") {
                break;
            } else {
                panic!("Found line {}", line);
            }
        } else {
            // TODO, maybe this will not handled
            if line.starts_with("extern \"system\" {") {
                started_to_read = true;
            }
        }
    }

    // for (function, arguments) in &file_data.functions{
    //     let mut  f = function.clone();
    //     f.push_str("    ");
    //     for argument in arguments{
    //         f.push_str(argument.as_str());
    //         f.push_str(",");
    //     }
    //     println!("{}",f);
    // }
}
