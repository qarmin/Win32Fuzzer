use windows::core::{GUID, PCSTR, PCWSTR, PSTR};
use windows::Win32::Foundation::{HANDLE, HWND};

pub fn get_file_handle() -> (HANDLE, String){
    (HANDLE(0),"HANDLE(0)".to_string())
}
pub fn get_file_pcwstr() -> (PCWSTR,String){
    (PCWSTR::default(),"PCWSTR::default()".to_string())

}
pub fn get_file_pstr() -> (PSTR,String){
    (PSTR::default(),"PSTR::default()".to_string())
}
pub fn get_file_pcstr() -> (PCSTR,String){
    (PCSTR::default(),"PCSTR::default()".to_string())
}
pub fn get_hwnd() -> (HWND,String){

        (HWND(0),"HWND(0)".to_string())
}
pub fn get_guid() -> (GUID,String){

        (GUID::zeroed(),"GUID::zeroed()".to_string())
}
