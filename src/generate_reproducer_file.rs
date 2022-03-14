use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

pub fn generate_reproducer_file(functions_classes: BTreeMap<String, Vec<String>>) {
    //pub const DATA_TO_USE: &[(&str, &str)] = &[("DwmShowContact", "Dwm")];
    let file_name = format!("Reproducer/src/classes_origin.rs");

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_name).unwrap();
    let mut file = BufWriter::new(file);

    writeln!(file, "pub const DATA_TO_USE: &[(&str, &str)] = &[").unwrap();

    for (name_of_class, mut functions) in functions_classes {
        functions.sort();

        for function in functions {
            writeln!(file, "\t(\"{}\",\"{}\"),", name_of_class, function).unwrap();
        }
    }

    writeln!(file, "];").unwrap();
}
