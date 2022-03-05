mod create_main_file;
mod create_project_file;
mod parse_file;
mod settings;

use crate::parse_file::*;
use crate::settings::*;
use create_main_file::*;
use create_project_file::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use walkdir::WalkDir;

fn main() {
    let things = load_settings();

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
        // ("i8", "take_i8"),
        ("i64", "take_i64"),
        ("isize", "take_isize"),
        ("char", "take_char"),
        ("string", "take_string"),
        // More advanced
        ("super::super::Foundation::BOOL", "get_foundation_bool"),
        ("super::super::Foundation::HANDLE", "get_file_handle"),
        ("super::super::super::Foundation::HANDLE", "get_file_handle"),
        ("::windows_sys::core::PCSTR", "get_file_pcstr"),
        ("::windows_sys::core::PCWSTR", "get_file_pcwstr"),
        ("::windows_sys::core::PSTR", "get_file_pstr"),
        ("::windows_sys::core::PWSTR", "get_file_pwstr"),
        ("super::super::Foundation::LPARAM", "get_lparam"),
        ("super::super::Foundation::HWND", "get_hwnd"),
        ("super::Gdi::DEVMODEA", "get_devmodea"),
        ("super::Gdi::DEVMODEW", "get_devmodew"),
        ("PFNPROPSHEETUI", "get_pfnpropsheetui"),
        ("get_lparam", "super::super::Foundation::LPARAM"),
    ];

    let mut create_renames: HashMap<&str, &str> = HashMap::new();
    for i in create_renames2 {
        create_renames.insert(i.0, i.1);
    }

    // find_things();
    create_main_file(&things);

    let mut ignored_arguments: BTreeMap<String, u32> = Default::default(); // List of ignored arguments

    for (class_name, path, exceptions) in things {
        if DISABLED_CLASSES.contains(&&class_name) {
            let _ = fs::remove_file(&format!("WinProject/src/z_{}.rs", class_name.to_lowercase()));
            continue;
        }
        let exceptions: HashSet<_> = exceptions.iter().map(|e| e.to_string()).collect();
        let mut file_data = FileData::new();
        parse_file(&mut file_data, &path);
        create_project_file(&mut file_data, &class_name, &create_renames, &exceptions, &mut ignored_arguments);
    }

    {
        let mut new_btreemap: BTreeMap<u32, String> = Default::default();
        for (key, value) in ignored_arguments {
            new_btreemap.insert(value, key);
        }
        for (key, value) in new_btreemap {
            println!("Not supported '''{}''', found {} occurences", value, key);
        }
    }
}

pub fn find_things() {
    let base_path = format!("{}crates/libs/sys/src/Windows/Win32/", WINDOWS_RS_FOLDER);

    let mut idx = 0;

    let mut btreeset: BTreeSet<String> = Default::default();
    let mut used_names: HashSet<String> = Default::default();

    for entry in WalkDir::new(&base_path) {
        if let Ok(entry) = entry {
            let path = entry.path().to_string_lossy().to_string();
            if path.ends_with("mod.rs") {
                // println!("{}", path);
                let without_prefix = path.strip_prefix(WINDOWS_RS_FOLDER).unwrap();
                // println!("WP - {}", without_prefix);

                let file_content = match fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("Failed to read file {}", e);
                        continue;
                    }
                };
                if file_content.contains("\nextern \"system\" {") {
                    idx += 1;
                    let mut end;
                    {
                        let split: Vec<_> = path.split("/").collect();
                        end = split[split.len() - 2].to_string();

                        // println!("Checking {}", end);
                        if used_names.contains(&end) {
                            // println!("DUPLICATED {}", end);
                            end.push('2');
                        }
                        used_names.insert(end.clone());
                        // println!("END {}", end);
                    }
                    // Manually generated list of classes which have already exceptions
                    if MANUAL_CLASSES.contains(&end.as_str()) {
                        // println!("Manually handled {}", end);
                        continue;
                    }

                    btreeset.insert(format!(
                        "(\"{}\",format!(\"{{}}{{}}\",WINDOWS_RS_FOLDER,\"{}\"),vec![],),",
                        end, without_prefix
                    ));
                    // println!("(\"{}\",format!(\"{{}}{{}}\",WINDOWS_RS_FOLDER,\"{}\"),vec![],),", end, without_prefix);
                    // println!("CONTAINS! {}", path);
                }
            }
        }
    }

    for i in btreeset {
        println!("{}", i);
    }

    println!("Found {}", idx);
}
