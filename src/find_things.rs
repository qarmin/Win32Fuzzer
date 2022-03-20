use std::collections::{BTreeSet, HashSet};
use std::fs;

use walkdir::WalkDir;

use crate::get_windows_rs_folder;
use crate::settings::*;

pub fn find_things(things: &[(&str, &str)]) {
    let base_path = format!("{}crates/libs/sys/src/Windows/Win32/", get_windows_rs_folder());

    let mut _idx = 0;

    let mut btreeset: BTreeSet<String> = Default::default();
    let mut used_names: HashSet<String> = Default::default();

    let mut used_path: HashSet<String> = Default::default();
    for (_name, path) in things {
        let full_path = format!("{}{}", get_windows_rs_folder(), path);
        if used_path.contains(&full_path) {
            panic!("Found duplicated path {}", full_path);
        }
        used_path.insert(full_path);
    }

    for entry in WalkDir::new(&base_path).into_iter().flatten() {
        let path = entry.path().to_string_lossy().to_string();
        if path.ends_with("mod.rs") {
            // println!("{}", path);
            let without_prefix = path.strip_prefix(&get_windows_rs_folder()).unwrap();
            // println!("WP - {}", without_prefix);

            let file_content = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(e) => {
                    println!("Failed to read file {}", e);
                    continue;
                }
            };
            if file_content.contains("\nextern \"system\" {") {
                if !used_path.contains(&path) {
                    println!("Missing file used to parse {}", path);
                    continue;
                }

                _idx += 1;
                let mut end;
                {
                    let split: Vec<_> = path.split('/').collect();
                    end = split[split.len() - 2].to_string();

                    // println!("Checking {}", end);
                    if used_names.contains(&end) {
                        // println!("DUPLICATED {}", end);
                        end.push('2');
                    }
                    used_names.insert(end.clone());
                    // println!("END {}", end);
                }

                btreeset.insert(format!("(\"{}\",format!(\"{{}}{{}}\",get_windows_rs_folder(),\"{}\"),vec![],),", end, without_prefix));
                // println!("(\"{}\",format!(\"{{}}{{}}\",get_windows_rs_folder(),\"{}\"),vec![],),", end, without_prefix);
                // println!("CONTAINS! {}", path);
            }
        }
    }

    // for i in btreeset {
    //     println!("{}", i);
    // }
    //
    // println!("Found {}", idx);
}
