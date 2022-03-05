use windows::core::{GUID, PCSTR, PCWSTR, PSTR, PWSTR};
use windows::Win32::Foundation::{BOOL, HANDLE, HWND, LPARAM};
use windows::Win32::Graphics::Gdi::{DEVMODEA, DEVMODEW};
use windows::Win32::Graphics::Printing::PFNPROPSHEETUI;

pub fn get_file_handle() -> (HANDLE, String) {
    (HANDLE(0), "HANDLE(0)".to_string())
}
pub fn get_file_pcwstr() -> (PCWSTR, String) {
    (PCWSTR::default(), "PCWSTR::default()".to_string())
}
pub fn get_file_pwstr() -> (PWSTR, String) {
    (PWSTR::default(), "PWSTR::default()".to_string())
}
pub fn get_file_pstr() -> (PSTR, String) {
    (PSTR::default(), "PSTR::default()".to_string())
}
pub fn get_file_pcstr() -> (PCSTR, String) {
    (PCSTR::default(), "PCSTR::default()".to_string())
}
pub fn get_hwnd() -> (HWND, String) {
    (HWND(0), "HWND(0)".to_string())
}
pub fn get_guid() -> (GUID, String) {
    (GUID::zeroed(), "GUID::zeroed()".to_string())
}

pub fn get_devmodea() -> (DEVMODEA, String) {
    (DEVMODEA::default(), "DEVMODEA::default()".to_string())
}
pub fn get_devmodew() -> (DEVMODEW, String) {
    (DEVMODEW::default(), "DEVMODEW::default()".to_string())
}
pub fn get_pfnpropsheetui() -> (PFNPROPSHEETUI, String) {
    (PFNPROPSHEETUI::default(), "PFNPROPSHEETUI::default()".to_string())
}
pub fn get_lparam() -> (LPARAM, String) {
    (LPARAM::default(), "LPARAM::default()".to_string())
}

pub fn get_foundation_bool() -> (BOOL, String) {
    (BOOL::default(), "BOOL::default()".to_string())
}
