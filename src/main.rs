mod parse_file;
mod settings;
mod create_project_file;

use std::io::Write;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use crate::parse_file::*;
use create_project_file::*;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::iter::zip;
use crate::settings::*;

const WINDOWS_RS_FOLDER: &str = "/home/rafal/test/windows-rs/";

fn main() {
    let things = ("Printer",format!("{}{}",WINDOWS_RS_FOLDER,"crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"));

    let create_renames2 = [
        ("u32","take_u32"),
        ("usize","take_usize"),
        ("super::super::Foundation::HANDLE","get_file_handle"),
        ("::windows_sys::core::PCSTR","get_file_pcstr"),
        ("::windows_sys::core::PCWSTR","get_file_pcwstr"),
        ("::windows_sys::core::PSTR","get_file_pstr"),
            ("super::super::Foundation::HWND","get_hwnd"),
    ];

    let mut create_renames : HashMap<&str,&str> = HashMap::new();
    for i in create_renames2{
        create_renames.insert(i.0,i.1);
    }





    let mut file_data = FileData::new();
    parse_file(&mut file_data, &things.1);
    create_project_file(&mut file_data, &things.0, &create_renames);
}


// pub fn ChoosePixelFormat(hdc: super::Gdi::HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;