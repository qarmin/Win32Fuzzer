use windows::core::{GUID, HSTRING, IUnknown, PCSTR, PCWSTR, PSTR, PWSTR};
use windows::Win32::Devices::WebServicesOnDevices::IWSDXMLContext;
use windows::Win32::Foundation::{BOOL, BOOLEAN, BSTR, CHAR, FILETIME, HANDLE, HINSTANCE, HWND, LPARAM, POINT, PSID, RECT, WPARAM};
use windows::Win32::Globalization::{HIMC, HIMCC};
use windows::Win32::Graphics::Gdi::{DEVMODEA, DEVMODEW, HBITMAP, HDC, HRGN};
use windows::Win32::Graphics::Printing::PFNPROPSHEETUI;
use windows::Win32::Networking::Ldap::ldap;
use windows::Win32::NetworkManagement::Snmp::AsnObjectIdentifier;
use windows::Win32::Security::HDIAGNOSTIC_DATA_QUERY_SESSION;
use windows::Win32::Storage::StructuredStorage::{JET_SESID, JET_TABLEID};
use windows::Win32::System::Com::{IBindCtx, IMalloc, IMoniker, IStream};
use windows::Win32::System::Com::StructuredStorage::{IPropertyBag, IPropertyStorage};
use windows::Win32::System::Console::COORD;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::UI::Controls::HIMAGELIST;
use windows::Win32::UI::TextServices::HKL;
use windows::Win32::UI::WindowsAndMessaging::{HMENU, MENU_ITEM_FLAGS};

pub fn get_hwnd() -> (HWND, String) {
    (HWND(0), "HWND(0)".to_string())
}
pub fn get_guid() -> (GUID, String) {
    (GUID::zeroed(), "GUID::zeroed()".to_string())
}
pub fn get_file_handle() -> (HANDLE, String) {
    (HANDLE(0), "HANDLE(0)".to_string())
}


pub fn get_strange_AsnObjectIdentifier() -> (AsnObjectIdentifier,String) { (AsnObjectIdentifier::default(),"AsnObjectIdentifier::default()".to_string()) }
pub fn get_strange_BOOL() -> (BOOL,String) { (BOOL::default(),"BOOL::default()".to_string()) }
pub fn get_strange_BOOLEAN() -> (BOOLEAN,String) { (BOOLEAN::default(),"BOOLEAN::default()".to_string()) }
pub fn get_strange_BSTR() -> (BSTR,String) { (BSTR::default(),"BSTR::default()".to_string()) }
pub fn get_strange_CHAR() -> (CHAR,String) { (CHAR::default(),"CHAR::default()".to_string()) }
pub fn get_strange_COORD() -> (COORD,String) { (COORD::default(),"COORD::default()".to_string()) }
pub fn get_strange_DEVMODEA() -> (DEVMODEA,String) { (DEVMODEA::default(),"DEVMODEA::default()".to_string()) }
pub fn get_strange_DEVMODEW() -> (DEVMODEW,String) { (DEVMODEW::default(),"DEVMODEW::default()".to_string()) }
pub fn get_strange_FILETIME() -> (FILETIME,String) { (FILETIME::default(),"FILETIME::default()".to_string()) }
pub fn get_strange_GUID() -> (GUID,String) { (GUID::default(),"GUID::default()".to_string()) }
pub fn get_strange_HANDLE() -> (HANDLE,String) { (HANDLE::default(),"HANDLE::default()".to_string()) }
pub fn get_strange_HBITMAP() -> (HBITMAP,String) { (HBITMAP::default(),"HBITMAP::default()".to_string()) }
pub fn get_strange_HDC() -> (HDC,String) { (HDC::default(),"HDC::default()".to_string()) }
pub fn get_strange_HDIAGNOSTIC_DATA_QUERY_SESSION() -> (HDIAGNOSTIC_DATA_QUERY_SESSION,String) { (HDIAGNOSTIC_DATA_QUERY_SESSION::default(),"HDIAGNOSTIC_DATA_QUERY_SESSION::default()".to_string()) }
pub fn get_strange_HIMAGELIST() -> (HIMAGELIST,String) { (HIMAGELIST::default(),"HIMAGELIST::default()".to_string()) }
pub fn get_strange_HIMC() -> (HIMC,String) { (HIMC::default(),"HIMC::default()".to_string()) }
pub fn get_strange_HIMCC() -> (HIMCC,String) { (HIMCC::default(),"HIMCC::default()".to_string()) }
pub fn get_strange_HINSTANCE() -> (HINSTANCE,String) { (HINSTANCE::default(),"HINSTANCE::default()".to_string()) }
pub fn get_strange_HKEY() -> (HKEY,String) { (HKEY::default(),"HKEY::default()".to_string()) }
pub fn get_strange_HKL() -> (HKL,String) { (HKL::default(),"HKL::default()".to_string()) }
pub fn get_strange_HMENU() -> (HMENU,String) { (HMENU::default(),"HMENU::default()".to_string()) }
pub fn get_strange_HRGN() -> (HRGN,String) { (HRGN::default(),"HRGN::default()".to_string()) }
pub fn get_strange_HSTRING() -> (HSTRING,String) { (HSTRING::default(),"HSTRING::default()".to_string()) }
pub fn get_strange_HWND() -> (HWND,String) { (HWND::default(),"HWND::default()".to_string()) }
pub fn get_strange_JET_SESID() -> (JET_SESID,String) { (JET_SESID::default(),"JET_SESID::default()".to_string()) }
pub fn get_strange_JET_TABLEID() -> (JET_TABLEID,String) { (JET_TABLEID::default(),"JET_TABLEID::default()".to_string()) }
pub fn get_strange_LPARAM() -> (LPARAM,String) { (LPARAM::default(),"LPARAM::default()".to_string()) }
pub fn get_strange_MENU_ITEM_FLAGS() -> (MENU_ITEM_FLAGS,String) { (MENU_ITEM_FLAGS::default(),"MENU_ITEM_FLAGS::default()".to_string()) }
pub fn get_strange_PCSTR() -> (PCSTR,String) { (PCSTR::default(),"PCSTR::default()".to_string()) }
pub fn get_strange_PCWSTR() -> (PCWSTR,String) { (PCWSTR::default(),"PCWSTR::default()".to_string()) }
pub fn get_strange_PFNPROPSHEETUI() -> (PFNPROPSHEETUI,String) { (PFNPROPSHEETUI::default(),"PFNPROPSHEETUI::default()".to_string()) }
pub fn get_strange_POINT() -> (POINT,String) { (POINT::default(),"POINT::default()".to_string()) }
pub fn get_strange_PSID() -> (PSID,String) { (PSID::default(),"PSID::default()".to_string()) }
pub fn get_strange_PSTR() -> (PSTR,String) { (PSTR::default(),"PSTR::default()".to_string()) }
pub fn get_strange_PWSTR() -> (PWSTR,String) { (PWSTR::default(),"PWSTR::default()".to_string()) }
pub fn get_strange_RECT() -> (RECT,String) { (RECT::default(),"RECT::default()".to_string()) }
pub fn get_strange_WPARAM() -> (WPARAM,String) { (WPARAM::default(),"WPARAM::default()".to_string()) }
pub fn get_strange_ldap() -> (ldap,String) { (ldap::default(),"ldap::default()".to_string()) }


// pub fn get_strange_IBindCtx() -> (IBindCtx,String) { (IBindCtx::(),"IBindCtx::default()".to_string()) }
// pub fn get_strange_IMFMediaType() -> (IMFMediaType,String) { (IMFMediaType::(),"IMFMediaType::default()".to_string()) }
// pub fn get_strange_IMalloc() -> (IMalloc,String) { (IMalloc::ze(),"IMalloc::default()".to_string()) }
// pub fn get_strange_IMoniker() -> (IMoniker,String) { (IMoniker::default(),"IMoniker::default()".to_string()) }
// pub fn get_strange_IPropertyBag() -> (IPropertyBag,String) { (IPropertyBag::default(),"IPropertyBag::default()".to_string()) }
// pub fn get_strange_IPropertyStorage() -> (IPropertyStorage,String) { (IPropertyStorage::default(),"IPropertyStorage::default()".to_string()) }
// pub fn get_strange_IStream() -> (IStream,String) { (IStream::default(),"IStream::default()".to_string()) }
// pub fn get_strange_IUnknown() -> (IUnknown,String) { (IUnknown::default(),"IUnknown::default()".to_string()) }
// pub fn get_strange_IWSDXMLContext() -> (IWSDXMLContext,String) { (IWSDXMLContext::default(),"IWSDXMLContext::default()".to_string()) }

// AsnObjectIdentifier
// BOOL
// BOOLEAN
// BSTR
// CHAR
// COORD
// DEVMODEA
// DEVMODEW
// FILETIME
// GUID
// HANDLE
// HBITMAP
// HDC
// HDIAGNOSTIC_DATA_QUERY_SESSION
// HIMAGELIST
// HIMC
// HIMCC
// HINSTANCE
// HKEY
// HKL
// HMENU
// HRGN
// HSTRING
// HWND
// IBindCtx
// IMFMediaType
// IMalloc
// IMoniker
// IPropertyBag
// IPropertyStorage
// IStream
// IUnknown
// IWSDXMLContext
// JET_SESID
// JET_TABLEID
// LPARAM
// MENU_ITEM_FLAGS
// PCSTR
// PCWSTR
// PFNPROPSHEETUI
// POINT
// PSID
// PSTR
// PWSTR
// RECT
// WPARAM
// ldap