use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

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
