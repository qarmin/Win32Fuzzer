mod parse_file;
mod settings;

use std::io::Write;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use crate::parse_file::*;
use std::fs::OpenOptions;
use std::io::BufWriter;
use crate::settings::*;

const WINDOWS_RS_FOLDER: &str = "/home/rafal/test/windows-rs/";

fn main() {
    let things = ("Printer",format!("{}{}",WINDOWS_RS_FOLDER,"crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"));

    let create_renames2 = [("asasf","afsa")];

    let mut create_renames : HashMap<String,String> = HashMap::new();
    for i in create_renames2{
        create_renames.insert(i.0.to_string(),i.1.to_string());
    }





    let mut file_data = FileData::new();
    parse_file(&mut file_data, &things.1);
    create_project_file(&mut file_data, &things.0);
}

pub fn create_project_file(file_data: &FileData, file_path : &str){

    let header = r###"use windows::{Win32::Foundation::*, Win32::Graphics::Printing::*};
use windows::core::{GUID, PCSTR, PCWSTR};

fn main()  {
    unsafe {
    "###;
    let footer = r###"    }
}"###;

    let file_name = format!("WinProject/src/z_{}.rs",file_path.to_lowercase());

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_name).unwrap();
    let mut file = BufWriter::new(file);

    writeln!(file,"{}",header).unwrap();

    for functions in &file_data.functions{

    }





    writeln!(file,"{}",footer).unwrap();
}

// pub fn ChoosePixelFormat(hdc: super::Gdi::HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;