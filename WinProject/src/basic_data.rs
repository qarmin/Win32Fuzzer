use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

static IGNORE_RESULT_PRINTING: AtomicBool = AtomicBool::new(false);

pub struct SettingsTaker {
    pub(crate) ignored_functions: Vec<String>,
    pub(crate) allowed_functions: Vec<String>,
    pub(crate) ignored_classes: Vec<String>,
    pub(crate) allowed_classes: Vec<String>,
    pub(crate) repeating_number: u32,
    pub(crate) all_repeating_number: u32,
    pub(crate) number_of_max_executed_function: i32,
    pub(crate) random: bool,
}

pub fn print_and_save(file: &mut File, text: String) {
    if !IGNORE_RESULT_PRINTING.load(Ordering::Relaxed) {
        println!("{}", text);
        writeln!(file, "{}", text).unwrap();
    }
}

pub fn take_string() -> (String, String) {
    let to_return;

    if rand::random::<bool>() {
        to_return = "".to_string();
    } else {
        to_return = thread_rng().gen_range(-100000..100000).to_string();
    }
    (to_return.to_string(), to_return.to_string())
}

pub fn take_vec_string() -> (Vec<String>, String) {
    let mut to_return = Vec::new();

    for _i in 0..thread_rng().gen_range(0..10) {
        to_return.push(take_string().0);
    }

    (to_return, "Vec::new()".to_string())
}

pub fn take_i32() -> (i32, String) {
    let to_return: i32 = thread_rng().gen_range(-100000..100000);

    (to_return, to_return.to_string())
}
pub fn take_i16() -> (i16, String) {
    let to_return: i16 = thread_rng().gen_range(-32767..=32767);

    (to_return, to_return.to_string())
}
pub fn take_i64() -> (i64, String) {
    let to_return: i64 = thread_rng().gen_range(-100000..100000);

    (to_return, to_return.to_string())
}
pub fn take_u16() -> (u16, String) {
    let to_return: u16 = thread_rng().gen_range(0..=65535);

    (to_return, to_return.to_string())
}
pub fn take_u8() -> (u8, String) {
    let to_return: u8 = thread_rng().gen_range(0..=255);

    (to_return, to_return.to_string())
}
pub fn take_i8() -> (i8, String) {
    let to_return: i8 = thread_rng().gen_range(-127..=127);

    (to_return, to_return.to_string())
}
pub fn take_u32() -> (u32, String) {
    let to_return: u32 = thread_rng().gen_range(0..100000);

    (to_return, to_return.to_string())
}
pub fn take_u64() -> (u64, String) {
    let to_return: u64 = thread_rng().gen_range(0..100000);

    (to_return, to_return.to_string())
}
pub fn take_f64() -> (f64, String) {
    let to_return: f64 = thread_rng().gen_range(-100000.0..100000.0);

    (to_return, to_return.to_string())
}
pub fn take_f32() -> (f32, String) {
    let to_return: f32 = thread_rng().gen_range(-100000.0..100000.0);

    (to_return, to_return.to_string())
}
pub fn take_usize() -> (usize, String) {
    let to_return: usize = thread_rng().gen_range(0..100000);

    (to_return, to_return.to_string())
}
pub fn take_isize() -> (isize, String) {
    let to_return: isize = thread_rng().gen_range(-100000..100000);

    (to_return, to_return.to_string())
}
pub fn take_char() -> (char, String) {
    let to_return: char = thread_rng().gen_range(0..127) as u8 as char;

    (to_return, to_return.to_string())
}

pub fn take_bool() -> (bool, String) {
    let to_return: bool = rand::random::<bool>();

    (to_return, to_return.to_string())
}

pub fn read_from_file() -> SettingsTaker {
    let args : Vec<_>= std::env::args().collect();

    let mut file_location = "settings.txt".to_string();
    let mut used_class = vec![];
    let mut used_function = vec![];
    let mut disable_printing = false;
    let mut repeating_number = 3;

    for i in args{
        if let Some(st) =  i.strip_prefix("AAA"){
            file_location = st.to_string();
        }
        else if let Some(st) =  i.strip_prefix("BBB"){
            used_class.push( st.to_string());
        }
        else if let Some(st) =  i.strip_prefix("CCC"){
            used_function.push( st.to_string());
        }
        else if let Some(st) =  i.strip_prefix("DDD"){
            if let Ok(t) = st.parse::<u32>(){
                repeating_number = t;
            }
        }
        else if i == "DISABLE_PRINTING"{
            disable_printing = true;
        }
    }


    let string: String = match fs::read_to_string(&file_location) {
        Ok(t) => t,
        Err(_) => {
            println!("Missing {} file",file_location);
            if !used_class.is_empty() {
                println!("Allowed classes:");
                for i in &used_class {
                    println!("{}", i);
                }
            }
            if !used_function.is_empty() {
                println!("Allowed functions:");
                for i in &used_function {
                    println!("{}", i);
                }
            }
            if disable_printing {
                IGNORE_RESULT_PRINTING.store(true,Ordering::Relaxed);
            }
            println!("Repeating - {}", repeating_number);
            println!("Ignore pringing - {}", IGNORE_RESULT_PRINTING.load(Ordering::Relaxed));
            return SettingsTaker {
                ignored_functions: vec![],
                allowed_functions: used_function,
                ignored_classes: vec![],
                allowed_classes: used_class,
                repeating_number,
                all_repeating_number: 1,
                number_of_max_executed_function: -1,
                random: false,
            };
        }
    };

    let mut st: SettingsTaker = SettingsTaker {
        ignored_functions: vec![],
        allowed_functions: vec![],
        ignored_classes: vec![],
        allowed_classes: vec![],
        repeating_number: 3,
        all_repeating_number: 1,
        number_of_max_executed_function: -1,
        random: false,
    };

    enum MODES {
        None,
        IgnoredFunctions,
        AllowedFunctions,
        IgnoredClasses,
        AllowedClasses,
        Repeating,
        AllRepeating,
        MaxExecutedFunction,
        Random,
    }

    let mut current_mode: MODES = MODES::None;
    for line in string.split('\n').map(|e| e.to_string()).collect::<Vec<String>>() {
        let new_line = line.trim().to_string();
        if new_line.starts_with("//") {
            continue; // Comment
        }

        if new_line == "ignored_functions:" {
            current_mode = MODES::IgnoredFunctions;
        } else if new_line == "allowed_functions:" {
            current_mode = MODES::AllowedFunctions;
        } else if new_line == "ignored_classes:" {
            current_mode = MODES::IgnoredClasses;
        } else if new_line == "allowed_classes:" {
            current_mode = MODES::AllowedClasses;
        } else if new_line == "repeating_number:" {
            current_mode = MODES::Repeating;
        } else if new_line == "all_repeating_number:" {
            current_mode = MODES::AllRepeating;
        } else if new_line == "number_of_max_executed_function:" {
            current_mode = MODES::MaxExecutedFunction;
        } else if new_line == "number_of_max_executed_function:" {
            current_mode = MODES::Random;
        } else if new_line == "disable_printing:" {
            IGNORE_RESULT_PRINTING.store(true,Ordering::Relaxed);
        } else {
            if !new_line.is_empty() {
                match current_mode {
                    MODES::IgnoredFunctions => st.ignored_functions.push(new_line),
                    MODES::AllowedFunctions => st.allowed_functions.push(new_line),
                    MODES::IgnoredClasses => st.ignored_classes.push(new_line),
                    MODES::AllowedClasses => st.allowed_classes.push(new_line),
                    MODES::Repeating => {
                        if let Ok(number) = new_line.parse() {
                            st.repeating_number = number;
                        }
                    }
                    MODES::AllRepeating => {
                        if let Ok(number) = new_line.parse() {
                            st.all_repeating_number = number;
                        }
                    }
                    MODES::MaxExecutedFunction => {
                        if let Ok(number) = new_line.parse() {
                            st.number_of_max_executed_function = number;
                        }
                    }
                    MODES::Random => match new_line.trim() {
                        "1" | "true" => st.random = true,
                        "0" | "false" => st.random = false,
                        _ => (),
                    },
                    MODES::None => println!("SETTING: Missing mode for {}", new_line),
                }
            }
        }
    }
    // Print found data
    {
        println!("Start settings loading");

        if !st.ignored_classes.is_empty() {
            println!("Ignored classes:");
            for i in &st.ignored_classes {
                println!("{}", i);
            }
        }
        if !st.allowed_classes.is_empty() {
            println!("Allowed classes:");
            for i in &st.allowed_classes {
                println!("{}", i);
            }
        }
        if !st.allowed_functions.is_empty() {
            println!("Allowed functions:");
            for i in &st.allowed_functions {
                println!("{}", i);
            }
        }
        if !st.ignored_functions.is_empty() {
            println!("Ignored functions:");
            for i in &st.ignored_functions {
                println!("{}", i);
            }
        }
        println!("Repeating - {}", st.repeating_number);
        println!("All Repeating - {}", st.all_repeating_number);
        println!("Max Executed Functions - {}", st.number_of_max_executed_function);
        println!("Randoms - {}", st.random);
        println!("Ignore pringing - {}", IGNORE_RESULT_PRINTING.load(Ordering::Relaxed));
        println!("End settings loading");
    }

    st
}
