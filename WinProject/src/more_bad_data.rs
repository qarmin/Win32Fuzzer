use windows::core::{IUnknown, GUID, HSTRING, PCSTR, PCWSTR, PSTR, PWSTR};
use windows::Win32::Devices::DeviceAndDriverInstallation::SETUP_FILE_OPERATION;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINFO_DATA;
use windows::Win32::Devices::Display::SURFOBJ;
use windows::Win32::Devices::Tapi::VARSTRING;
use windows::Win32::Devices::WebServicesOnDevices::IWSDXMLContext;
use windows::Win32::Foundation::DECIMAL;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Foundation::{BOOL, BOOLEAN, BSTR, CHAR, FILETIME, HANDLE, HINSTANCE, HWND, LPARAM, POINT, PSID, RECT, WPARAM};
use windows::Win32::Globalization::{HIMC, HIMCC};
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::{DEVMODEA, DEVMODEW, HBITMAP, HDC, HRGN};
use windows::Win32::Graphics::Printing::PFNPROPSHEETUI;
use windows::Win32::Media::Audio::HACMDRIVER;
use windows::Win32::Media::Audio::HWAVEOUT;
use windows::Win32::NetworkManagement::Snmp::AsnObjectIdentifier;
use windows::Win32::Networking::Ldap::ldap;
use windows::Win32::Security::ACL;
use windows::Win32::Security::HDIAGNOSTIC_DATA_QUERY_SESSION;
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::Storage::StructuredStorage::JET_INSTANCE;
use windows::Win32::Storage::StructuredStorage::{JET_SESID, JET_TABLEID};
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::StructuredStorage::{IPropertyBag, IPropertyStorage};
use windows::Win32::System::Com::CY;
use windows::Win32::System::Com::VARIANT;
use windows::Win32::System::Com::{IBindCtx, IMalloc, IMoniker, IStream};
use windows::Win32::System::Console::COORD;
use windows::Win32::System::Performance::PERF_DETAIL;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::REG_SAM_FLAGS;
use windows::Win32::System::Threading::LPTHREAD_START_ROUTINE;
use windows::Win32::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE;
use windows::Win32::UI::Accessibility::HUIAPATTERNOBJECT;
use windows::Win32::UI::Controls::HIMAGELIST;
use windows::Win32::UI::InteractionContext::HINTERACTIONCONTEXT;
use windows::Win32::UI::TextServices::HKL;
use windows::Win32::UI::WindowsAndMessaging::MrmPlatformVersion;
use windows::Win32::UI::WindowsAndMessaging::POINTER_INPUT_TYPE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_LONG_PTR_INDEX;
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

pub fn get_strange_AsnObjectIdentifier() -> (AsnObjectIdentifier, String) {
    (AsnObjectIdentifier::default(), "AsnObjectIdentifier::default()".to_string())
}
pub fn get_strange_BOOL() -> (BOOL, String) {
    (BOOL::default(), "BOOL::default()".to_string())
}
pub fn get_strange_BOOLEAN() -> (BOOLEAN, String) {
    (BOOLEAN::default(), "BOOLEAN::default()".to_string())
}
pub fn get_strange_BSTR() -> (BSTR, String) {
    (BSTR::default(), "BSTR::default()".to_string())
}
pub fn get_strange_CHAR() -> (CHAR, String) {
    (CHAR::default(), "CHAR::default()".to_string())
}
pub fn get_strange_COORD() -> (COORD, String) {
    (COORD::default(), "COORD::default()".to_string())
}
pub fn get_strange_DEVMODEA() -> (DEVMODEA, String) {
    (DEVMODEA::default(), "DEVMODEA::default()".to_string())
}
pub fn get_strange_DEVMODEW() -> (DEVMODEW, String) {
    (DEVMODEW::default(), "DEVMODEW::default()".to_string())
}
pub fn get_strange_FILETIME() -> (FILETIME, String) {
    (FILETIME::default(), "FILETIME::default()".to_string())
}
pub fn get_strange_GUID() -> (GUID, String) {
    (GUID::default(), "GUID::default()".to_string())
}
pub fn get_strange_HANDLE() -> (HANDLE, String) {
    (HANDLE::default(), "HANDLE::default()".to_string())
}
pub fn get_strange_HBITMAP() -> (HBITMAP, String) {
    (HBITMAP::default(), "HBITMAP::default()".to_string())
}
pub fn get_strange_HDC() -> (HDC, String) {
    (HDC::default(), "HDC::default()".to_string())
}
pub fn get_strange_HDIAGNOSTIC_DATA_QUERY_SESSION() -> (HDIAGNOSTIC_DATA_QUERY_SESSION, String) {
    (
        HDIAGNOSTIC_DATA_QUERY_SESSION::default(),
        "HDIAGNOSTIC_DATA_QUERY_SESSION::default()".to_string(),
    )
}
pub fn get_strange_HIMAGELIST() -> (HIMAGELIST, String) {
    (HIMAGELIST::default(), "HIMAGELIST::default()".to_string())
}
pub fn get_strange_HIMC() -> (HIMC, String) {
    (HIMC::default(), "HIMC::default()".to_string())
}
pub fn get_strange_HIMCC() -> (HIMCC, String) {
    (HIMCC::default(), "HIMCC::default()".to_string())
}
pub fn get_strange_HINSTANCE() -> (HINSTANCE, String) {
    (HINSTANCE::default(), "HINSTANCE::default()".to_string())
}
pub fn get_strange_HKEY() -> (HKEY, String) {
    (HKEY::default(), "HKEY::default()".to_string())
}
pub fn get_strange_HKL() -> (HKL, String) {
    (HKL::default(), "HKL::default()".to_string())
}
pub fn get_strange_HMENU() -> (HMENU, String) {
    (HMENU::default(), "HMENU::default()".to_string())
}
pub fn get_strange_HRGN() -> (HRGN, String) {
    (HRGN::default(), "HRGN::default()".to_string())
}
pub fn get_strange_HSTRING() -> (HSTRING, String) {
    (HSTRING::default(), "HSTRING::default()".to_string())
}
pub fn get_strange_HWND() -> (HWND, String) {
    (HWND::default(), "HWND::default()".to_string())
}
pub fn get_strange_JET_SESID() -> (JET_SESID, String) {
    (JET_SESID::default(), "JET_SESID::default()".to_string())
}
pub fn get_strange_JET_TABLEID() -> (JET_TABLEID, String) {
    (JET_TABLEID::default(), "JET_TABLEID::default()".to_string())
}
pub fn get_strange_LPARAM() -> (LPARAM, String) {
    (LPARAM::default(), "LPARAM::default()".to_string())
}
pub fn get_strange_MENU_ITEM_FLAGS() -> (MENU_ITEM_FLAGS, String) {
    (MENU_ITEM_FLAGS::default(), "MENU_ITEM_FLAGS::default()".to_string())
}
pub fn get_strange_PCSTR() -> (PCSTR, String) {
    (PCSTR::default(), "PCSTR::default()".to_string())
}
pub fn get_strange_PCWSTR() -> (PCWSTR, String) {
    (PCWSTR::default(), "PCWSTR::default()".to_string())
}
pub fn get_strange_PFNPROPSHEETUI() -> (PFNPROPSHEETUI, String) {
    (PFNPROPSHEETUI::default(), "PFNPROPSHEETUI::default()".to_string())
}
pub fn get_strange_POINT() -> (POINT, String) {
    (POINT::default(), "POINT::default()".to_string())
}
pub fn get_strange_PSID() -> (PSID, String) {
    (PSID::default(), "PSID::default()".to_string())
}
pub fn get_strange_PSTR() -> (PSTR, String) {
    (PSTR::default(), "PSTR::default()".to_string())
}
pub fn get_strange_PWSTR() -> (PWSTR, String) {
    (PWSTR::default(), "PWSTR::default()".to_string())
}
pub fn get_strange_RECT() -> (RECT, String) {
    (RECT::default(), "RECT::default()".to_string())
}
pub fn get_strange_WPARAM() -> (WPARAM, String) {
    (WPARAM::default(), "WPARAM::default()".to_string())
}
pub fn get_strange_ldap() -> (ldap, String) {
    (ldap::default(), "ldap::default()".to_string())
}
pub fn get_strange_SC_HANDLE() -> (SC_HANDLE, String) {
    (SC_HANDLE::default(), "SC_HANDLE::default()".to_string())
}
pub fn get_strange_JET_INSTANCE() -> (JET_INSTANCE, String) {
    (JET_INSTANCE::default(), "JET_INSTANCE::default()".to_string())
}
pub fn get_strange_HINTERACTIONCONTEXT() -> (HINTERACTIONCONTEXT, String) {
    (HINTERACTIONCONTEXT::default(), "HINTERACTIONCONTEXT::default()".to_string())
}
pub fn get_strange_CY() -> (CY, String) {
    (CY::default(), "CY::default()".to_string())
}
pub fn get_strange_DECIMAL() -> (DECIMAL, String) {
    (DECIMAL::default(), "DECIMAL::default()".to_string())
}
pub fn get_strange_SP_DEVINFO_DATA() -> (SP_DEVINFO_DATA, String) {
    (SP_DEVINFO_DATA::default(), "SP_DEVINFO_DATA::default()".to_string())
}
pub fn get_strange_HACMDRIVER() -> (HACMDRIVER, String) {
    (HACMDRIVER::default(), "HACMDRIVER::default()".to_string())
}
pub fn get_strange_VARIANT() -> (VARIANT, String) {
    (VARIANT::default(), "VARIANT::default()".to_string())
}
pub fn get_strange_HWAVEOUT() -> (HWAVEOUT, String) {
    (HWAVEOUT::default(), "HWAVEOUT::default()".to_string())
}
pub fn get_strange_WINDOW_LONG_PTR_INDEX() -> (WINDOW_LONG_PTR_INDEX, String) {
    (WINDOW_LONG_PTR_INDEX::default(), "WINDOW_LONG_PTR_INDEX::default()".to_string())
}
pub fn get_strange_FH_SERVICE_PIPE_HANDLE() -> (FH_SERVICE_PIPE_HANDLE, String) {
    (FH_SERVICE_PIPE_HANDLE::default(), "FH_SERVICE_PIPE_HANDLE::default()".to_string())
}
pub fn get_strange_LPTHREAD_START_ROUTINE() -> (LPTHREAD_START_ROUTINE, String) {
    (LPTHREAD_START_ROUTINE::default(), "LPTHREAD_START_ROUTINE::default()".to_string())
}
pub fn get_strange_HUIAPATTERNOBJECT() -> (HUIAPATTERNOBJECT, String) {
    (HUIAPATTERNOBJECT::default(), "HUIAPATTERNOBJECT::default()".to_string())
}
pub fn get_strange_SETUP_FILE_OPERATION() -> (SETUP_FILE_OPERATION, String) {
    (SETUP_FILE_OPERATION::default(), "SETUP_FILE_OPERATION::default()".to_string())
}
pub fn get_strange_SIZE() -> (SIZE, String) {
    (SIZE::default(), "SIZE::default()".to_string())
}
pub fn get_strange_PROPVARIANT() -> (PROPVARIANT, String) {
    (PROPVARIANT::default(), "PROPVARIANT::default()".to_string())
}
pub fn get_strange_SURFOBJ() -> (SURFOBJ, String) {
    (SURFOBJ::default(), "SURFOBJ::default()".to_string())
}
pub fn get_strange_VARSTRING() -> (VARSTRING, String) {
    (VARSTRING::default(), "VARSTRING::default()".to_string())
}
pub fn get_strange_REG_SAM_FLAGS() -> (REG_SAM_FLAGS, String) {
    (REG_SAM_FLAGS::default(), "REG_SAM_FLAGS::default()".to_string())
}
pub fn get_strange_HMONITOR() -> (HMONITOR, String) {
    (HMONITOR::default(), "HMONITOR::default()".to_string())
}
pub fn get_strange_ACL() -> (ACL, String) {
    (ACL::default(), "ACL::default()".to_string())
}
pub fn get_strange_PERF_DETAIL() -> (PERF_DETAIL, String) {
    (PERF_DETAIL::default(), "PERF_DETAIL::default()".to_string())
}
pub fn get_strange_POINTER_INPUT_TYPE() -> (POINTER_INPUT_TYPE, String) {
    (POINTER_INPUT_TYPE::default(), "POINTER_INPUT_TYPE::default()".to_string())
}
pub fn get_strange_MrmPlatformVersion() -> (MrmPlatformVersion, String) {
    (MrmPlatformVersion::default(), "MrmPlatformVersion::default()".to_string())
}

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
