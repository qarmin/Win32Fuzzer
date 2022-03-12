use windows::core::{IUnknown, GUID, HSTRING, PCSTR, PCWSTR, PSTR, PWSTR};
use windows::Win32::Devices::Bluetooth::{AUTHENTICATION_REQUIREMENTS, BLUETOOTH_DEVICE_INFO, BLUETOOTH_RADIO_INFO, BLUETOOTH_SELECT_DEVICE_PARAMS};
use windows::Win32::Devices::DeviceAndDriverInstallation::{CONFIGRET, CONFLICT_DETAILS_A, CONFLICT_DETAILS_W, SETUP_FILE_OPERATION};
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINFO_DATA;
use windows::Win32::Devices::Display::{AR_STATE, BLENDOBJ, BRUSHOBJ, CLIPLINE, CLIPOBJ, SURFOBJ};
use windows::Win32::Devices::Tapi::VARSTRING;
use windows::Win32::Devices::WebServicesOnDevices::IWSDXMLContext;
use windows::Win32::Foundation::DECIMAL;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Foundation::{BOOL, BOOLEAN, BSTR, CHAR, FILETIME, HANDLE, HINSTANCE, HWND, LPARAM, POINT, PSID, RECT, WPARAM};
use windows::Win32::Globalization::{HIMC, HIMCC};
use windows::Win32::Graphics::DirectComposition::{COMPOSITION_FRAME_ID_TYPE, COMPOSITION_FRAME_STATS, COMPOSITION_TARGET_ID, COMPOSITION_TARGET_STATS};
use windows::Win32::Graphics::Gdi::{ABC, ABCFLOAT, ARC_DIRECTION, BACKGROUND_MODE, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION, CDS_TYPE, CFP_ALLOCPROC, CFP_FREEPROC, CFP_REALLOCPROC, COLORADJUSTMENT, CREATE_FONT_PACKAGE_SUBSET_ENCODING, CREATE_FONT_PACKAGE_SUBSET_PLATFORM, CREATE_POLYGON_RGN_MODE, CreatedHDC, HMONITOR};
use windows::Win32::Graphics::Gdi::{DEVMODEA, DEVMODEW, HBITMAP, HDC, HRGN};
use windows::Win32::Graphics::Printing::{ATTRIBUTE_INFO_3, CORE_PRINTER_DRIVERA, CORE_PRINTER_DRIVERW, PFNPROPSHEETUI};
use windows::Win32::Media::Audio::{ACMDRIVERDETAILSA, ACMDRIVERDETAILSW, ACMDRIVERENUMCB, ACMFILTERCHOOSEA, ACMFILTERCHOOSEW, ACMFILTERDETAILSA, ACMFILTERDETAILSW, ACMFILTERENUMCBA, ACMFILTERENUMCBW, ACMFILTERTAGDETAILSA, ACMFILTERTAGDETAILSW, ACMFILTERTAGENUMCBA, ACMFILTERTAGENUMCBW, ACMFORMATCHOOSEA, ACMFORMATCHOOSEW, ACMFORMATDETAILSA, ACMFORMATENUMCBA, ACMFORMATENUMCBW, ACMFORMATTAGDETAILSA, ACMFORMATTAGDETAILSW, ACMFORMATTAGENUMCBA, ACMFORMATTAGENUMCBW, ACMSTREAMHEADER, AUDIO_STREAM_CATEGORY, AUXCAPSA, AUXCAPSW, HACMDRIVER};
use windows::Win32::Media::Audio::HWAVEOUT;
use windows::Win32::Media::Multimedia::{AVIFILEINFOA, AVIFILEINFOW, AVISAVECALLBACK, AVISTREAMINFOA, AVISTREAMINFOW, COMPVARS};
use windows::Win32::NetworkManagement::Snmp::{AsnAny, AsnObjectIdentifier, AsnOctetString};
use windows::Win32::Networking::Ldap::ldap;
use windows::Win32::Networking::WinInet::{APP_CACHE_DOWNLOAD_LIST, APP_CACHE_FINALIZE_STATE, APP_CACHE_GROUP_INFO, APP_CACHE_GROUP_LIST, APP_CACHE_STATE, CACHE_CONFIG, CACHE_OPERATOR};
use windows::Win32::NetworkManagement::IpHelper::ADDRESS_FAMILY;
use windows::Win32::Security::{ACE_FLAGS, ACE_REVISION, ACL, ACL_INFORMATION_CLASS, AUDIT_EVENT_TYPE, CREATE_RESTRICTED_TOKEN_FLAGS};
use windows::Win32::Security::Authorization::{ACCESS_MODE, AUTHZ_ACCESS_CHECK_FLAGS, AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, AUTHZ_ACCESS_REPLY, AUTHZ_AUDIT_EVENT_HANDLE, AUTHZ_AUDIT_EVENT_TYPE_HANDLE, AUTHZ_CLIENT_CONTEXT_HANDLE, AUTHZ_CONTEXT_INFORMATION_CLASS, AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, AUTHZ_RESOURCE_MANAGER_FLAGS, AUTHZ_RESOURCE_MANAGER_HANDLE, AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, AUTHZ_SOURCE_SCHEMA_REGISTRATION};
use windows::Win32::Security::Credentials::{CRED_ENUMERATE_FLAGS, CRED_MARSHAL_TYPE, CRED_PACK_FLAGS, CRED_PROTECTION_TYPE, CREDUI_FLAGS, CREDUIWIN_FLAGS};
use windows::Win32::Security::Cryptography::Catalog::{CATALOG_INFO, CRYPTCAT_OPEN_FLAGS, CRYPTCAT_VERSION, CRYPTCATATTRIBUTE, CRYPTCATCDF, CRYPTCATMEMBER, CRYPTCATSTORE};
use windows::Win32::Security::Cryptography::{CERT_INFO, CERT_QUERY_ENCODING_TYPE};
use windows::Win32::Security::Cryptography::UI::CRYPTUI_WIZ_FLAGS;
use windows::Win32::Security::HDIAGNOSTIC_DATA_QUERY_SESSION;
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::Security::WinTrust::{CRYPT_PROVIDER_DATA, CRYPT_PROVIDER_DEFUSAGE, CRYPT_PROVIDER_FUNCTIONS, CRYPT_PROVIDER_SGNR};
use windows::Win32::Storage::Compression::{COMPRESS_ALGORITHM, COMPRESS_INFORMATION_CLASS, COMPRESSOR_HANDLE};
use windows::Win32::Storage::Packaging::Appx::{AddPackageDependencyOptions, AppPolicyClrCompat, AppPolicyCreateFileAccess, AppPolicyLifecycleManagement, AppPolicyMediaFoundationCodecLoading, AppPolicyProcessTerminationMethod, AppPolicyShowDeveloperDiagnostic, AppPolicyThreadInitializationType, AppPolicyWindowingModel, CreatePackageDependencyOptions};
use windows::Win32::Storage::StructuredStorage::JET_INSTANCE;
use windows::Win32::Storage::StructuredStorage::{JET_SESID, JET_TABLEID};
use windows::Win32::Storage::Vhd::{APPLY_SNAPSHOT_VHDSET_FLAG, ATTACH_VIRTUAL_DISK_FLAG, COMPACT_VIRTUAL_DISK_FLAG, CREATE_VIRTUAL_DISK_FLAG};
use windows::Win32::Storage::Xps::ABORTPROC;
use windows::Win32::System::Antimalware::AMSI_RESULT;
use windows::Win32::System::ApplicationVerifier::AVRF_RESOURCE_ENUMERATE_CALLBACK;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::StructuredStorage::{IPropertyBag, IPropertyStorage};
use windows::Win32::System::Com::{APTTYPE, APTTYPEQUALIFIER, BINDINFO, CALLCONV, CLSCTX, CO_DEVICE_CATALOG_COOKIE, CO_MTA_USAGE_COOKIE, COINIT, COMSD, CUSTDATA, CY};
use windows::Win32::System::Com::VARIANT;
use windows::Win32::System::Com::{IBindCtx, IMalloc, IMoniker, IStream};
use windows::Win32::System::Console::{CHAR_INFO, CONSOLE_CURSOR_INFO, CONSOLE_FONT_INFO, CONSOLE_FONT_INFOEX, CONSOLE_HISTORY_INFO, CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_SCREEN_BUFFER_INFOEX, CONSOLE_SELECTION_INFO, COORD};
use windows::Win32::System::CorrelationVector::CORRELATION_VECTOR;
use windows::Win32::System::DataExchange::CONVINFO;
use windows::Win32::System::Diagnostics::ToolHelp::CREATE_TOOLHELP_SNAPSHOT_FLAGS;
use windows::Win32::System::Performance::PERF_DETAIL;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::REG_SAM_FLAGS;
use windows::Win32::System::StationsAndDesktops::{BROADCAST_SYSTEM_MESSAGE_FLAGS, BROADCAST_SYSTEM_MESSAGE_INFO, BSMINFO};
use windows::Win32::System::Threading::{AVRT_PRIORITY, BoundaryDescriptorHandle, CREATE_EVENT, CREATE_PROCESS_LOGON_FLAGS, LPTHREAD_START_ROUTINE};
use windows::Win32::System::WindowsProgramming::{APPLICATION_RECOVERY_CALLBACK, FH_SERVICE_PIPE_HANDLE};
use windows::Win32::System::WinRT::{AgileReferenceOptions, APARTMENT_SHUTDOWN_REGISTRATION_COOKIE, BSOS_OPTIONS};
use windows::Win32::UI::Accessibility::{ACC_UTILITY_STATE_FLAGS, AsyncContentLoadedState, AutomationIdentifierType, HUIAPATTERNOBJECT};
use windows::Win32::UI::Controls::{BP_BUFFERFORMAT, COMBOBOXINFO, HIMAGELIST};
use windows::Win32::UI::Controls::Dialogs::{CHOOSECOLORA, CHOOSECOLORW, CHOOSEFONTA, CHOOSEFONTW};
use windows::Win32::UI::Input::Ime::{CANDIDATEFORM, CANDIDATELIST, COMPOSITIONFORM};
use windows::Win32::UI::Input::KeyboardAndMouse::ACTIVATE_KEYBOARD_LAYOUT_FLAGS;
use windows::Win32::UI::InteractionContext::{CROSS_SLIDE_THRESHOLD, HINTERACTIONCONTEXT};
use windows::Win32::UI::TabletPC::CHARACTER_RANGE;
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
pub fn get_strange_ABC() -> (ABC, String) { (ABC::default(), "ABC::default()".to_string()) }
pub fn get_strange_ABCFLOAT() -> (ABCFLOAT, String) { (ABCFLOAT::default(), "ABCFLOAT::default()".to_string()) }
pub fn get_strange_ABORTPROC() -> (ABORTPROC, String) { (ABORTPROC::default(), "ABORTPROC::default()".to_string()) }
pub fn get_strange_ACCESS_MODE() -> (ACCESS_MODE, String) { (ACCESS_MODE::default(), "ACCESS_MODE::default()".to_string()) }
pub fn get_strange_ACC_UTILITY_STATE_FLAGS() -> (ACC_UTILITY_STATE_FLAGS, String) { (ACC_UTILITY_STATE_FLAGS::default(), "ACC_UTILITY_STATE_FLAGS::default()".to_string()) }
pub fn get_strange_ACE_FLAGS() -> (ACE_FLAGS, String) { (ACE_FLAGS::default(), "ACE_FLAGS::default()".to_string()) }
pub fn get_strange_ACE_REVISION() -> (ACE_REVISION, String) { (ACE_REVISION::default(), "ACE_REVISION::default()".to_string()) }
pub fn get_strange_ACL_INFORMATION_CLASS() -> (ACL_INFORMATION_CLASS, String) { (ACL_INFORMATION_CLASS::default(), "ACL_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_ACMDRIVERDETAILSA() -> (ACMDRIVERDETAILSA, String) { (ACMDRIVERDETAILSA::default(), "ACMDRIVERDETAILSA::default()".to_string()) }
pub fn get_strange_ACMDRIVERDETAILSW() -> (ACMDRIVERDETAILSW, String) { (ACMDRIVERDETAILSW::default(), "ACMDRIVERDETAILSW::default()".to_string()) }
pub fn get_strange_ACMDRIVERENUMCB() -> (ACMDRIVERENUMCB, String) { (ACMDRIVERENUMCB::default(), "ACMDRIVERENUMCB::default()".to_string()) }
pub fn get_strange_ACMFILTERCHOOSEA() -> (ACMFILTERCHOOSEA, String) { (ACMFILTERCHOOSEA::default(), "ACMFILTERCHOOSEA::default()".to_string()) }
pub fn get_strange_ACMFILTERCHOOSEW() -> (ACMFILTERCHOOSEW, String) { (ACMFILTERCHOOSEW::default(), "ACMFILTERCHOOSEW::default()".to_string()) }
pub fn get_strange_ACMFILTERDETAILSA() -> (ACMFILTERDETAILSA, String) { (ACMFILTERDETAILSA::default(), "ACMFILTERDETAILSA::default()".to_string()) }
pub fn get_strange_ACMFILTERDETAILSW() -> (ACMFILTERDETAILSW, String) { (ACMFILTERDETAILSW::default(), "ACMFILTERDETAILSW::default()".to_string()) }
pub fn get_strange_ACMFILTERENUMCBA() -> (ACMFILTERENUMCBA, String) { (ACMFILTERENUMCBA::default(), "ACMFILTERENUMCBA::default()".to_string()) }
pub fn get_strange_ACMFILTERENUMCBW() -> (ACMFILTERENUMCBW, String) { (ACMFILTERENUMCBW::default(), "ACMFILTERENUMCBW::default()".to_string()) }
pub fn get_strange_ACMFILTERTAGDETAILSA() -> (ACMFILTERTAGDETAILSA, String) { (ACMFILTERTAGDETAILSA::default(), "ACMFILTERTAGDETAILSA::default()".to_string()) }
pub fn get_strange_ACMFILTERTAGDETAILSW() -> (ACMFILTERTAGDETAILSW, String) { (ACMFILTERTAGDETAILSW::default(), "ACMFILTERTAGDETAILSW::default()".to_string()) }
pub fn get_strange_ACMFILTERTAGENUMCBA() -> (ACMFILTERTAGENUMCBA, String) { (ACMFILTERTAGENUMCBA::default(), "ACMFILTERTAGENUMCBA::default()".to_string()) }
pub fn get_strange_ACMFILTERTAGENUMCBW() -> (ACMFILTERTAGENUMCBW, String) { (ACMFILTERTAGENUMCBW::default(), "ACMFILTERTAGENUMCBW::default()".to_string()) }
pub fn get_strange_ACMFORMATCHOOSEA() -> (ACMFORMATCHOOSEA, String) { (ACMFORMATCHOOSEA::default(), "ACMFORMATCHOOSEA::default()".to_string()) }
pub fn get_strange_ACMFORMATCHOOSEW() -> (ACMFORMATCHOOSEW, String) { (ACMFORMATCHOOSEW::default(), "ACMFORMATCHOOSEW::default()".to_string()) }
pub fn get_strange_ACMFORMATDETAILSA() -> (ACMFORMATDETAILSA, String) { (ACMFORMATDETAILSA::default(), "ACMFORMATDETAILSA::default()".to_string()) }
pub fn get_strange_ACMFORMATENUMCBA() -> (ACMFORMATENUMCBA, String) { (ACMFORMATENUMCBA::default(), "ACMFORMATENUMCBA::default()".to_string()) }
pub fn get_strange_ACMFORMATENUMCBW() -> (ACMFORMATENUMCBW, String) { (ACMFORMATENUMCBW::default(), "ACMFORMATENUMCBW::default()".to_string()) }
pub fn get_strange_ACMFORMATTAGDETAILSA() -> (ACMFORMATTAGDETAILSA, String) { (ACMFORMATTAGDETAILSA::default(), "ACMFORMATTAGDETAILSA::default()".to_string()) }
pub fn get_strange_ACMFORMATTAGDETAILSW() -> (ACMFORMATTAGDETAILSW, String) { (ACMFORMATTAGDETAILSW::default(), "ACMFORMATTAGDETAILSW::default()".to_string()) }
pub fn get_strange_ACMFORMATTAGENUMCBA() -> (ACMFORMATTAGENUMCBA, String) { (ACMFORMATTAGENUMCBA::default(), "ACMFORMATTAGENUMCBA::default()".to_string()) }
pub fn get_strange_ACMFORMATTAGENUMCBW() -> (ACMFORMATTAGENUMCBW, String) { (ACMFORMATTAGENUMCBW::default(), "ACMFORMATTAGENUMCBW::default()".to_string()) }
pub fn get_strange_ACMSTREAMHEADER() -> (ACMSTREAMHEADER, String) { (ACMSTREAMHEADER::default(), "ACMSTREAMHEADER::default()".to_string()) }
pub fn get_strange_ACTIVATE_KEYBOARD_LAYOUT_FLAGS() -> (ACTIVATE_KEYBOARD_LAYOUT_FLAGS, String) { (ACTIVATE_KEYBOARD_LAYOUT_FLAGS::default(), "ACTIVATE_KEYBOARD_LAYOUT_FLAGS::default()".to_string()) }
pub fn get_strange_ADDRESS_FAMILY() -> (ADDRESS_FAMILY, String) { (ADDRESS_FAMILY::default(), "ADDRESS_FAMILY::default()".to_string()) }
pub fn get_strange_AMSI_RESULT() -> (AMSI_RESULT, String) { (AMSI_RESULT::default(), "AMSI_RESULT::default()".to_string()) }
pub fn get_strange_APARTMENT_SHUTDOWN_REGISTRATION_COOKIE() -> (APARTMENT_SHUTDOWN_REGISTRATION_COOKIE, String) { (APARTMENT_SHUTDOWN_REGISTRATION_COOKIE::default(), "APARTMENT_SHUTDOWN_REGISTRATION_COOKIE::default()".to_string()) }

pub fn get_strange_APPLICATION_RECOVERY_CALLBACK() -> (APPLICATION_RECOVERY_CALLBACK, String) { (APPLICATION_RECOVERY_CALLBACK::default(), "APPLICATION_RECOVERY_CALLBACK::default()".to_string()) }
pub fn get_strange_APPLY_SNAPSHOT_VHDSET_FLAG() -> (APPLY_SNAPSHOT_VHDSET_FLAG, String) { (APPLY_SNAPSHOT_VHDSET_FLAG::default(), "APPLY_SNAPSHOT_VHDSET_FLAG::default()".to_string()) }
pub fn get_strange_APP_CACHE_DOWNLOAD_LIST() -> (APP_CACHE_DOWNLOAD_LIST, String) { (APP_CACHE_DOWNLOAD_LIST::default(), "APP_CACHE_DOWNLOAD_LIST::default()".to_string()) }
pub fn get_strange_APP_CACHE_FINALIZE_STATE() -> (APP_CACHE_FINALIZE_STATE, String) { (APP_CACHE_FINALIZE_STATE::default(), "APP_CACHE_FINALIZE_STATE::default()".to_string()) }
pub fn get_strange_APP_CACHE_GROUP_INFO() -> (APP_CACHE_GROUP_INFO, String) { (APP_CACHE_GROUP_INFO::default(), "APP_CACHE_GROUP_INFO::default()".to_string()) }
pub fn get_strange_APP_CACHE_GROUP_LIST() -> (APP_CACHE_GROUP_LIST, String) { (APP_CACHE_GROUP_LIST::default(), "APP_CACHE_GROUP_LIST::default()".to_string()) }
pub fn get_strange_APP_CACHE_STATE() -> (APP_CACHE_STATE, String) { (APP_CACHE_STATE::default(), "APP_CACHE_STATE::default()".to_string()) }
pub fn get_strange_APTTYPE() -> (APTTYPE, String) { (APTTYPE::default(), "APTTYPE::default()".to_string()) }
pub fn get_strange_APTTYPEQUALIFIER() -> (APTTYPEQUALIFIER, String) { (APTTYPEQUALIFIER::default(), "APTTYPEQUALIFIER::default()".to_string()) }
pub fn get_strange_ARC_DIRECTION() -> (ARC_DIRECTION, String) { (ARC_DIRECTION::default(), "ARC_DIRECTION::default()".to_string()) }
pub fn get_strange_AR_STATE() -> (AR_STATE, String) { (AR_STATE::default(), "AR_STATE::default()".to_string()) }
pub fn get_strange_ATTACH_VIRTUAL_DISK_FLAG() -> (ATTACH_VIRTUAL_DISK_FLAG, String) { (ATTACH_VIRTUAL_DISK_FLAG::default(), "ATTACH_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_ATTRIBUTE_INFO_3() -> (ATTRIBUTE_INFO_3, String) { (ATTRIBUTE_INFO_3::default(), "ATTRIBUTE_INFO_3::default()".to_string()) }
pub fn get_strange_AUDIO_STREAM_CATEGORY() -> (AUDIO_STREAM_CATEGORY, String) { (AUDIO_STREAM_CATEGORY::default(), "AUDIO_STREAM_CATEGORY::default()".to_string()) }
pub fn get_strange_AUDIT_EVENT_TYPE() -> (AUDIT_EVENT_TYPE, String) { (AUDIT_EVENT_TYPE::default(), "AUDIT_EVENT_TYPE::default()".to_string()) }
pub fn get_strange_AUTHENTICATION_REQUIREMENTS() -> (AUTHENTICATION_REQUIREMENTS, String) { (AUTHENTICATION_REQUIREMENTS::default(), "AUTHENTICATION_REQUIREMENTS::default()".to_string()) }
pub fn get_strange_AUTHZ_ACCESS_CHECK_FLAGS() -> (AUTHZ_ACCESS_CHECK_FLAGS, String) { (AUTHZ_ACCESS_CHECK_FLAGS::default(), "AUTHZ_ACCESS_CHECK_FLAGS::default()".to_string()) }
pub fn get_strange_AUTHZ_ACCESS_CHECK_RESULTS_HANDLE() -> (AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, String) { (AUTHZ_ACCESS_CHECK_RESULTS_HANDLE::default(), "AUTHZ_ACCESS_CHECK_RESULTS_HANDLE::default()".to_string()) }
pub fn get_strange_AUTHZ_ACCESS_REPLY() -> (AUTHZ_ACCESS_REPLY, String) { (AUTHZ_ACCESS_REPLY::default(), "AUTHZ_ACCESS_REPLY::default()".to_string()) }
pub fn get_strange_AUTHZ_AUDIT_EVENT_HANDLE() -> (AUTHZ_AUDIT_EVENT_HANDLE, String) { (AUTHZ_AUDIT_EVENT_HANDLE::default(), "AUTHZ_AUDIT_EVENT_HANDLE::default()".to_string()) }
pub fn get_strange_AUTHZ_AUDIT_EVENT_TYPE_HANDLE() -> (AUTHZ_AUDIT_EVENT_TYPE_HANDLE, String) { (AUTHZ_AUDIT_EVENT_TYPE_HANDLE::default(), "AUTHZ_AUDIT_EVENT_TYPE_HANDLE::default()".to_string()) }
pub fn get_strange_AUTHZ_CLIENT_CONTEXT_HANDLE() -> (AUTHZ_CLIENT_CONTEXT_HANDLE, String) { (AUTHZ_CLIENT_CONTEXT_HANDLE::default(), "AUTHZ_CLIENT_CONTEXT_HANDLE::default()".to_string()) }
pub fn get_strange_AUTHZ_CONTEXT_INFORMATION_CLASS() -> (AUTHZ_CONTEXT_INFORMATION_CLASS, String) { (AUTHZ_CONTEXT_INFORMATION_CLASS::default(), "AUTHZ_CONTEXT_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS() -> (AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, String) { (AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS::default(), "AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS::default()".to_string()) }
pub fn get_strange_AUTHZ_RESOURCE_MANAGER_FLAGS() -> (AUTHZ_RESOURCE_MANAGER_FLAGS, String) { (AUTHZ_RESOURCE_MANAGER_FLAGS::default(), "AUTHZ_RESOURCE_MANAGER_FLAGS::default()".to_string()) }
pub fn get_strange_AUTHZ_RESOURCE_MANAGER_HANDLE() -> (AUTHZ_RESOURCE_MANAGER_HANDLE, String) { (AUTHZ_RESOURCE_MANAGER_HANDLE::default(), "AUTHZ_RESOURCE_MANAGER_HANDLE::default()".to_string()) }
pub fn get_strange_AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE() -> (AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, String) { (AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE::default(), "AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE::default()".to_string()) }
pub fn get_strange_AUTHZ_SOURCE_SCHEMA_REGISTRATION() -> (AUTHZ_SOURCE_SCHEMA_REGISTRATION, String) { (AUTHZ_SOURCE_SCHEMA_REGISTRATION::default(), "AUTHZ_SOURCE_SCHEMA_REGISTRATION::default()".to_string()) }
pub fn get_strange_AUXCAPSA() -> (AUXCAPSA, String) { (AUXCAPSA::default(), "AUXCAPSA::default()".to_string()) }
pub fn get_strange_AUXCAPSW() -> (AUXCAPSW, String) { (AUXCAPSW::default(), "AUXCAPSW::default()".to_string()) }
pub fn get_strange_AVIFILEINFOA() -> (AVIFILEINFOA, String) { (AVIFILEINFOA::default(), "AVIFILEINFOA::default()".to_string()) }
pub fn get_strange_AVIFILEINFOW() -> (AVIFILEINFOW, String) { (AVIFILEINFOW::default(), "AVIFILEINFOW::default()".to_string()) }
pub fn get_strange_AVISAVECALLBACK() -> (AVISAVECALLBACK, String) { (AVISAVECALLBACK::default(), "AVISAVECALLBACK::default()".to_string()) }
pub fn get_strange_AVISTREAMINFOA() -> (AVISTREAMINFOA, String) { (AVISTREAMINFOA::default(), "AVISTREAMINFOA::default()".to_string()) }
pub fn get_strange_AVISTREAMINFOW() -> (AVISTREAMINFOW, String) { (AVISTREAMINFOW::default(), "AVISTREAMINFOW::default()".to_string()) }
pub fn get_strange_AVRF_RESOURCE_ENUMERATE_CALLBACK() -> (AVRF_RESOURCE_ENUMERATE_CALLBACK, String) { (AVRF_RESOURCE_ENUMERATE_CALLBACK::default(), "AVRF_RESOURCE_ENUMERATE_CALLBACK::default()".to_string()) }
pub fn get_strange_AVRT_PRIORITY() -> (AVRT_PRIORITY, String) { (AVRT_PRIORITY::default(), "AVRT_PRIORITY::default()".to_string()) }
pub fn get_strange_AddPackageDependencyOptions() -> (AddPackageDependencyOptions, String) { (AddPackageDependencyOptions::default(), "AddPackageDependencyOptions::default()".to_string()) }
pub fn get_strange_AgileReferenceOptions() -> (AgileReferenceOptions, String) { (AgileReferenceOptions::default(), "AgileReferenceOptions::default()".to_string()) }
pub fn get_strange_AppPolicyClrCompat() -> (AppPolicyClrCompat, String) { (AppPolicyClrCompat::default(), "AppPolicyClrCompat::default()".to_string()) }
pub fn get_strange_AppPolicyCreateFileAccess() -> (AppPolicyCreateFileAccess, String) { (AppPolicyCreateFileAccess::default(), "AppPolicyCreateFileAccess::default()".to_string()) }
pub fn get_strange_AppPolicyLifecycleManagement() -> (AppPolicyLifecycleManagement, String) { (AppPolicyLifecycleManagement::default(), "AppPolicyLifecycleManagement::default()".to_string()) }
pub fn get_strange_AppPolicyMediaFoundationCodecLoading() -> (AppPolicyMediaFoundationCodecLoading, String) { (AppPolicyMediaFoundationCodecLoading::default(), "AppPolicyMediaFoundationCodecLoading::default()".to_string()) }
pub fn get_strange_AppPolicyProcessTerminationMethod() -> (AppPolicyProcessTerminationMethod, String) { (AppPolicyProcessTerminationMethod::default(), "AppPolicyProcessTerminationMethod::default()".to_string()) }
pub fn get_strange_AppPolicyShowDeveloperDiagnostic() -> (AppPolicyShowDeveloperDiagnostic, String) { (AppPolicyShowDeveloperDiagnostic::default(), "AppPolicyShowDeveloperDiagnostic::default()".to_string()) }
pub fn get_strange_AppPolicyThreadInitializationType() -> (AppPolicyThreadInitializationType, String) { (AppPolicyThreadInitializationType::default(), "AppPolicyThreadInitializationType::default()".to_string()) }
pub fn get_strange_AppPolicyWindowingModel() -> (AppPolicyWindowingModel, String) { (AppPolicyWindowingModel::default(), "AppPolicyWindowingModel::default()".to_string()) }
pub fn get_strange_AsnAny() -> (AsnAny, String) { (AsnAny::default(), "AsnAny::default()".to_string()) }
pub fn get_strange_AsnOctetString() -> (AsnOctetString, String) { (AsnOctetString::default(), "AsnOctetString::default()".to_string()) }
pub fn get_strange_AsyncContentLoadedState() -> (AsyncContentLoadedState, String) { (AsyncContentLoadedState::default(), "AsyncContentLoadedState::default()".to_string()) }
pub fn get_strange_AutomationIdentifierType() -> (AutomationIdentifierType, String) { (AutomationIdentifierType::default(), "AutomationIdentifierType::default()".to_string()) }
pub fn get_strange_BACKGROUND_MODE() -> (BACKGROUND_MODE, String) { (BACKGROUND_MODE::default(), "BACKGROUND_MODE::default()".to_string()) }
pub fn get_strange_BINDINFO() -> (BINDINFO, String) { (BINDINFO::default(), "BINDINFO::default()".to_string()) }
pub fn get_strange_BITMAPINFO() -> (BITMAPINFO, String) { (BITMAPINFO::default(), "BITMAPINFO::default()".to_string()) }
pub fn get_strange_BITMAPINFOHEADER() -> (BITMAPINFOHEADER, String) { (BITMAPINFOHEADER::default(), "BITMAPINFOHEADER::default()".to_string()) }
pub fn get_strange_BLENDFUNCTION() -> (BLENDFUNCTION, String) { (BLENDFUNCTION::default(), "BLENDFUNCTION::default()".to_string()) }
pub fn get_strange_BLENDOBJ() -> (BLENDOBJ, String) { (BLENDOBJ::default(), "BLENDOBJ::default()".to_string()) }
pub fn get_strange_BLUETOOTH_DEVICE_INFO() -> (BLUETOOTH_DEVICE_INFO, String) { (BLUETOOTH_DEVICE_INFO::default(), "BLUETOOTH_DEVICE_INFO::default()".to_string()) }
pub fn get_strange_BLUETOOTH_RADIO_INFO() -> (BLUETOOTH_RADIO_INFO, String) { (BLUETOOTH_RADIO_INFO::default(), "BLUETOOTH_RADIO_INFO::default()".to_string()) }
pub fn get_strange_BLUETOOTH_SELECT_DEVICE_PARAMS() -> (BLUETOOTH_SELECT_DEVICE_PARAMS, String) { (BLUETOOTH_SELECT_DEVICE_PARAMS::default(), "BLUETOOTH_SELECT_DEVICE_PARAMS::default()".to_string()) }

pub fn get_strange_BP_BUFFERFORMAT() -> (BP_BUFFERFORMAT, String) { (BP_BUFFERFORMAT::default(), "BP_BUFFERFORMAT::default()".to_string()) }
pub fn get_strange_BROADCAST_SYSTEM_MESSAGE_FLAGS() -> (BROADCAST_SYSTEM_MESSAGE_FLAGS, String) { (BROADCAST_SYSTEM_MESSAGE_FLAGS::default(), "BROADCAST_SYSTEM_MESSAGE_FLAGS::default()".to_string()) }
pub fn get_strange_BROADCAST_SYSTEM_MESSAGE_INFO() -> (BROADCAST_SYSTEM_MESSAGE_INFO, String) { (BROADCAST_SYSTEM_MESSAGE_INFO::default(), "BROADCAST_SYSTEM_MESSAGE_INFO::default()".to_string()) }
pub fn get_strange_BRUSHOBJ() -> (BRUSHOBJ, String) { (BRUSHOBJ::default(), "BRUSHOBJ::default()".to_string()) }
pub fn get_strange_BSMINFO() -> (BSMINFO, String) { (BSMINFO::default(), "BSMINFO::default()".to_string()) }
pub fn get_strange_BSOS_OPTIONS() -> (BSOS_OPTIONS, String) { (BSOS_OPTIONS::default(), "BSOS_OPTIONS::default()".to_string()) }
pub fn get_strange_BoundaryDescriptorHandle() -> (BoundaryDescriptorHandle, String) { (BoundaryDescriptorHandle::default(), "BoundaryDescriptorHandle::default()".to_string()) }
pub fn get_strange_CACHE_CONFIG() -> (CACHE_CONFIG, String) { (CACHE_CONFIG::default(), "CACHE_CONFIG::default()".to_string()) }
pub fn get_strange_CACHE_OPERATOR() -> (CACHE_OPERATOR, String) { (CACHE_OPERATOR::default(), "CACHE_OPERATOR::default()".to_string()) }
pub fn get_strange_CALLCONV() -> (CALLCONV, String) { (CALLCONV::default(), "CALLCONV::default()".to_string()) }
pub fn get_strange_CANDIDATEFORM() -> (CANDIDATEFORM, String) { (CANDIDATEFORM::default(), "CANDIDATEFORM::default()".to_string()) }
pub fn get_strange_CANDIDATELIST() -> (CANDIDATELIST, String) { (CANDIDATELIST::default(), "CANDIDATELIST::default()".to_string()) }
pub fn get_strange_CATALOG_INFO() -> (CATALOG_INFO, String) { (CATALOG_INFO::default(), "CATALOG_INFO::default()".to_string()) }
pub fn get_strange_CDS_TYPE() -> (CDS_TYPE, String) { (CDS_TYPE::default(), "CDS_TYPE::default()".to_string()) }
pub fn get_strange_CERT_INFO() -> (CERT_INFO, String) { (CERT_INFO::default(), "CERT_INFO::default()".to_string()) }
pub fn get_strange_CERT_QUERY_ENCODING_TYPE() -> (CERT_QUERY_ENCODING_TYPE, String) { (CERT_QUERY_ENCODING_TYPE::default(), "CERT_QUERY_ENCODING_TYPE::default()".to_string()) }
pub fn get_strange_CFP_ALLOCPROC() -> (CFP_ALLOCPROC, String) { (CFP_ALLOCPROC::default(), "CFP_ALLOCPROC::default()".to_string()) }
pub fn get_strange_CFP_FREEPROC() -> (CFP_FREEPROC, String) { (CFP_FREEPROC::default(), "CFP_FREEPROC::default()".to_string()) }
pub fn get_strange_CFP_REALLOCPROC() -> (CFP_REALLOCPROC, String) { (CFP_REALLOCPROC::default(), "CFP_REALLOCPROC::default()".to_string()) }
pub fn get_strange_CHARACTER_RANGE() -> (CHARACTER_RANGE, String) { (CHARACTER_RANGE::default(), "CHARACTER_RANGE::default()".to_string()) }
pub fn get_strange_CHAR_INFO() -> (CHAR_INFO, String) { (CHAR_INFO::default(), "CHAR_INFO::default()".to_string()) }
pub fn get_strange_CHOOSECOLORA() -> (CHOOSECOLORA, String) { (CHOOSECOLORA::default(), "CHOOSECOLORA::default()".to_string()) }
pub fn get_strange_CHOOSECOLORW() -> (CHOOSECOLORW, String) { (CHOOSECOLORW::default(), "CHOOSECOLORW::default()".to_string()) }
pub fn get_strange_CHOOSEFONTA() -> (CHOOSEFONTA, String) { (CHOOSEFONTA::default(), "CHOOSEFONTA::default()".to_string()) }
pub fn get_strange_CHOOSEFONTW() -> (CHOOSEFONTW, String) { (CHOOSEFONTW::default(), "CHOOSEFONTW::default()".to_string()) }
pub fn get_strange_CLIPLINE() -> (CLIPLINE, String) { (CLIPLINE::default(), "CLIPLINE::default()".to_string()) }
pub fn get_strange_CLIPOBJ() -> (CLIPOBJ, String) { (CLIPOBJ::default(), "CLIPOBJ::default()".to_string()) }
pub fn get_strange_CLSCTX() -> (CLSCTX, String) { (CLSCTX::default(), "CLSCTX::default()".to_string()) }
pub fn get_strange_COINIT() -> (COINIT, String) { (COINIT::default(), "COINIT::default()".to_string()) }
pub fn get_strange_COLORADJUSTMENT() -> (COLORADJUSTMENT, String) { (COLORADJUSTMENT::default(), "COLORADJUSTMENT::default()".to_string()) }
pub fn get_strange_COMBOBOXINFO() -> (COMBOBOXINFO, String) { (COMBOBOXINFO::default(), "COMBOBOXINFO::default()".to_string()) }
pub fn get_strange_COMPACT_VIRTUAL_DISK_FLAG() -> (COMPACT_VIRTUAL_DISK_FLAG, String) { (COMPACT_VIRTUAL_DISK_FLAG::default(), "COMPACT_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_COMPOSITIONFORM() -> (COMPOSITIONFORM, String) { (COMPOSITIONFORM::default(), "COMPOSITIONFORM::default()".to_string()) }
pub fn get_strange_COMPOSITION_FRAME_ID_TYPE() -> (COMPOSITION_FRAME_ID_TYPE, String) { (COMPOSITION_FRAME_ID_TYPE::default(), "COMPOSITION_FRAME_ID_TYPE::default()".to_string()) }
pub fn get_strange_COMPOSITION_FRAME_STATS() -> (COMPOSITION_FRAME_STATS, String) { (COMPOSITION_FRAME_STATS::default(), "COMPOSITION_FRAME_STATS::default()".to_string()) }
pub fn get_strange_COMPOSITION_TARGET_ID() -> (COMPOSITION_TARGET_ID, String) { (COMPOSITION_TARGET_ID::default(), "COMPOSITION_TARGET_ID::default()".to_string()) }
pub fn get_strange_COMPOSITION_TARGET_STATS() -> (COMPOSITION_TARGET_STATS, String) { (COMPOSITION_TARGET_STATS::default(), "COMPOSITION_TARGET_STATS::default()".to_string()) }
pub fn get_strange_COMPRESSOR_HANDLE() -> (COMPRESSOR_HANDLE, String) { (COMPRESSOR_HANDLE::default(), "COMPRESSOR_HANDLE::default()".to_string()) }
pub fn get_strange_COMPRESS_ALGORITHM() -> (COMPRESS_ALGORITHM, String) { (COMPRESS_ALGORITHM::default(), "COMPRESS_ALGORITHM::default()".to_string()) }
pub fn get_strange_COMPRESS_INFORMATION_CLASS() -> (COMPRESS_INFORMATION_CLASS, String) { (COMPRESS_INFORMATION_CLASS::default(), "COMPRESS_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_COMPVARS() -> (COMPVARS, String) { (COMPVARS::default(), "COMPVARS::default()".to_string()) }
pub fn get_strange_COMSD() -> (COMSD, String) { (COMSD::default(), "COMSD::default()".to_string()) }
pub fn get_strange_CONFIGRET() -> (CONFIGRET, String) { (CONFIGRET::default(), "CONFIGRET::default()".to_string()) }
pub fn get_strange_CONFLICT_DETAILS_A() -> (CONFLICT_DETAILS_A, String) { (CONFLICT_DETAILS_A::default(), "CONFLICT_DETAILS_A::default()".to_string()) }
pub fn get_strange_CONFLICT_DETAILS_W() -> (CONFLICT_DETAILS_W, String) { (CONFLICT_DETAILS_W::default(), "CONFLICT_DETAILS_W::default()".to_string()) }
pub fn get_strange_CONSOLE_CURSOR_INFO() -> (CONSOLE_CURSOR_INFO, String) { (CONSOLE_CURSOR_INFO::default(), "CONSOLE_CURSOR_INFO::default()".to_string()) }
pub fn get_strange_CONSOLE_FONT_INFO() -> (CONSOLE_FONT_INFO, String) { (CONSOLE_FONT_INFO::default(), "CONSOLE_FONT_INFO::default()".to_string()) }
pub fn get_strange_CONSOLE_FONT_INFOEX() -> (CONSOLE_FONT_INFOEX, String) { (CONSOLE_FONT_INFOEX::default(), "CONSOLE_FONT_INFOEX::default()".to_string()) }
pub fn get_strange_CONSOLE_HISTORY_INFO() -> (CONSOLE_HISTORY_INFO, String) { (CONSOLE_HISTORY_INFO::default(), "CONSOLE_HISTORY_INFO::default()".to_string()) }
pub fn get_strange_CONSOLE_MODE() -> (CONSOLE_MODE, String) { (CONSOLE_MODE::default(), "CONSOLE_MODE::default()".to_string()) }
pub fn get_strange_CONSOLE_SCREEN_BUFFER_INFO() -> (CONSOLE_SCREEN_BUFFER_INFO, String) { (CONSOLE_SCREEN_BUFFER_INFO::default(), "CONSOLE_SCREEN_BUFFER_INFO::default()".to_string()) }
pub fn get_strange_CONSOLE_SCREEN_BUFFER_INFOEX() -> (CONSOLE_SCREEN_BUFFER_INFOEX, String) { (CONSOLE_SCREEN_BUFFER_INFOEX::default(), "CONSOLE_SCREEN_BUFFER_INFOEX::default()".to_string()) }
pub fn get_strange_CONSOLE_SELECTION_INFO() -> (CONSOLE_SELECTION_INFO, String) { (CONSOLE_SELECTION_INFO::default(), "CONSOLE_SELECTION_INFO::default()".to_string()) }
pub fn get_strange_CONVINFO() -> (CONVINFO, String) { (CONVINFO::default(), "CONVINFO::default()".to_string()) }
pub fn get_strange_CORE_PRINTER_DRIVERA() -> (CORE_PRINTER_DRIVERA, String) { (CORE_PRINTER_DRIVERA::default(), "CORE_PRINTER_DRIVERA::default()".to_string()) }
pub fn get_strange_CORE_PRINTER_DRIVERW() -> (CORE_PRINTER_DRIVERW, String) { (CORE_PRINTER_DRIVERW::default(), "CORE_PRINTER_DRIVERW::default()".to_string()) }
pub fn get_strange_CORRELATION_VECTOR() -> (CORRELATION_VECTOR, String) { (CORRELATION_VECTOR::default(), "CORRELATION_VECTOR::default()".to_string()) }
pub fn get_strange_CO_DEVICE_CATALOG_COOKIE() -> (CO_DEVICE_CATALOG_COOKIE, String) { (CO_DEVICE_CATALOG_COOKIE::default(), "CO_DEVICE_CATALOG_COOKIE::default()".to_string()) }
pub fn get_strange_CO_MTA_USAGE_COOKIE() -> (CO_MTA_USAGE_COOKIE, String) { (CO_MTA_USAGE_COOKIE::default(), "CO_MTA_USAGE_COOKIE::default()".to_string()) }
pub fn get_strange_CREATE_EVENT() -> (CREATE_EVENT, String) { (CREATE_EVENT::default(), "CREATE_EVENT::default()".to_string()) }
pub fn get_strange_CREATE_FONT_PACKAGE_SUBSET_ENCODING() -> (CREATE_FONT_PACKAGE_SUBSET_ENCODING, String) { (CREATE_FONT_PACKAGE_SUBSET_ENCODING::default(), "CREATE_FONT_PACKAGE_SUBSET_ENCODING::default()".to_string()) }
pub fn get_strange_CREATE_FONT_PACKAGE_SUBSET_PLATFORM() -> (CREATE_FONT_PACKAGE_SUBSET_PLATFORM, String) { (CREATE_FONT_PACKAGE_SUBSET_PLATFORM::default(), "CREATE_FONT_PACKAGE_SUBSET_PLATFORM::default()".to_string()) }
pub fn get_strange_CREATE_POLYGON_RGN_MODE() -> (CREATE_POLYGON_RGN_MODE, String) { (CREATE_POLYGON_RGN_MODE::default(), "CREATE_POLYGON_RGN_MODE::default()".to_string()) }
pub fn get_strange_CREATE_PROCESS_LOGON_FLAGS() -> (CREATE_PROCESS_LOGON_FLAGS, String) { (CREATE_PROCESS_LOGON_FLAGS::default(), "CREATE_PROCESS_LOGON_FLAGS::default()".to_string()) }
pub fn get_strange_CREATE_RESTRICTED_TOKEN_FLAGS() -> (CREATE_RESTRICTED_TOKEN_FLAGS, String) { (CREATE_RESTRICTED_TOKEN_FLAGS::default(), "CREATE_RESTRICTED_TOKEN_FLAGS::default()".to_string()) }
pub fn get_strange_CREATE_TOOLHELP_SNAPSHOT_FLAGS() -> (CREATE_TOOLHELP_SNAPSHOT_FLAGS, String) { (CREATE_TOOLHELP_SNAPSHOT_FLAGS::default(), "CREATE_TOOLHELP_SNAPSHOT_FLAGS::default()".to_string()) }
pub fn get_strange_CREATE_VIRTUAL_DISK_FLAG() -> (CREATE_VIRTUAL_DISK_FLAG, String) { (CREATE_VIRTUAL_DISK_FLAG::default(), "CREATE_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_CREDUIWIN_FLAGS() -> (CREDUIWIN_FLAGS, String) { (CREDUIWIN_FLAGS::default(), "CREDUIWIN_FLAGS::default()".to_string()) }
pub fn get_strange_CREDUI_FLAGS() -> (CREDUI_FLAGS, String) { (CREDUI_FLAGS::default(), "CREDUI_FLAGS::default()".to_string()) }
pub fn get_strange_CRED_ENUMERATE_FLAGS() -> (CRED_ENUMERATE_FLAGS, String) { (CRED_ENUMERATE_FLAGS::default(), "CRED_ENUMERATE_FLAGS::default()".to_string()) }
pub fn get_strange_CRED_MARSHAL_TYPE() -> (CRED_MARSHAL_TYPE, String) { (CRED_MARSHAL_TYPE::default(), "CRED_MARSHAL_TYPE::default()".to_string()) }
pub fn get_strange_CRED_PACK_FLAGS() -> (CRED_PACK_FLAGS, String) { (CRED_PACK_FLAGS::default(), "CRED_PACK_FLAGS::default()".to_string()) }
pub fn get_strange_CRED_PROTECTION_TYPE() -> (CRED_PROTECTION_TYPE, String) { (CRED_PROTECTION_TYPE::default(), "CRED_PROTECTION_TYPE::default()".to_string()) }
pub fn get_strange_CROSS_SLIDE_THRESHOLD() -> (CROSS_SLIDE_THRESHOLD, String) { (CROSS_SLIDE_THRESHOLD::default(), "CROSS_SLIDE_THRESHOLD::default()".to_string()) }
pub fn get_strange_CRYPTCATATTRIBUTE() -> (CRYPTCATATTRIBUTE, String) { (CRYPTCATATTRIBUTE::default(), "CRYPTCATATTRIBUTE::default()".to_string()) }
pub fn get_strange_CRYPTCATCDF() -> (CRYPTCATCDF, String) { (CRYPTCATCDF::default(), "CRYPTCATCDF::default()".to_string()) }
pub fn get_strange_CRYPTCATMEMBER() -> (CRYPTCATMEMBER, String) { (CRYPTCATMEMBER::default(), "CRYPTCATMEMBER::default()".to_string()) }
pub fn get_strange_CRYPTCATSTORE() -> (CRYPTCATSTORE, String) { (CRYPTCATSTORE::default(), "CRYPTCATSTORE::default()".to_string()) }
pub fn get_strange_CRYPTCAT_OPEN_FLAGS() -> (CRYPTCAT_OPEN_FLAGS, String) { (CRYPTCAT_OPEN_FLAGS::default(), "CRYPTCAT_OPEN_FLAGS::default()".to_string()) }
pub fn get_strange_CRYPTCAT_VERSION() -> (CRYPTCAT_VERSION, String) { (CRYPTCAT_VERSION::default(), "CRYPTCAT_VERSION::default()".to_string()) }
pub fn get_strange_CRYPTUI_WIZ_FLAGS() -> (CRYPTUI_WIZ_FLAGS, String) { (CRYPTUI_WIZ_FLAGS::default(), "CRYPTUI_WIZ_FLAGS::default()".to_string()) }
pub fn get_strange_CRYPT_PROVIDER_DATA() -> (CRYPT_PROVIDER_DATA, String) { (CRYPT_PROVIDER_DATA::default(), "CRYPT_PROVIDER_DATA::default()".to_string()) }
pub fn get_strange_CRYPT_PROVIDER_DEFUSAGE() -> (CRYPT_PROVIDER_DEFUSAGE, String) { (CRYPT_PROVIDER_DEFUSAGE::default(), "CRYPT_PROVIDER_DEFUSAGE::default()".to_string()) }
pub fn get_strange_CRYPT_PROVIDER_FUNCTIONS() -> (CRYPT_PROVIDER_FUNCTIONS, String) { (CRYPT_PROVIDER_FUNCTIONS::default(), "CRYPT_PROVIDER_FUNCTIONS::default()".to_string()) }
pub fn get_strange_CRYPT_PROVIDER_SGNR() -> (CRYPT_PROVIDER_SGNR, String) { (CRYPT_PROVIDER_SGNR::default(), "CRYPT_PROVIDER_SGNR::default()".to_string()) }
pub fn get_strange_CUSTDATA() -> (CUSTDATA, String) { (CUSTDATA::default(), "CUSTDATA::default()".to_string()) }
pub fn get_strange_CreatePackageDependencyOptions() -> (CreatePackageDependencyOptions, String) { (CreatePackageDependencyOptions::default(), "CreatePackageDependencyOptions::default()".to_string()) }
pub fn get_strange_CreatedHDC() -> (CreatedHDC, String) { (CreatedHDC::default(), "CreatedHDC::default()".to_string()) }
// ([^\n]+)
// pub fn get_strange_$1\(\) -> \($1, String\) { \($1::default\(\), "$1::default\(\)".to_string\(\)\) }
// pub fn get_strange_ACL() -> (ACL, String) { (ACL::default(), "ACL::default()".to_string()) }

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
