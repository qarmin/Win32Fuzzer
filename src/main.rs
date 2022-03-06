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
        ("i8", "take_i8"),
        ("i64", "take_i64"),
        ("isize", "take_isize"),
        ("char", "take_char"),
        ("string", "take_string"),
        // More advanced
        ("AsnObjectIdentifier", "get_strange_AsnObjectIdentifier"),
        ("super::super::super::Foundation::BOOL", "get_strange_BOOL"),
        ("super::super::Foundation::BOOL", "get_strange_BOOL"),
        ("super::super::Foundation::BOOLEAN", "get_strange_BOOLEAN"),
        ("super::super::super::Foundation::BOOLEAN", "get_strange_BOOLEAN"),
        ("super::super::Foundation::BSTR", "get_strange_BSTR"),
        ("super::super::Foundation::CHAR", "get_strange_CHAR"),
        ("COORD", "get_strange_COORD"),
        ("super::super::Foundation::FILETIME", "get_strange_FILETIME"),
        ("::windows_sys::core::GUID", "get_strange_GUID"),
        ("super::Foundation::HANDLE", "get_strange_HANDLE"),
        ("super::super::Foundation::HANDLE", "get_strange_HANDLE"),
        ("super::super::super::Foundation::HANDLE", "get_strange_HANDLE"),
        ("super::super::Graphics::Gdi::HBITMAP", "get_strange_HBITMAP"),
        ("super::Gdi::HDC", "get_strange_HDC"),
        ("super::super::Graphics::Gdi::HDC", "get_strange_HDC"),
        ("super::HDIAGNOSTIC_DATA_QUERY_SESSION", "get_strange_HDIAGNOSTIC_DATA_QUERY_SESSION"),
        ("HIMAGELIST", "get_strange_HIMAGELIST"),
        ("super::super::super::Globalization::HIMC", "get_strange_HIMC"),
        ("super::super::super::Globalization::HIMCC", "get_strange_HIMCC"),
        ("super::super::Foundation::HINSTANCE", "get_strange_HINSTANCE"),
        ("super::super::super::Foundation::HINSTANCE", "get_strange_HINSTANCE"),
        ("super::super::System::Registry::HKEY", "get_strange_HKEY"),
        ("HKEY", "get_strange_HKEY"),
        ("super::super::TextServices::HKL", "get_strange_HKL"),
        ("HMENU", "get_strange_HMENU"),
        ("super::WindowsAndMessaging::HMENU", "get_strange_HMENU"),
        ("super::super::UI::WindowsAndMessaging::HMENU", "get_strange_HMENU"),
        ("HRGN", "get_strange_HRGN"),
        ("::windows_sys::core::HSTRING", "get_strange_HSTRING"),
        ("super::super::super::Foundation::HWND", "get_strange_HWND"),
        ("super::super::Foundation::HWND", "get_strange_HWND"),
        ("super::StructuredStorage::JET_SESID", "get_strange_JET_SESID"),
        ("super::StructuredStorage::JET_TABLEID", "get_strange_JET_TABLEID"),
        ("super::super::super::Foundation::LPARAM", "get_strange_LPARAM"),
        ("super::super::Foundation::LPARAM", "get_strange_LPARAM"),
        ("MENU_ITEM_FLAGS", "get_strange_MENU_ITEM_FLAGS"),
        ("MENU_ITEM_FLAGS", "get_strange_MENU_ITEM_FLAGS"),
        ("::windows_sys::core::PCWSTR", "get_strange_PCWSTR"),
        ("PFNPROPSHEETUI", "get_strange_PFNPROPSHEETUI"),
        ("super::super::super::Foundation::PSID", "get_strange_PSID"),
        ("super::super::Foundation::PSID", "get_strange_PSID"),
        ("super::Foundation::PSID", "get_strange_PSID"),
        ("::windows_sys::core::PCSTR", "get_strange_PCSTR"),
        ("::windows_sys::core::PSTR", "get_strange_PSTR"),
        ("::windows_sys::core::PCWSTR", "get_strange_PCWSTR"),
        ("::windows_sys::core::PWSTR", "get_strange_PWSTR"),
        ("super::super::Foundation::RECT", "get_strange_RECT"),
        ("super::super::super::Foundation::WPARAM", "get_strange_WPARAM"),
        ("super::super::Foundation::WPARAM", "get_strange_WPARAM"),
        ("ldap", "get_strange_ldap"),
    ];

    let mut create_renames: HashMap<&str, &str> = HashMap::new();
    for i in create_renames2 {
        create_renames.insert(i.0, i.1);
    }

    // find_things();
    create_main_file(&things);

    let mut ignored_arguments: BTreeMap<String, u32> = Default::default(); // List of ignored arguments

    for entry in WalkDir::new("WinProject") {
        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().to_string();
        if path.contains("z_") {
            // println!("Path to remove {}", path);
            let _ = fs::remove_file(path);
        }
    }

    for (class_name, path, exceptions) in things {
        if DISABLED_CLASSES.contains(&class_name) {
            continue;
        }
        let exceptions: HashSet<_> = exceptions.iter().map(|e| e.to_string()).collect();
        let mut file_data = FileData::new();
        parse_file(&mut file_data, &path);
        create_project_file(&file_data, class_name, &create_renames, &exceptions, &mut ignored_arguments);
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

    for entry in WalkDir::new(&base_path).into_iter().flatten() {
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

    for i in btreeset {
        println!("{}", i);
    }

    println!("Found {}", idx);
}
