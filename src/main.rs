mod create_main_file;
mod create_project_file;
mod parse_file;
mod settings;

use crate::parse_file::*;
use crate::settings::*;
use create_main_file::*;
use create_project_file::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

fn main() {
    let things = load_settings();

    let create_renames2 = [
        // Basic
        ("u32", "take_u32"),
        ("usize", "take_usize"),
        ("u64", "take_u64"),
        ("i32", "take_i32"),
        // ("i8", "take_i8"),
        ("i64", "take_i64"),
        // ("isize", "take_isize"),
        ("char", "take_char"),
        ("string", "take_string"),
        // More advanced
        ("super::super::Foundation::HANDLE", "get_file_handle"),
        ("::windows_sys::core::PCSTR", "get_file_pcstr"),
        ("::windows_sys::core::PCWSTR", "get_file_pcwstr"),
        ("::windows_sys::core::PSTR", "get_file_pstr"),
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

// pub fn ChoosePixelFormat(hdc: super::Gdi::HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
