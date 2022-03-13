use windows::core::{IUnknown, GUID, HSTRING, PCSTR, PCWSTR, PSTR, PWSTR, HRESULT};
use windows::Win32::AI::MachineLearning::DirectML::{DML_CREATE_DEVICE_FLAGS, DML_FEATURE_LEVEL};
use windows::Win32::Data::RightsManagement::{DRM_ACTSERV_INFO, DRM_CLIENT_VERSION_INFO, DRM_USAGEPOLICY_TYPE, DRMATTESTTYPE, DRMBOUNDLICENSEPARAMS, DRMCALLBACK, DRMENCODINGTYPE, DRMGLOBALOPTIONS, DRMID, DRMSECURITYPROVIDERTYPE, DRMSPECTYPE, DRMTIMETYPE};
use windows::Win32::Devices::Bluetooth::{AUTHENTICATION_REQUIREMENTS, BLUETOOTH_DEVICE_INFO, BLUETOOTH_RADIO_INFO, BLUETOOTH_SELECT_DEVICE_PARAMS};
use windows::Win32::Devices::DeviceAndDriverInstallation::{CONFIGRET, CONFLICT_DETAILS_A, CONFLICT_DETAILS_W, HWProfileInfo_sA, HWProfileInfo_sW, SETUP_FILE_OPERATION};
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINFO_DATA;
use windows::Win32::Devices::DeviceQuery::DEV_OBJECT_TYPE;
use windows::Win32::Devices::Display::{AR_STATE, BLENDOBJ, BRUSHOBJ, CLIPLINE, CLIPOBJ, DHSURF, DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_TOPOLOGY_ID, EMFINFO, ENG_TIME_FIELDS, FONTINFO, FONTOBJ, HSEMAPHORE, HSURF, SURFOBJ};
use windows::Win32::Devices::Enumeration::Pnp::HSWDEVICE;
use windows::Win32::Devices::Properties::{DEVPROPKEY, DEVPROPSTORE};
use windows::Win32::Devices::Tapi::VARSTRING;
use windows::Win32::Devices::WebServicesOnDevices::IWSDXMLContext;
use windows::Win32::Foundation::{DECIMAL, DUPLICATE_HANDLE_OPTIONS, FARPROC, HRSRC};
use windows::Win32::Foundation::SIZE;
use windows::Win32::Foundation::{BOOL, BOOLEAN, BSTR, CHAR, FILETIME, HANDLE, HINSTANCE, HWND, LPARAM, POINT, PSID, RECT, WPARAM};
use windows::Win32::Globalization::{HIMC, HIMCC};
use windows::Win32::Graphics::Direct2D::{D2D1_COLOR_SPACE, D2D1_FACTORY_TYPE};
use windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F;
use windows::Win32::Graphics::Direct3D10::{D3D10_DEVICE_STATE_TYPES, D3D10_DRIVER_TYPE, D3D10_FEATURE_LEVEL1, D3D10_STATE_BLOCK_MASK};
use windows::Win32::Graphics::Direct3D11::{D3D11_CREATE_DEVICE_FLAG, D3DX11_FFT_BUFFER_INFO};
use windows::Win32::Graphics::Direct3D9on12::D3D9ON12_ARGS;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL};
use windows::Win32::Graphics::Direct3D12::D3D_ROOT_SIGNATURE_VERSION;
use windows::Win32::Graphics::Direct3D::Fxc::D3D_BLOB_PART;
use windows::Win32::Graphics::DirectComposition::{COMPOSITION_FRAME_ID_TYPE, COMPOSITION_FRAME_STATS, COMPOSITION_TARGET_ID, COMPOSITION_TARGET_STATS};
use windows::Win32::Graphics::DirectWrite::DWRITE_FACTORY_TYPE;
use windows::Win32::Graphics::Dwm::{DWM_PRESENT_PARAMETERS, DWM_SHOWCONTACT, DWM_TAB_WINDOW_REQUIREMENTS, DWM_TIMING_INFO, DWMTRANSITION_OWNEDWINDOW_TARGET, DWMWINDOWATTRIBUTE};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;
use windows::Win32::Graphics::Gdi::{ABC, ABCFLOAT, ARC_DIRECTION, BACKGROUND_MODE, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION, CDS_TYPE, CFP_ALLOCPROC, CFP_FREEPROC, CFP_REALLOCPROC, COLORADJUSTMENT, CREATE_FONT_PACKAGE_SUBSET_ENCODING, CREATE_FONT_PACKAGE_SUBSET_PLATFORM, CREATE_POLYGON_RGN_MODE, CreatedHDC, DC_LAYOUT, DFC_TYPE, DFCS_STATE, DIB_USAGE, DISPLAY_DEVICEA, DISPLAY_DEVICEW, DRAW_CAPTION_FLAGS, DRAW_EDGE_FLAGS, DRAW_TEXT_FORMAT, DRAWEDGE_FLAGS, DRAWSTATE_FLAGS, DRAWSTATEPROC, EMBED_FONT_CHARSET, EMBEDDED_FONT_PRIV_STATUS, ENHMETAHEADER, ENHMFENUMPROC, ENUM_DISPLAY_SETTINGS_MODE, ETO_OPTIONS, EXT_FLOOD_FILL_TYPE, FONT_CLIP_PRECISION, FONT_LICENSE_PRIVS, FONT_OUTPUT_PRECISION, FONT_PITCH_AND_FAMILY, FONT_QUALITY, FONT_RESOURCE_CHARACTERISTICS, FONTENUMPROCA, FONTENUMPROCW, GCP_RESULTSA, GCP_RESULTSW, HMONITOR};
use windows::Win32::Graphics::Gdi::{DEVMODEA, DEVMODEW, HBITMAP, HDC, HRGN};
use windows::Win32::Graphics::Printing::{ATTRIBUTE_INFO_3, CORE_PRINTER_DRIVERA, CORE_PRINTER_DRIVERW, DEVQUERYPRINT_INFO, EPrintXPSJobOperation, EPrintXPSJobProgress, PFNPROPSHEETUI};
use windows::Win32::Graphics::Printing::PrintTicket::{EDefaultDevmodeType, EPrintTicketScope};
use windows::Win32::Media::Audio::{ACMDRIVERDETAILSA, ACMDRIVERDETAILSW, ACMDRIVERENUMCB, ACMFILTERCHOOSEA, ACMFILTERCHOOSEW, ACMFILTERDETAILSA, ACMFILTERDETAILSW, ACMFILTERENUMCBA, ACMFILTERENUMCBW, ACMFILTERTAGDETAILSA, ACMFILTERTAGDETAILSW, ACMFILTERTAGENUMCBA, ACMFILTERTAGENUMCBW, ACMFORMATCHOOSEA, ACMFORMATCHOOSEW, ACMFORMATDETAILSA, ACMFORMATENUMCBA, ACMFORMATENUMCBW, ACMFORMATTAGDETAILSA, ACMFORMATTAGDETAILSW, ACMFORMATTAGENUMCBA, ACMFORMATTAGENUMCBW, ACMSTREAMHEADER, AUDIO_STREAM_CATEGORY, AUXCAPSA, AUXCAPSW, ERole, HACMDRIVER, HWAVEIN};
use windows::Win32::Media::Audio::HWAVEOUT;
use windows::Win32::Media::DxMediaObjects::{DMO_MEDIA_TYPE, DMO_PARTIAL_MEDIATYPE};
use windows::Win32::Media::Multimedia::{AVIFILEINFOA, AVIFILEINFOW, AVISAVECALLBACK, AVISTREAMINFOA, AVISTREAMINFOW, COMPVARS, DRAWDIBTIME, DRIVERMSGPROC};
use windows::Win32::NetworkManagement::Snmp::{AsnAny, AsnObjectIdentifier, AsnOctetString};
use windows::Win32::Networking::Ldap::{DBGPRINT, ldap};
use windows::Win32::Networking::WinInet::{APP_CACHE_DOWNLOAD_LIST, APP_CACHE_FINALIZE_STATE, APP_CACHE_GROUP_INFO, APP_CACHE_GROUP_LIST, APP_CACHE_STATE, CACHE_CONFIG, CACHE_OPERATOR, FTP_FLAGS, HTTP_ADDREQ_FLAG, HTTP_PUSH_NOTIFICATION_STATUS, HTTP_PUSH_WAIT_HANDLE, HTTP_PUSH_WAIT_TYPE, HTTP_WEB_SOCKET_BUFFER_TYPE};
use windows::Win32::NetworkManagement::Dns::{DNS_APPLICATION_SETTINGS, DNS_CHARSET, DNS_CONFIG_TYPE, DNS_CONNECTION_NAME_LIST, DNS_CONNECTION_POLICY_TAG, DNS_CONNECTION_PROXY_INFO, DNS_CONNECTION_PROXY_INFO_EX, DNS_CONNECTION_PROXY_LIST, DNS_CONNECTION_PROXY_TYPE, DNS_FREE_TYPE, DNS_MESSAGE_BUFFER, DNS_NAME_FORMAT, DNS_PROXY_COMPLETION_ROUTINE, DNS_PROXY_INFORMATION, DNS_QUERY_CANCEL, DNS_QUERY_RESULT, DNS_RECORDA, DNS_SERVICE_CANCEL, DnsContextHandle};
use windows::Win32::NetworkManagement::IpHelper::{ADDRESS_FAMILY, DNS_INTERFACE_SETTINGS, DNS_SETTINGS, FIXED_INFO_W2KSP1};
use windows::Win32::NetworkManagement::NetManagement::FORCE_LEVEL_FLAGS;
use windows::Win32::NetworkManagement::QoS::ENUMERATION_BUFFER;
use windows::Win32::NetworkManagement::WiFi::DOT11_BSS_TYPE;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::{FWPM_CALLOUT_CHANGE_CALLBACK0, FWPM_CONNECTION_CALLBACK0, FWPM_DYNAMIC_KEYWORD_CALLBACK0, FWPM_ENGINE_OPTION, FWPM_FILTER_CHANGE_CALLBACK0, FWPM_NET_EVENT_CALLBACK0, FWPM_NET_EVENT_CALLBACK1, FWPM_NET_EVENT_CALLBACK2, FWPM_NET_EVENT_CALLBACK3, FWPM_NET_EVENT_CALLBACK4, FWPM_PROVIDER_CHANGE_CALLBACK0, FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0, FWPM_SUBLAYER_CHANGE_CALLBACK0, FWPM_SYSTEM_PORTS_CALLBACK0, FWPM_VSWITCH_EVENT_CALLBACK0};
use windows::Win32::Security::{ACE_FLAGS, ACE_REVISION, ACL, ACL_INFORMATION_CLASS, AUDIT_EVENT_TYPE, CREATE_RESTRICTED_TOKEN_FLAGS};
use windows::Win32::Security::Authorization::{ACCESS_MODE, AUTHZ_ACCESS_CHECK_FLAGS, AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, AUTHZ_ACCESS_REPLY, AUTHZ_AUDIT_EVENT_HANDLE, AUTHZ_AUDIT_EVENT_TYPE_HANDLE, AUTHZ_CLIENT_CONTEXT_HANDLE, AUTHZ_CONTEXT_INFORMATION_CLASS, AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, AUTHZ_RESOURCE_MANAGER_FLAGS, AUTHZ_RESOURCE_MANAGER_HANDLE, AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, AUTHZ_SOURCE_SCHEMA_REGISTRATION, EXPLICIT_ACCESS_A, EXPLICIT_ACCESS_W, FN_PROGRESS};
use windows::Win32::Security::Credentials::{CRED_ENUMERATE_FLAGS, CRED_MARSHAL_TYPE, CRED_PACK_FLAGS, CRED_PROTECTION_TYPE, CREDUI_FLAGS, CREDUIWIN_FLAGS};
use windows::Win32::Security::Cryptography::Catalog::{CATALOG_INFO, CRYPTCAT_OPEN_FLAGS, CRYPTCAT_VERSION, CRYPTCATATTRIBUTE, CRYPTCATCDF, CRYPTCATMEMBER, CRYPTCATSTORE};
use windows::Win32::Security::Cryptography::{CERT_INFO, CERT_QUERY_ENCODING_TYPE};
use windows::Win32::Security::Cryptography::UI::CRYPTUI_WIZ_FLAGS;
use windows::Win32::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE;
use windows::Win32::Security::HDIAGNOSTIC_DATA_QUERY_SESSION;
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::Security::WinTrust::{CRYPT_PROVIDER_DATA, CRYPT_PROVIDER_DEFUSAGE, CRYPT_PROVIDER_FUNCTIONS, CRYPT_PROVIDER_SGNR};
use windows::Win32::Storage::Cabinets::{ERF, FDICABINETINFO, FDICREATE_CPU_TYPE};
use windows::Win32::Storage::Compression::{COMPRESS_ALGORITHM, COMPRESS_INFORMATION_CLASS, COMPRESSOR_HANDLE};
use windows::Win32::Storage::DistributedFileSystem::DFS_NAMESPACE_VERSION_ORIGIN;
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::Storage::InstallableFileSystems::{FILTER_INFORMATION_CLASS, FILTER_MESSAGE_HEADER, FILTER_VOLUME_INFORMATION_CLASS, FilterFindHandle, FilterInstanceFindHandle, FilterVolumeFindHandle, FilterVolumeInstanceFindHandle};
use windows::Win32::Storage::Packaging::Appx::{AddPackageDependencyOptions, AppPolicyClrCompat, AppPolicyCreateFileAccess, AppPolicyLifecycleManagement, AppPolicyMediaFoundationCodecLoading, AppPolicyProcessTerminationMethod, AppPolicyShowDeveloperDiagnostic, AppPolicyThreadInitializationType, AppPolicyWindowingModel, CreatePackageDependencyOptions};
use windows::Win32::Storage::StructuredStorage::JET_INSTANCE;
use windows::Win32::Storage::StructuredStorage::{JET_SESID, JET_TABLEID};
use windows::Win32::Storage::Vhd::{APPLY_SNAPSHOT_VHDSET_FLAG, ATTACH_VIRTUAL_DISK_FLAG, COMPACT_VIRTUAL_DISK_FLAG, CREATE_VIRTUAL_DISK_FLAG, DELETE_SNAPSHOT_VHDSET_FLAG, DETACH_VIRTUAL_DISK_FLAG, EXPAND_VIRTUAL_DISK_FLAG, FORK_VIRTUAL_DISK_FLAG};
use windows::Win32::Storage::Xps::{ABORTPROC, DEVICE_CAPABILITIES, DOCINFOW, HPTPROVIDER};
use windows::Win32::System::Antimalware::AMSI_RESULT;
use windows::Win32::System::ApplicationVerifier::AVRF_RESOURCE_ENUMERATE_CALLBACK;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::StructuredStorage::{IPropertyBag, IPropertyStorage};
use windows::Win32::System::Com::{APTTYPE, APTTYPEQUALIFIER, BINDINFO, CALLCONV, CLSCTX, CO_DEVICE_CATALOG_COOKIE, CO_MTA_USAGE_COOKIE, COINIT, COMSD, CUSTDATA, CY, DISPPARAMS, EOLE_AUTHENTICATION_CAPABILITIES, EXCEPINFO, FORMATETC};
use windows::Win32::System::Com::VARIANT;
use windows::Win32::System::Com::{IBindCtx, IMalloc, IMoniker, IStream};
use windows::Win32::System::Console::{CHAR_INFO, CONSOLE_CURSOR_INFO, CONSOLE_FONT_INFO, CONSOLE_FONT_INFOEX, CONSOLE_HISTORY_INFO, CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_SCREEN_BUFFER_INFOEX, CONSOLE_SELECTION_INFO, COORD};
use windows::Win32::System::CorrelationVector::CORRELATION_VECTOR;
use windows::Win32::System::DataExchange::{CONVINFO, DDE_CLIENT_TRANSACTION_TYPE, DDE_ENABLE_CALLBACK_CMD, DDE_INITIALIZE_COMMAND, DDE_NAME_SERVICE_CMD, HSZ};
use windows::Win32::System::Diagnostics::Etw::{DECODING_SOURCE, ETW_PROCESS_HANDLE_INFO_TYPE, EVENT_FIELD_TYPE, EVENT_FILTER_DESCRIPTOR, EVENT_INFO_CLASS, EVENT_INSTANCE_INFO, EVENT_MAP_INFO, EVENT_TRACE_CONTROL, EVENT_TRACE_LOGFILEA, EVENT_TRACE_LOGFILEW, EVENT_TRACE_PROPERTIES};
use windows::Win32::System::Diagnostics::ToolHelp::CREATE_TOOLHELP_SNAPSHOT_FLAGS;
use windows::Win32::System::Environment::{ENCLAVE_IDENTITY, ENCLAVE_INFORMATION, ENCLAVE_SEALING_IDENTITY_POLICY};
use windows::Win32::System::EventLog::{EventLogHandle, EventSourceHandle, EVT_CHANNEL_CONFIG_PROPERTY_ID, EVT_EVENT_METADATA_PROPERTY_ID, EVT_EVENT_PROPERTY_ID, EVT_LOG_PROPERTY_ID, EVT_LOGIN_CLASS, EVT_PUBLISHER_METADATA_PROPERTY_ID, EVT_QUERY_PROPERTY_ID, EVT_SUBSCRIBE_CALLBACK, EVT_VARIANT};
use windows::Win32::System::LibraryLoader::{ENUMRESLANGPROCA, ENUMRESLANGPROCW, ENUMRESNAMEPROCA, ENUMRESNAMEPROCW, ENUMRESTYPEPROCA, ENUMRESTYPEPROCW};
use windows::Win32::System::Ole::FONTDESC;
use windows::Win32::System::Performance::PERF_DETAIL;
use windows::Win32::System::Power::{EFFECTIVE_POWER_MODE_CALLBACK, EXECUTION_STATE};
use windows::Win32::System::ProcessStatus::ENUM_PROCESS_MODULES_EX_FLAGS;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::REG_SAM_FLAGS;
use windows::Win32::System::Services::{ENUM_SERVICE_STATE, ENUM_SERVICE_STATUSA, ENUM_SERVICE_STATUSW, ENUM_SERVICE_TYPE};
use windows::Win32::System::StationsAndDesktops::{BROADCAST_SYSTEM_MESSAGE_FLAGS, BROADCAST_SYSTEM_MESSAGE_INFO, BSMINFO, DESKTOPENUMPROCA, DESKTOPENUMPROCW, HWINSTA};
use windows::Win32::System::Threading::{AVRT_PRIORITY, BoundaryDescriptorHandle, CREATE_EVENT, CREATE_PROCESS_LOGON_FLAGS, LPTHREAD_START_ROUTINE};
use windows::Win32::System::Time::DYNAMIC_TIME_ZONE_INFORMATION;
use windows::Win32::System::WindowsProgramming::{APPLICATION_RECOVERY_CALLBACK, FH_SERVICE_PIPE_HANDLE};
use windows::Win32::System::WinRT::{AgileReferenceOptions, APARTMENT_SHUTDOWN_REGISTRATION_COOKIE, BSOS_OPTIONS, DispatcherQueueOptions, HSTRING_BUFFER, HSTRING_HEADER};
use windows::Win32::UI::Accessibility::{ACC_UTILITY_STATE_FLAGS, AsyncContentLoadedState, AutomationIdentifierType, DockPosition, HUIAEVENT, HUIANODE, HUIAPATTERNOBJECT, HUIATEXTRANGE, HWINEVENTHOOK};
use windows::Win32::UI::Controls::{BP_BUFFERFORMAT, COMBOBOXINFO, DLG_BUTTON_CHECK_STATE, DLG_DIR_LIST_FILE_TYPE, DRAW_THEME_PARENT_BACKGROUND_FLAGS, ENABLE_SCROLL_BAR_ARROWS, FEEDBACK_TYPE, HIMAGELIST, HPROPSHEETPAGE, HSYNTHETICPOINTERDEVICE};
use windows::Win32::UI::Controls::Dialogs::{CHOOSECOLORA, CHOOSECOLORW, CHOOSEFONTA, CHOOSEFONTW, FINDREPLACEA, FINDREPLACEW};
use windows::Win32::UI::HiDpi::{DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, DIALOG_DPI_CHANGE_BEHAVIORS, DPI_AWARENESS_CONTEXT, DPI_HOSTING_BEHAVIOR};
use windows::Win32::UI::Input::Ime::{CANDIDATEFORM, CANDIDATELIST, COMPOSITIONFORM};
use windows::Win32::UI::Input::KeyboardAndMouse::ACTIVATE_KEYBOARD_LAYOUT_FLAGS;
use windows::Win32::UI::InteractionContext::{CROSS_SLIDE_THRESHOLD, HINTERACTIONCONTEXT};
use windows::Win32::UI::TabletPC::{CHARACTER_RANGE, HRECOCONTEXT, HRECOGNIZER, HRECOWORDLIST};
use windows::Win32::UI::TextServices::HKL;
use windows::Win32::UI::WindowsAndMessaging::MrmPlatformVersion;
use windows::Win32::UI::WindowsAndMessaging::POINTER_INPUT_TYPE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_LONG_PTR_INDEX;
use windows::Win32::UI::WindowsAndMessaging::{HMENU, MENU_ITEM_FLAGS};
use windows::Win32::Media::MediaFoundation::EAllocationType;
use windows::Win32::Media::MediaFoundation::DXVAHD_DEVICE_USAGE;
use windows::Win32::Networking::HttpServer::{HTTP_BYTE_RANGE, HTTP_CACHE_POLICY, HTTP_DATA_CHUNK, HTTP_FEATURE_ID, HTTP_INITIALIZE, HTTP_LOG_DATA, HTTP_RECEIVE_HTTP_REQUEST_FLAGS, HTTP_REQUEST_PROPERTY, HTTP_REQUEST_V2, HTTP_RESPONSE_V2, HTTP_SERVER_PROPERTY, HTTP_SERVICE_CONFIG_ID, HTTP_SSL_CLIENT_CERT_INFO, HTTP_VERB, HTTPAPI_VERSION};
use windows::Win32::UI::Shell::DISPLAY_DEVICE_TYPE;
use windows::Win32::System::Diagnostics::Debug::DIGEST_FUNCTION;
use windows::Win32::System::Diagnostics::ProcessSnapshotting::{HPSS, HPSSWALK};
use windows::Win32::System::ErrorReporting::{HREPORT, HREPORTSTORE};
use windows::Win32::System::Iis::{HSE_VERSION_INFO, HTTP_FILTER_CONTEXT, HTTP_FILTER_VERSION};
use windows::Win32::UI::Input::HRAWINPUT;
use windows::Win32::UI::Input::Touch::{GESTURECONFIG, HTOUCHINPUT};
use windows::Win32::UI::Shell::Common::DEVICE_SCALE_FACTOR;
use windows::Win32::AI::MachineLearning::WinML::IMLOperatorRegistry;
use windows::Win32::AI::MachineLearning::WinML::IWinMLRuntime;
use windows::Win32::Data::HtmlHelp::PRIORITY;
use windows::Win32::Devices::Bluetooth::PFN_AUTHENTICATION_CALLBACK;
use windows::Win32::Devices::Bluetooth::PFN_AUTHENTICATION_CALLBACK_EX;
use windows::Win32::Devices::Bluetooth::PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK;
use windows::Win32::Devices::Bluetooth::SDP_ELEMENT_DATA;
use windows::Win32::Devices::DeviceAccess::ICreateDeviceAccessAsync;
use windows::Win32::Devices::DeviceAndDriverInstallation::HCMNOTIFICATION;
use windows::Win32::Devices::DeviceAndDriverInstallation::INFCONTEXT;
use windows::Win32::Devices::DeviceAndDriverInstallation::OEM_SOURCE_MEDIA_TYPE;
use windows::Win32::Devices::DeviceAndDriverInstallation::PCM_NOTIFY_CALLBACK;
use windows::Win32::Devices::DeviceAndDriverInstallation::PNP_VETO_TYPE;
use windows::Win32::Devices::DeviceAndDriverInstallation::PSP_DETSIG_CMPPROC;
use windows::Win32::Devices::DeviceAndDriverInstallation::PSP_FILE_CALLBACK_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::PSP_FILE_CALLBACK_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SETUP_DI_BUILD_DRIVER_DRIVER_TYPE;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_BACKUP_QUEUE_PARAMS_V2_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_BACKUP_QUEUE_PARAMS_V2_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_CLASSIMAGELIST_DATA;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_CLASSINSTALL_HEADER;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_COPY_STYLE;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVICE_INTERFACE_DATA;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVICE_INTERFACE_DETAIL_DATA_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVICE_INTERFACE_DETAIL_DATA_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINFO_LIST_DETAIL_DATA_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINFO_LIST_DETAIL_DATA_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINSTALL_PARAMS_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINSTALL_PARAMS_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DRVINFO_DATA_V2_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DRVINFO_DATA_V2_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DRVINFO_DETAIL_DATA_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DRVINFO_DETAIL_DATA_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_DRVINSTALL_PARAMS;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_INF_INFORMATION;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_INF_SIGNER_INFO_V2_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_INF_SIGNER_INFO_V2_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_ORIGINAL_FILE_INFO_A;
use windows::Win32::Devices::DeviceAndDriverInstallation::SP_ORIGINAL_FILE_INFO_W;
use windows::Win32::Devices::DeviceAndDriverInstallation::SetupFileLogInfo;
use windows::Win32::Devices::DeviceQuery::PDEV_QUERY_RESULT_CALLBACK;
use windows::Win32::Devices::Display::HDEV;
use windows::Win32::Devices::Display::LINEATTRS;
use windows::Win32::Devices::Display::MC_COLOR_TEMPERATURE;
use windows::Win32::Devices::Display::MC_DISPLAY_TECHNOLOGY_TYPE;
use windows::Win32::Devices::Display::MC_DRIVE_TYPE;
use windows::Win32::Devices::Display::MC_GAIN_TYPE;
use windows::Win32::Devices::Display::MC_POSITION_TYPE;
use windows::Win32::Devices::Display::MC_SIZE_TYPE;
use windows::Win32::Devices::Display::MC_TIMING_REPORT;
use windows::Win32::Devices::Display::MC_VCP_CODE_TYPE;
use windows::Win32::Devices::Display::ORIENTATION_PREFERENCE;
use windows::Win32::Devices::Display::PATHDATA;
use windows::Win32::Devices::Display::PATHOBJ;
use windows::Win32::Devices::Display::PHYSICAL_MONITOR;
use windows::Win32::Devices::Display::POINTFIX;
use windows::Win32::Devices::Display::POINTQF;
use windows::Win32::Devices::Display::RECTFX;
use windows::Win32::Devices::Display::STROBJ;
use windows::Win32::Devices::Display::XFORML;
use windows::Win32::Devices::Display::XFORMOBJ;
use windows::Win32::Devices::Display::XLATEOBJ;
use windows::Win32::Devices::Enumeration::Pnp::SW_DEVICE_CREATE_CALLBACK;
use windows::Win32::Devices::Enumeration::Pnp::SW_DEVICE_LIFETIME;
use windows::Win32::Devices::HumanInterfaceDevice::HIDD_ATTRIBUTES;
use windows::Win32::Devices::HumanInterfaceDevice::HIDD_CONFIGURATION;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_BUTTON_ARRAY_DATA;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_BUTTON_CAPS;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_CAPS;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_DATA;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_EXTENDED_ATTRIBUTES;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_KEYBOARD_DIRECTION;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_KEYBOARD_MODIFIER_STATE;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_LINK_COLLECTION_NODE;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_REPORT_TYPE;
use windows::Win32::Devices::HumanInterfaceDevice::HIDP_VALUE_CAPS;
use windows::Win32::Devices::HumanInterfaceDevice::PHIDP_INSERT_SCANCODES;
use windows::Win32::Devices::HumanInterfaceDevice::USAGE_AND_PAGE;
use windows::Win32::Devices::SerialCommunication::HCOMDB;
use windows::Win32::Devices::Tapi::ITnef;
use windows::Win32::Devices::Tapi::LINEADDRESSCAPS;
use windows::Win32::Devices::Tapi::LINEADDRESSSTATUS;
use windows::Win32::Devices::Tapi::LINEAGENTACTIVITYLIST;
use windows::Win32::Devices::Tapi::LINEAGENTCAPS;
use windows::Win32::Devices::Tapi::LINEAGENTGROUPLIST;
use windows::Win32::Devices::Tapi::LINEAGENTINFO;
use windows::Win32::Devices::Tapi::LINEAGENTSESSIONINFO;
use windows::Win32::Devices::Tapi::LINEAGENTSESSIONLIST;
use windows::Win32::Devices::Tapi::LINEAGENTSTATUS;
use windows::Win32::Devices::Tapi::LINECALLBACK;
use windows::Win32::Devices::Tapi::LINECALLINFO;
use windows::Win32::Devices::Tapi::LINECALLLIST;
use windows::Win32::Devices::Tapi::LINECALLSTATUS;
use windows::Win32::Devices::Tapi::LINECOUNTRYLIST;
use windows::Win32::Devices::Tapi::LINEDEVCAPS;
use windows::Win32::Devices::Tapi::LINEDEVSTATUS;
use windows::Win32::Devices::Tapi::LINEEXTENSIONID;
use windows::Win32::Devices::Tapi::LINEINITIALIZEEXPARAMS;
use windows::Win32::Devices::Tapi::LINEMESSAGE;
use windows::Win32::Devices::Tapi::LINEPROVIDERLIST;
use windows::Win32::Devices::Tapi::LINEPROXYREQUEST;
use windows::Win32::Devices::Tapi::LINEPROXYREQUESTLIST;
use windows::Win32::Devices::Tapi::LINEQUEUEINFO;
use windows::Win32::Devices::Tapi::LINEQUEUELIST;
use windows::Win32::Devices::Tapi::LINETRANSLATECAPS;
use windows::Win32::Devices::Tapi::LINETRANSLATEOUTPUT;
use windows::Win32::Devices::Tapi::PHONEBUTTONINFO;
use windows::Win32::Devices::Tapi::PHONECALLBACK;
use windows::Win32::Devices::Tapi::PHONECAPS;
use windows::Win32::Devices::Tapi::PHONEEXTENSIONID;
use windows::Win32::Devices::Tapi::PHONEINITIALIZEEXPARAMS;
use windows::Win32::Devices::Tapi::PHONEMESSAGE;
use windows::Win32::Devices::Tapi::PHONESTATUS;
use windows::Win32::Devices::Usb::USBD_ISO_PACKET_DESCRIPTOR;
use windows::Win32::Devices::Usb::USB_INTERFACE_DESCRIPTOR;
use windows::Win32::Devices::Usb::WINUSB_PIPE_INFORMATION;
use windows::Win32::Devices::Usb::WINUSB_PIPE_INFORMATION_EX;
use windows::Win32::Devices::Usb::WINUSB_SETUP_PACKET;
use windows::Win32::Devices::WebServicesOnDevices::IWSDDeviceHost;
use windows::Win32::Devices::WebServicesOnDevices::IWSDDeviceProxy;
use windows::Win32::Devices::WebServicesOnDevices::IWSDHttpAddress;
use windows::Win32::Devices::WebServicesOnDevices::IWSDHttpMessageParameters;
use windows::Win32::Devices::WebServicesOnDevices::IWSDOutboundAttachment;
use windows::Win32::Devices::WebServicesOnDevices::IWSDUdpAddress;
use windows::Win32::Devices::WebServicesOnDevices::IWSDUdpMessageParameters;
use windows::Win32::Devices::WebServicesOnDevices::IWSDiscoveryProvider;
use windows::Win32::Devices::WebServicesOnDevices::IWSDiscoveryPublisher;
use windows::Win32::Devices::WebServicesOnDevices::WSDXML_ELEMENT;
use windows::Win32::Devices::WebServicesOnDevices::WSDXML_NAME;
use windows::Win32::Foundation::HANDLE_FLAGS;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::LUID;
use windows::Win32::Foundation::NTSTATUS;
use windows::Win32::Foundation::PAPCFUNC;
use windows::Win32::Foundation::POINTL;
use windows::Win32::Foundation::POINTS;
use windows::Win32::Foundation::RECTL;
use windows::Win32::Foundation::SYSTEMTIME;
use windows::Win32::Foundation::UNICODE_STRING;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::Graphics::Direct2D::ID2D1Device;
use windows::Win32::Graphics::Direct2D::ID2D1DeviceContext;
use windows::Win32::Graphics::Direct3D10::ID3D10Device1;
use windows::Win32::Graphics::Direct3D10::ID3D10Device;
use windows::Win32::Graphics::Direct3D10::ID3D10Effect;
use windows::Win32::Graphics::Direct3D10::ID3D10EffectPool;
use windows::Win32::Graphics::Direct3D10::ID3D10ShaderReflection;
use windows::Win32::Graphics::Direct3D10::ID3D10StateBlock;
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext;
use windows::Win32::Graphics::Direct3D11::ID3DX11FFT;
use windows::Win32::Graphics::Direct3D11::ID3DX11Scan;
use windows::Win32::Graphics::Direct3D11::ID3DX11SegmentedScan;
use windows::Win32::Graphics::Direct3D9::IDirect3D9Ex;
use windows::Win32::Graphics::DirectDraw::IDirectDraw;
use windows::Win32::Graphics::DirectDraw::IDirectDrawClipper;
use windows::Win32::Graphics::DirectDraw::LPDDENUMCALLBACKA;
use windows::Win32::Graphics::DirectDraw::LPDDENUMCALLBACKEXA;
use windows::Win32::Graphics::DirectDraw::LPDDENUMCALLBACKEXW;
use windows::Win32::Graphics::DirectDraw::LPDDENUMCALLBACKW;
use windows::Win32::Graphics::Dwm::GESTURE_TYPE;
use windows::Win32::Graphics::Dwm::MilMatrix3x2D;
use windows::Win32::Graphics::Gdi::GET_CHARACTER_PLACEMENT_FLAGS;
use windows::Win32::Graphics::Gdi::GET_DCX_FLAGS;
use windows::Win32::Graphics::Gdi::GET_DEVICE_CAPS_INDEX;
use windows::Win32::Graphics::Gdi::GET_GLYPH_OUTLINE_FORMAT;
use windows::Win32::Graphics::Gdi::GET_STOCK_OBJECT_FLAGS;
use windows::Win32::Graphics::Gdi::GLYPHMETRICS;
use windows::Win32::Graphics::Gdi::GLYPHSET;
use windows::Win32::Graphics::Gdi::GOBJENUMPROC;
use windows::Win32::Graphics::Gdi::GRADIENT_FILL;
use windows::Win32::Graphics::Gdi::GRAPHICS_MODE;
use windows::Win32::Graphics::Gdi::GRAYSTRINGPROC;
use windows::Win32::Graphics::Gdi::HATCH_BRUSH_STYLE;
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::Graphics::Gdi::HDC_MAP_MODE;
use windows::Win32::Graphics::Gdi::HENHMETAFILE;
use windows::Win32::Graphics::Gdi::HGDIOBJ;
use windows::Win32::Graphics::Gdi::HMETAFILE;
use windows::Win32::Graphics::Gdi::HPALETTE;
use windows::Win32::Graphics::Gdi::KERNINGPAIR;
use windows::Win32::Graphics::Gdi::LINEDDAPROC;
use windows::Win32::Graphics::Gdi::LOGFONTA;
use windows::Win32::Graphics::Gdi::LOGFONTW;
use windows::Win32::Graphics::Gdi::MFENUMPROC;
use windows::Win32::Graphics::Gdi::MODIFY_WORLD_TRANSFORM_MODE;
use windows::Win32::Graphics::Gdi::MONITORENUMPROC;
use windows::Win32::Graphics::Gdi::MONITORINFO;
use windows::Win32::Graphics::Gdi::MONITOR_FROM_FLAGS;
use windows::Win32::Graphics::Gdi::OBJ_TYPE;
use windows::Win32::Graphics::Gdi::OUTLINETEXTMETRICA;
use windows::Win32::Graphics::Gdi::OUTLINETEXTMETRICW;
use windows::Win32::Graphics::Gdi::PAINTSTRUCT;
use windows::Win32::Graphics::Gdi::PALETTEENTRY;
use windows::Win32::Graphics::Gdi::PEN_STYLE;
use windows::Win32::Graphics::Gdi::R2_MODE;
use windows::Win32::Graphics::Gdi::RASTERIZER_STATUS;
use windows::Win32::Graphics::Gdi::READEMBEDPROC;
use windows::Win32::Graphics::Gdi::REDRAW_WINDOW_FLAGS;
use windows::Win32::Graphics::Gdi::RGBQUAD;
use windows::Win32::Graphics::Gdi::RGNDATA;
use windows::Win32::Graphics::Gdi::RGN_COMBINE_MODE;
use windows::Win32::Graphics::Gdi::ROP_CODE;
use windows::Win32::Graphics::Gdi::SET_BOUNDS_RECT_FLAGS;
use windows::Win32::Graphics::Gdi::STRETCH_BLT_MODE;
use windows::Win32::Graphics::Gdi::SYSTEM_PALETTE_USE;
use windows::Win32::Graphics::Gdi::TEXTMETRICA;
use windows::Win32::Graphics::Gdi::TEXTMETRICW;
use windows::Win32::Graphics::Gdi::TEXT_ALIGN_OPTIONS;
use windows::Win32::Graphics::Gdi::TRIVERTEX;
use windows::Win32::Graphics::Gdi::TTEMBED_FLAGS;
use windows::Win32::Graphics::Gdi::TTLOAD_EMBEDDED_FONT_STATUS;
use windows::Win32::Graphics::Gdi::WRITEEMBEDPROC;
use windows::Win32::Graphics::Gdi::XFORM;
use windows::Win32::Graphics::Imaging::IWICBitmap;
use windows::Win32::Graphics::Imaging::IWICBitmapSource;
use windows::Win32::Graphics::Imaging::WICSectionAccessLevel;
use windows::Win32::Graphics::OpenGL::GLUnurbs;
use windows::Win32::Graphics::OpenGL::GLUquadric;
use windows::Win32::Graphics::OpenGL::GLUtesselator;
use windows::Win32::Graphics::OpenGL::GLYPHMETRICSFLOAT;
use windows::Win32::Graphics::OpenGL::HGLRC;
use windows::Win32::Graphics::OpenGL::LAYERPLANEDESCRIPTOR;
use windows::Win32::Graphics::OpenGL::PIXELFORMATDESCRIPTOR;
use windows::Win32::Graphics::Printing::IPrintAsyncNotifyChannel;
use windows::Win32::Graphics::Printing::PRINT_EXECUTION_DATA;
use windows::Win32::Graphics::Printing::PrintAsyncNotifyConversationStyle;
use windows::Win32::Graphics::Printing::PrintAsyncNotifyUserFilter;
use windows::Win32::Graphics::Printing::PrintPropertyValue;
use windows::Win32::Media::Audio::DirectSound::IDirectSound8;
use windows::Win32::Media::Audio::DirectSound::IDirectSound;
use windows::Win32::Media::Audio::DirectSound::IDirectSoundBuffer8;
use windows::Win32::Media::Audio::DirectSound::IDirectSoundCapture;
use windows::Win32::Media::Audio::DirectSound::IDirectSoundCaptureBuffer8;
use windows::Win32::Media::Audio::DirectSound::IDirectSoundFullDuplex;
use windows::Win32::Media::Audio::DirectSound::LPDSENUMCALLBACKA;
use windows::Win32::Media::Audio::DirectSound::LPDSENUMCALLBACKW;
use windows::Win32::Media::Audio::HACMDRIVERID;
use windows::Win32::Media::Audio::HACMOBJ;
use windows::Win32::Media::Audio::HACMSTREAM;
use windows::Win32::Media::Audio::HMIDI;
use windows::Win32::Media::Audio::HMIDIIN;
use windows::Win32::Media::Audio::HMIDIOUT;
use windows::Win32::Media::Audio::HMIDISTRM;
use windows::Win32::Media::Audio::HMIXER;
use windows::Win32::Media::Audio::HMIXEROBJ;
use windows::Win32::Media::Audio::IActivateAudioInterfaceAsyncOperation;
use windows::Win32::Media::Audio::IAudioStateMonitor;
use windows::Win32::Media::Audio::IMessageFilter;
use windows::Win32::Media::Audio::MIDIHDR;
use windows::Win32::Media::Audio::MIDIINCAPSA;
use windows::Win32::Media::Audio::MIDIINCAPSW;
use windows::Win32::Media::Audio::MIDIOUTCAPSA;
use windows::Win32::Media::Audio::MIDIOUTCAPSW;
use windows::Win32::Media::Audio::MIDI_WAVE_OPEN_TYPE;
use windows::Win32::Media::Audio::MIXERCAPSA;
use windows::Win32::Media::Audio::MIXERCAPSW;
use windows::Win32::Media::Audio::MIXERCONTROLDETAILS;
use windows::Win32::Media::Audio::MIXERLINEA;
use windows::Win32::Media::Audio::MIXERLINECONTROLSA;
use windows::Win32::Media::Audio::MIXERLINECONTROLSW;
use windows::Win32::Media::Audio::MIXERLINEW;
use windows::Win32::Media::Audio::WAVEFILTER;
use windows::Win32::Media::Audio::WAVEFORMATEX;
use windows::Win32::Media::Audio::WAVEHDR;
use windows::Win32::Media::Audio::WAVEINCAPSA;
use windows::Win32::Media::Audio::WAVEINCAPSW;
use windows::Win32::Media::Audio::WAVEOUTCAPSA;
use windows::Win32::Media::Audio::WAVEOUTCAPSW;
use windows::Win32::Media::Audio::XAudio2::IXAPO;
use windows::Win32::Media::Audio::XAudio2::IXAudio2;
use windows::Win32::Media::Audio::tACMFORMATDETAILSW;
use windows::Win32::Media::DxMediaObjects::IEnumDMO;
use windows::Win32::Media::LPTIMECALLBACK;
use windows::Win32::Media::MMTIME;
use windows::Win32::Media::MediaFoundation::IDXVAHD_Device;
use windows::Win32::Media::MediaFoundation::IDirect3DDeviceManager9;
use windows::Win32::Media::MediaFoundation::IMFASFContentInfo;
use windows::Win32::Media::MediaFoundation::IMFASFIndexer;
use windows::Win32::Media::MediaFoundation::IMFASFMultiplexer;
use windows::Win32::Media::MediaFoundation::IMFASFProfile;
use windows::Win32::Media::MediaFoundation::IMFASFSplitter;
use windows::Win32::Media::MediaFoundation::IMFASFStreamSelector;
use windows::Win32::Media::MediaFoundation::IMFActivate;
use windows::Win32::Media::MediaFoundation::IMFAsyncResult;
use windows::Win32::Media::MediaFoundation::IMFAttributes;
use windows::Win32::Media::MediaFoundation::IMFAudioMediaType;
use windows::Win32::Media::MediaFoundation::IMFByteStream;
use windows::Win32::Media::MediaFoundation::IMFCameraOcclusionStateMonitor;
use windows::Win32::Media::MediaFoundation::IMFCollection;
use windows::Win32::Media::MediaFoundation::IMFContentDecryptorContext;
use windows::Win32::Media::MediaFoundation::IMFContentProtectionDevice;
use windows::Win32::Media::MediaFoundation::IMFDXGIDeviceManager;
use windows::Win32::Media::MediaFoundation::IMFExtendedCameraIntrinsicModel;
use windows::Win32::Media::MediaFoundation::IMFExtendedCameraIntrinsics;
use windows::Win32::Media::MediaFoundation::IMFMediaBuffer;
use windows::Win32::Media::MediaFoundation::IMFMediaEvent;
use windows::Win32::Media::MediaFoundation::IMFMediaEventQueue;
use windows::Win32::Media::MediaFoundation::IMFMediaSession;
use windows::Win32::Media::MediaFoundation::IMFMediaSink;
use windows::Win32::Media::MediaFoundation::IMFMediaSource;
use windows::Win32::Media::MediaFoundation::IMFMediaType;
use windows::Win32::Media::MediaFoundation::IMFMediaTypeHandler;
use windows::Win32::Media::MediaFoundation::IMFNetCredentialCache;
use windows::Win32::Media::MediaFoundation::IMFNetProxyLocator;
use windows::Win32::Media::MediaFoundation::IMFPMPServer;
use windows::Win32::Media::MediaFoundation::IMFPMediaPlayer;
use windows::Win32::Media::MediaFoundation::IMFPluginControl;
use windows::Win32::Media::MediaFoundation::IMFPresentationClock;
use windows::Win32::Media::MediaFoundation::IMFPresentationDescriptor;
use windows::Win32::Media::MediaFoundation::IMFPresentationTimeSource;
use windows::Win32::Media::MediaFoundation::IMFProtectedEnvironmentAccess;
use windows::Win32::Media::MediaFoundation::IMFQualityManager;
use windows::Win32::Media::MediaFoundation::IMFRelativePanelWatcher;
use windows::Win32::Media::MediaFoundation::IMFRemoteDesktopPlugin;
use windows::Win32::Media::MediaFoundation::IMFSample;
use windows::Win32::Media::MediaFoundation::IMFSensorActivityMonitor;
use windows::Win32::Media::MediaFoundation::IMFSensorGroup;
use windows::Win32::Media::MediaFoundation::IMFSensorProfile;
use windows::Win32::Media::MediaFoundation::IMFSensorProfileCollection;
use windows::Win32::Media::MediaFoundation::IMFSensorStream;
use windows::Win32::Media::MediaFoundation::IMFSequencerSource;
use windows::Win32::Media::MediaFoundation::IMFSignedLibrary;
use windows::Win32::Media::MediaFoundation::IMFSinkWriter;
use windows::Win32::Media::MediaFoundation::IMFSourceReader;
use windows::Win32::Media::MediaFoundation::IMFSourceResolver;
use windows::Win32::Media::MediaFoundation::IMFStreamDescriptor;
use windows::Win32::Media::MediaFoundation::IMFSystemId;
use windows::Win32::Media::MediaFoundation::IMFTopoLoader;
use windows::Win32::Media::MediaFoundation::IMFTopology;
use windows::Win32::Media::MediaFoundation::IMFTopologyNode;
use windows::Win32::Media::MediaFoundation::IMFTrackedSample;
use windows::Win32::Media::MediaFoundation::IMFTranscodeProfile;
use windows::Win32::Media::MediaFoundation::IMFTransform;
use windows::Win32::Media::MediaFoundation::IMFVideoMediaType;
use windows::Win32::Media::MediaFoundation::IMFVirtualCamera;
use windows::Win32::Media::MediaFoundation::IOPMVideoOutput;
use windows::Win32::Media::MediaFoundation::MFASYNC_WORKQUEUE_TYPE;
use windows::Win32::Media::MediaFoundation::MFCameraIntrinsic_DistortionModelType;
use windows::Win32::Media::MediaFoundation::MFPERIODICCALLBACK;
use windows::Win32::Media::MediaFoundation::MFP_CREATION_OPTIONS;
use windows::Win32::Media::MediaFoundation::MFStandardVideoFormat;
use windows::Win32::Media::MediaFoundation::MFVIDEOFORMAT;
use windows::Win32::Media::MediaFoundation::MFVideoInterlaceMode;
use windows::Win32::Media::MediaFoundation::MF_FILE_ACCESSMODE;
use windows::Win32::Media::MediaFoundation::MF_FILE_FLAGS;
use windows::Win32::Media::MediaFoundation::MF_FILE_OPENMODE;
use windows::Win32::Media::MediaFoundation::MF_TOPOLOGY_TYPE;
use windows::Win32::Media::MediaFoundation::OPM_HDCP_STATUS;
use windows::Win32::Media::MediaFoundation::OPM_HDCP_TYPE;
use windows::Win32::Media::MediaFoundation::OPM_VIDEO_OUTPUT_SEMANTICS;
use windows::Win32::Media::MediaFoundation::PDXVAHDSW_Plugin;
use windows::Win32::Media::Multimedia::HDRVR;
use windows::Win32::Media::Multimedia::HIC;
use windows::Win32::Media::Multimedia::HMMIO;
use windows::Win32::Media::Multimedia::IAVIFile;
use windows::Win32::Media::Multimedia::IAVIStream;
use windows::Win32::Media::Multimedia::ICINFO;
use windows::Win32::Media::Multimedia::JOYCAPSA;
use windows::Win32::Media::Multimedia::JOYCAPSW;
use windows::Win32::Media::Multimedia::JOYINFO;
use windows::Win32::Media::Multimedia::JOYINFOEX;
use windows::Win32::Media::Multimedia::LPMMIOPROC;
use windows::Win32::Media::Multimedia::LPTASKCALLBACK;
use windows::Win32::Media::Multimedia::MMCKINFO;
use windows::Win32::Media::Multimedia::MMIOINFO;
use windows::Win32::Media::Multimedia::YIELDPROC;
use windows::Win32::Media::TIMECAPS;
use windows::Win32::Media::WindowsMediaFormat::IWMIndexer;
use windows::Win32::Media::WindowsMediaFormat::IWMLicenseBackup;
use windows::Win32::Media::WindowsMediaFormat::IWMMetadataEditor;
use windows::Win32::Media::WindowsMediaFormat::IWMProfileManager;
use windows::Win32::Media::WindowsMediaFormat::IWMReader;
use windows::Win32::Media::WindowsMediaFormat::IWMSyncReader;
use windows::Win32::Media::WindowsMediaFormat::IWMWriter;
use windows::Win32::Media::WindowsMediaFormat::IWMWriterFileSink;
use windows::Win32::Media::WindowsMediaFormat::IWMWriterNetworkSink;
use windows::Win32::Media::WindowsMediaFormat::IWMWriterPushSink;
use windows::Win32::NetworkManagement::Dns::MDNS_QUERY_HANDLE;
use windows::Win32::NetworkManagement::IpHelper::GET_ADAPTERS_ADDRESSES_FLAGS;
use windows::Win32::NetworkManagement::IpHelper::GLOBAL_FILTER;
use windows::Win32::NetworkManagement::IpHelper::HIFTIMESTAMPCHANGE;
use windows::Win32::NetworkManagement::IpHelper::INTERFACE_HARDWARE_CROSSTIMESTAMP;
use windows::Win32::NetworkManagement::IpHelper::INTERFACE_TIMESTAMP_CAPABILITIES;
use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;
use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO;
use windows::Win32::NetworkManagement::IpHelper::IP_INTERFACE_INFO;
use windows::Win32::NetworkManagement::IpHelper::IP_PER_ADAPTER_INFO_W2KSP1;
use windows::Win32::NetworkManagement::IpHelper::IP_UNIDIRECTIONAL_ADAPTER_ADDRESS;
use windows::Win32::NetworkManagement::IpHelper::MIB_ANYCASTIPADDRESS_ROW;
use windows::Win32::NetworkManagement::IpHelper::MIB_ICMP;
use windows::Win32::NetworkManagement::IpHelper::MIB_ICMP_EX_XPSP1;
use windows::Win32::NetworkManagement::IpHelper::MIB_IFROW;
use windows::Win32::NetworkManagement::IpHelper::MIB_IFTABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_IF_ENTRY_LEVEL;
use windows::Win32::NetworkManagement::IpHelper::MIB_IF_ROW2;
use windows::Win32::NetworkManagement::IpHelper::MIB_IF_TABLE_LEVEL;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPADDRTABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPFORWARDROW;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPFORWARDTABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPFORWARD_ROW2;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPINTERFACE_ROW;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPNETTABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPNET_ROW2;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPPATH_ROW;
use windows::Win32::NetworkManagement::IpHelper::MIB_IPSTATS_LH;
use windows::Win32::NetworkManagement::IpHelper::MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES;
use windows::Win32::NetworkManagement::IpHelper::MIB_MULTICASTIPADDRESS_ROW;
use windows::Win32::NetworkManagement::IpHelper::MIB_TCP6TABLE2;
use windows::Win32::NetworkManagement::IpHelper::MIB_TCP6TABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_TCPSTATS2;
use windows::Win32::NetworkManagement::IpHelper::MIB_TCPSTATS_LH;
use windows::Win32::NetworkManagement::IpHelper::MIB_TCPTABLE2;
use windows::Win32::NetworkManagement::IpHelper::MIB_TCPTABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_UDP6TABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_UDPSTATS2;
use windows::Win32::NetworkManagement::IpHelper::MIB_UDPSTATS;
use windows::Win32::NetworkManagement::IpHelper::MIB_UDPTABLE;
use windows::Win32::NetworkManagement::IpHelper::MIB_UNICASTIPADDRESS_ROW;
use windows::Win32::NetworkManagement::IpHelper::NET_LUID_LH;
use windows::Win32::NetworkManagement::IpHelper::PFADDRESSTYPE;
use windows::Win32::NetworkManagement::IpHelper::PFFORWARD_ACTION;
use windows::Win32::NetworkManagement::IpHelper::PF_FILTER_DESCRIPTOR;
use windows::Win32::NetworkManagement::IpHelper::PF_INTERFACE_STATS;
use windows::Win32::NetworkManagement::IpHelper::PF_LATEBIND_INFO;
use windows::Win32::NetworkManagement::IpHelper::PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::PIPFORWARD_CHANGE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::PIPINTERFACE_CHANGE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::PTEREDO_PORT_CHANGE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::PUNICAST_IPADDRESS_CHANGE_CALLBACK;
use windows::Win32::NetworkManagement::IpHelper::TCPIP_OWNER_MODULE_INFO_CLASS;
use windows::Win32::NetworkManagement::IpHelper::TCP_ESTATS_TYPE;
use windows::Win32::NetworkManagement::IpHelper::TCP_TABLE_CLASS;
use windows::Win32::NetworkManagement::IpHelper::UDP_TABLE_CLASS;
use windows::Win32::NetworkManagement::Multicast::MCAST_CLIENT_UID;
use windows::Win32::NetworkManagement::Multicast::MCAST_LEASE_REQUEST;
use windows::Win32::NetworkManagement::Multicast::MCAST_LEASE_RESPONSE;
use windows::Win32::NetworkManagement::Multicast::MCAST_SCOPE_CTX;
use windows::Win32::NetworkManagement::Multicast::MCAST_SCOPE_ENTRY;
use windows::Win32::NetworkManagement::NetBios::NCB;
use windows::Win32::NetworkManagement::NetManagement::HLOG;
use windows::Win32::NetworkManagement::NetManagement::NETSETUP_JOIN_STATUS;
use windows::Win32::NetworkManagement::NetManagement::NETSETUP_NAME_TYPE;
use windows::Win32::NetworkManagement::NetManagement::NETSETUP_PROVISION;
use windows::Win32::NetworkManagement::NetManagement::NET_COMPUTER_NAME_TYPE;
use windows::Win32::NetworkManagement::NetManagement::NET_JOIN_DOMAIN_JOIN_OPTIONS;
use windows::Win32::NetworkManagement::NetManagement::NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS;
use windows::Win32::NetworkManagement::NetManagement::NET_REQUEST_PROVISION_OPTIONS;
use windows::Win32::NetworkManagement::NetManagement::NET_SERVER_TYPE;
use windows::Win32::NetworkManagement::NetManagement::NET_USER_ENUM_FILTER_FLAGS;
use windows::Win32::NetworkManagement::NetManagement::NET_VALIDATE_PASSWORD_TYPE;
use windows::Win32::NetworkManagement::QoS::QOS_NOTIFY_FLOW;
use windows::Win32::NetworkManagement::QoS::QOS_QUERY_FLOW;
use windows::Win32::NetworkManagement::QoS::QOS_SET_FLOW;
use windows::Win32::NetworkManagement::QoS::QOS_TRAFFIC_TYPE;
use windows::Win32::NetworkManagement::QoS::TC_IFC_DESCRIPTOR;
use windows::Win32::NetworkManagement::Snmp::SNMPAPI_CALLBACK;
use windows::Win32::NetworkManagement::Snmp::SNMP_API_TRANSLATE_MODE;
use windows::Win32::NetworkManagement::Snmp::SNMP_ERROR;
use windows::Win32::NetworkManagement::Snmp::SNMP_ERROR_STATUS;
use windows::Win32::NetworkManagement::Snmp::SNMP_GENERICTRAP;
use windows::Win32::NetworkManagement::Snmp::SNMP_LOG;
use windows::Win32::NetworkManagement::Snmp::SNMP_OUTPUT_LOG_TYPE;
use windows::Win32::NetworkManagement::Snmp::SNMP_PDU_TYPE;
use windows::Win32::NetworkManagement::Snmp::SNMP_STATUS;
use windows::Win32::NetworkManagement::Snmp::SnmpVarBind;
use windows::Win32::NetworkManagement::Snmp::SnmpVarBindList;
use windows::Win32::NetworkManagement::Snmp::smiOCTETS;
use windows::Win32::NetworkManagement::Snmp::smiOID;
use windows::Win32::NetworkManagement::Snmp::smiVALUE;
use windows::Win32::NetworkManagement::Snmp::smiVENDORINFO;
use windows::Win32::NetworkManagement::WebDav::PFNDAVAUTHCALLBACK;
use windows::Win32::NetworkManagement::WiFi::WFD_OPEN_SESSION_COMPLETE_CALLBACK;
use windows::Win32::NetworkManagement::WiFi::WLAN_AUTOCONF_OPCODE;
use windows::Win32::NetworkManagement::WiFi::WLAN_FILTER_LIST_TYPE;
use windows::Win32::NetworkManagement::WiFi::WLAN_HOSTED_NETWORK_OPCODE;
use windows::Win32::NetworkManagement::WiFi::WLAN_HOSTED_NETWORK_REASON;
use windows::Win32::NetworkManagement::WiFi::WLAN_IHV_CONTROL_TYPE;
use windows::Win32::NetworkManagement::WiFi::WLAN_INTF_OPCODE;
use windows::Win32::NetworkManagement::WiFi::WLAN_NOTIFICATION_CALLBACK;
use windows::Win32::NetworkManagement::WiFi::WLAN_OPCODE_VALUE_TYPE;
use windows::Win32::NetworkManagement::WiFi::WLAN_SECURABLE_OBJECT;
use windows::Win32::NetworkManagement::WiFi::WLAN_SET_EAPHOST_FLAGS;
use windows::Win32::NetworkManagement::WiFi::WL_DISPLAY_PAGES;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS0;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS1;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATISTICS0;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS0;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS1;
use windows::Win32::Networking::Ldap::LDAPMessage;
use windows::Win32::Networking::Ldap::LDAP_BERVAL;
use windows::Win32::Networking::Ldap::LDAP_TIMEVAL;
use windows::Win32::Networking::Ldap::berelement;
use windows::Win32::Networking::Ldap::ldap_version_info;
use windows::Win32::Networking::Ldap::ldapcontrolA;
use windows::Win32::Networking::Ldap::ldapcontrolW;
use windows::Win32::Networking::Ldap::ldapsearch;
use windows::Win32::Networking::Ldap::ldapvlvinfo;
use windows::Win32::Networking::WebSocket::WEB_SOCKET_ACTION;
use windows::Win32::Networking::WebSocket::WEB_SOCKET_ACTION_QUEUE;
use windows::Win32::Networking::WebSocket::WEB_SOCKET_BUFFER;
use windows::Win32::Networking::WebSocket::WEB_SOCKET_BUFFER_TYPE;
use windows::Win32::Networking::WebSocket::WEB_SOCKET_HANDLE;
use windows::Win32::Networking::WebSocket::WEB_SOCKET_PROPERTY_TYPE;
use windows::Win32::Networking::WinHttp::URL_COMPONENTS;
use windows::Win32::Networking::WinHttp::WINHTTP_ACCESS_TYPE;
use windows::Win32::Networking::WinHttp::WINHTTP_AUTOPROXY_OPTIONS;
use windows::Win32::Networking::WinHttp::WINHTTP_CURRENT_USER_IE_PROXY_CONFIG;
use windows::Win32::Networking::WinHttp::WINHTTP_OPEN_REQUEST_FLAGS;
use windows::Win32::Networking::WinHttp::WINHTTP_PROXY_INFO;
use windows::Win32::Networking::WinHttp::WINHTTP_PROXY_RESULT;
use windows::Win32::Networking::WinHttp::WINHTTP_PROXY_RESULT_EX;
use windows::Win32::Networking::WinHttp::WINHTTP_PROXY_SETTINGS;
use windows::Win32::Networking::WinHttp::WINHTTP_QUERY_CONNECTION_GROUP_RESULT;
use windows::Win32::Networking::WinHttp::WINHTTP_STATUS_CALLBACK;
use windows::Win32::Networking::WinHttp::WINHTTP_WEB_SOCKET_BUFFER_TYPE;
use windows::Win32::Networking::WinHttp::WIN_HTTP_CREATE_URL_FLAGS;
use windows::Win32::Networking::WinInet::GOPHER_ATTRIBUTE_ENUMERATOR;
use windows::Win32::Networking::WinInet::GOPHER_FIND_DATAA;
use windows::Win32::Networking::WinInet::GOPHER_FIND_DATAW;
use windows::Win32::Networking::WinInet::INTERNET_BUFFERSA;
use windows::Win32::Networking::WinInet::INTERNET_BUFFERSW;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_CONFIG_INFOA;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_CONFIG_INFOW;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_CONTAINER_INFOA;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_CONTAINER_INFOW;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_ENTRY_INFOA;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_ENTRY_INFOW;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_GROUP_INFOA;
use windows::Win32::Networking::WinInet::INTERNET_CACHE_GROUP_INFOW;
use windows::Win32::Networking::WinInet::INTERNET_CONNECTION;
use windows::Win32::Networking::WinInet::INTERNET_COOKIE2;
use windows::Win32::Networking::WinInet::LPINTERNET_STATUS_CALLBACK;
use windows::Win32::Networking::WinInet::PROXY_AUTO_DETECT_TYPE;
use windows::Win32::Networking::WinInet::URLCACHE_ENTRY_INFO;
use windows::Win32::Networking::WinInet::URL_CACHE_LIMIT_TYPE;
use windows::Win32::Networking::WinInet::URL_COMPONENTSA;
use windows::Win32::Networking::WinInet::URL_COMPONENTSW;
use windows::Win32::Networking::WinInet::WININET_PROXY_INFO_LIST;
use windows::Win32::Networking::WinInet::WPAD_CACHE_DELETE;
use windows::Win32::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT;
use windows::Win32::Networking::WinSock::SOCKADDR_INET;
use windows::Win32::Networking::WinSock::SOCKET;
use windows::Win32::Security::AppLocker::SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS;
use windows::Win32::Security::AppLocker::SAFER_OBJECT_INFO_CLASS;
use windows::Win32::Security::AppLocker::SAFER_POLICY_INFO_CLASS;
use windows::Win32::Security::Authorization::INHERITED_FROMA;
use windows::Win32::Security::Authorization::INHERITED_FROMW;
use windows::Win32::Security::Authorization::PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS;
use windows::Win32::Security::Authorization::PFN_AUTHZ_DYNAMIC_ACCESS_CHECK;
use windows::Win32::Security::Authorization::PFN_AUTHZ_FREE_DYNAMIC_GROUPS;
use windows::Win32::Security::Authorization::PROG_INVOKE_SETTING;
use windows::Win32::Security::Authorization::SE_OBJECT_TYPE;
use windows::Win32::Security::Authorization::TREE_SEC_INFO;
use windows::Win32::Security::Authorization::TRUSTEE_A;
use windows::Win32::Security::Authorization::TRUSTEE_W;
use windows::Win32::Security::Authorization::UI::SI_PAGE_TYPE;
use windows::Win32::Security::Credentials::KeyCredentialManagerOperationErrorStates;
use windows::Win32::Security::Credentials::KeyCredentialManagerOperationType;
use windows::Win32::Security::Credentials::OPENCARDNAMEA;
use windows::Win32::Security::Credentials::OPENCARDNAMEW;
use windows::Win32::Security::Credentials::OPENCARDNAME_EXA;
use windows::Win32::Security::Credentials::OPENCARDNAME_EXW;
use windows::Win32::Security::Credentials::SCARD_IO_REQUEST;
use windows::Win32::Security::Credentials::SCARD_READERSTATEA;
use windows::Win32::Security::Credentials::SCARD_READERSTATEW;
use windows::Win32::Security::Credentials::SCARD_SCOPE;
use windows::Win32::Security::Credentials::SecHandle;
use windows::Win32::Security::Cryptography::Catalog::PFN_CDF_PARSE_ERROR_CALLBACK;
use windows::Win32::Security::Cryptography::HCERTSTORE;
use windows::Win32::Security::Cryptography::Sip::SIP_ADD_NEWPROVIDER;
use windows::Win32::Security::Cryptography::Sip::SIP_CAP_SET_V3;
use windows::Win32::Security::Cryptography::Sip::SIP_DISPATCH_INFO;
use windows::Win32::Security::Cryptography::Sip::SIP_INDIRECT_DATA;
use windows::Win32::Security::Cryptography::Sip::SIP_SUBJECTINFO;
use windows::Win32::Security::DirectoryServices::PFNREADOBJECTSECURITY;
use windows::Win32::Security::DirectoryServices::PFNWRITEOBJECTSECURITY;
use windows::Win32::Security::LOGON32_LOGON;
use windows::Win32::Security::LOGON32_PROVIDER;
use windows::Win32::Security::LicenseProtection::LicenseProtectionStatus;
use windows::Win32::Security::OBJECT_SECURITY_INFORMATION;
use windows::Win32::Security::OBJECT_TYPE_LIST;
use windows::Win32::Security::PRIVILEGE_SET;
use windows::Win32::Security::QUOTA_LIMITS;
use windows::Win32::Security::SAFER_LEVEL_HANDLE;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::Security::SECURITY_AUTO_INHERIT_FLAGS;
use windows::Win32::Security::SECURITY_DESCRIPTOR;
use windows::Win32::Security::SECURITY_IMPERSONATION_LEVEL;
use windows::Win32::Security::SECURITY_QUALITY_OF_SERVICE;
use windows::Win32::Security::SID_NAME_USE;
use windows::Win32::Security::TOKEN_ACCESS_MASK;
use windows::Win32::Security::TOKEN_GROUPS;
use windows::Win32::Security::TOKEN_INFORMATION_CLASS;
use windows::Win32::Security::TOKEN_PRIVILEGES;
use windows::Win32::Security::TOKEN_TYPE;
use windows::Win32::Security::WELL_KNOWN_SID_TYPE;
use windows::Win32::Security::WinTrust::WINTRUST_DATA;
use windows::Win32::Security::WinTrust::WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION;
use windows::Win32::Security::WinTrust::WINTRUST_POLICY_FLAGS;
use windows::Win32::Security::WinTrust::WIN_CERTIFICATE;
use windows::Win32::Storage::Cabinets::PFNALLOC;
use windows::Win32::Storage::Cabinets::PFNCLOSE;
use windows::Win32::Storage::Cabinets::PFNFCIALLOC;
use windows::Win32::Storage::Cabinets::PFNFCICLOSE;
use windows::Win32::Storage::Cabinets::PFNFCIDELETE;
use windows::Win32::Storage::Cabinets::PFNFCIFILEPLACED;
use windows::Win32::Storage::Cabinets::PFNFCIFREE;
use windows::Win32::Storage::Cabinets::PFNFCIGETNEXTCABINET;
use windows::Win32::Storage::Cabinets::PFNFCIGETOPENINFO;
use windows::Win32::Storage::Cabinets::PFNFCIGETTEMPFILE;
use windows::Win32::Storage::Cabinets::PFNFCIOPEN;
use windows::Win32::Storage::Cabinets::PFNFCIREAD;
use windows::Win32::Storage::Cabinets::PFNFCISEEK;
use windows::Win32::Storage::Cabinets::PFNFCISTATUS;
use windows::Win32::Storage::Cabinets::PFNFCIWRITE;
use windows::Win32::Storage::Cabinets::PFNFDIDECRYPT;
use windows::Win32::Storage::Cabinets::PFNFDINOTIFY;
use windows::Win32::Storage::Cabinets::PFNFREE;
use windows::Win32::Storage::Cabinets::PFNOPEN;
use windows::Win32::Storage::Cabinets::PFNREAD;
use windows::Win32::Storage::Cabinets::PFNSEEK;
use windows::Win32::Storage::Cabinets::PFNWRITE;
use windows::Win32::Storage::FileSystem::WIN32_FIND_DATAA;
use windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW;
use windows::Win32::Storage::InstallableFileSystems::HFILTER;
use windows::Win32::Storage::InstallableFileSystems::HFILTER_INSTANCE;
use windows::Win32::Storage::Jet::JET_CALLBACK;
use windows::Win32::Storage::Jet::JET_COMMIT_ID;
use windows::Win32::Storage::Jet::JET_LOGINFO_A;
use windows::Win32::Storage::Jet::JET_LOGINFO_W;
use windows::Win32::Storage::Jet::JET_LS;
use windows::Win32::Storage::Jet::JET_OSSNAPID;
use windows::Win32::Storage::Jet::JET_PFNREALLOC;
use windows::Win32::Storage::Jet::JET_PFNSTATUS;
use windows::Win32::Storage::Jet::JET_RECORDLIST;
use windows::Win32::Storage::Jet::JET_RECPOS;
use windows::Win32::Storage::Jet::JET_RECSIZE2;
use windows::Win32::Storage::Jet::JET_RECSIZE;
use windows::Win32::Storage::Jet::JET_RETINFO;
use windows::Win32::Storage::Jet::JET_RETRIEVECOLUMN;
use windows::Win32::Storage::Jet::JET_TABLECREATE2_A;
use windows::Win32::Storage::Jet::JET_TABLECREATE2_W;
use windows::Win32::Storage::Jet::JET_TABLECREATE3_A;
use windows::Win32::Storage::Jet::JET_TABLECREATE3_W;
use windows::Win32::Storage::Jet::JET_TABLECREATE4_A;
use windows::Win32::Storage::Jet::JET_TABLECREATE4_W;
use windows::Win32::Storage::Jet::JET_TABLECREATE_A;
use windows::Win32::Storage::Jet::JET_TABLECREATE_W;
use windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION;
use windows::Win32::Storage::Packaging::Appx::PackageDependencyLifetimeKind;
use windows::Win32::Storage::Packaging::Appx::PackageDependencyProcessorArchitectures;
use windows::Win32::Storage::Packaging::Appx::PackageOrigin;
use windows::Win32::Storage::Packaging::Appx::PackagePathType;
use windows::Win32::Storage::StructuredStorage::JET_API_PTR;
use windows::Win32::Storage::StructuredStorage::JET_HANDLE;
use windows::Win32::Storage::Vhd::GET_STORAGE_DEPENDENCY_FLAG;
use windows::Win32::Storage::Vhd::GET_VIRTUAL_DISK_INFO;
use windows::Win32::Storage::Vhd::MERGE_VIRTUAL_DISK_FLAG;
use windows::Win32::Storage::Vhd::MIRROR_VIRTUAL_DISK_FLAG;
use windows::Win32::Storage::Vhd::MODIFY_VHDSET_FLAG;
use windows::Win32::Storage::Vhd::OPEN_VIRTUAL_DISK_FLAG;
use windows::Win32::Storage::Vhd::QUERY_CHANGES_VIRTUAL_DISK_FLAG;
use windows::Win32::Storage::Vhd::QUERY_CHANGES_VIRTUAL_DISK_RANGE;
use windows::Win32::Storage::Vhd::RAW_SCSI_VIRTUAL_DISK_FLAG;
use windows::Win32::Storage::Vhd::RAW_SCSI_VIRTUAL_DISK_RESPONSE;
use windows::Win32::Storage::Vhd::RESIZE_VIRTUAL_DISK_FLAG;
use windows::Win32::Storage::Vhd::STORAGE_DEPENDENCY_INFO;
use windows::Win32::Storage::Vhd::TAKE_SNAPSHOT_VHDSET_FLAG;
use windows::Win32::Storage::Vhd::VIRTUAL_DISK_ACCESS_MASK;
use windows::Win32::Storage::Vhd::VIRTUAL_DISK_PROGRESS;
use windows::Win32::Storage::Vss::IVssExpressWriter;
use windows::Win32::Storage::Xps::PRINT_WINDOW_FLAGS;
use windows::Win32::Storage::Xps::Printing::IXpsPrintJob;
use windows::Win32::Storage::Xps::Printing::IXpsPrintJobStream;
use windows::Win32::System::Antimalware::HAMSICONTEXT;
use windows::Win32::System::Antimalware::HAMSISESSION;
use windows::Win32::System::ApplicationVerifier::VERIFIER_ENUM_RESOURCE_FLAGS;
use windows::Win32::System::ApplicationVerifier::eAvrfResourceTypes;
use windows::Win32::System::Com::IBindStatusCallback;
use windows::Win32::System::Com::IDataAdviseHolder;
use windows::Win32::System::Com::IErrorInfo;
use windows::Win32::System::Com::IRunningObjectTable;
use windows::Win32::System::Com::IUri;
use windows::Win32::System::Com::IUriBuilder;
use windows::Win32::System::Com::MULTI_QI;
use windows::Win32::System::Com::Marshal::IMarshal;
use windows::Win32::System::Com::QUERYCONTEXT;
use windows::Win32::System::Com::RPC_C_AUTHN_LEVEL;
use windows::Win32::System::Com::RPC_C_IMP_LEVEL;
use windows::Win32::System::Com::SAFEARRAY;
use windows::Win32::System::Com::STGMEDIUM;
use windows::Win32::System::Com::SYSKIND;
use windows::Win32::System::Com::StructuredStorage::IFillLockBytes;
use windows::Win32::System::Com::StructuredStorage::ILockBytes;
use windows::Win32::System::Com::StructuredStorage::IPropertySetStorage;
use windows::Win32::System::Com::StructuredStorage::IStorage;
use windows::Win32::System::Com::StructuredStorage::OLESTREAM;
use windows::Win32::System::Com::StructuredStorage::SERIALIZEDPROPERTYVALUE;
use windows::Win32::System::Com::StructuredStorage::STGFMT;
use windows::Win32::System::Com::StructuredStorage::STGM;
use windows::Win32::System::Com::StructuredStorage::STGOPTIONS;
use windows::Win32::System::Com::URI_CREATE_FLAGS;
use windows::Win32::System::Com::Urlmon::IInternetSecurityManager;
use windows::Win32::System::Com::Urlmon::IInternetSession;
use windows::Win32::System::Com::Urlmon::IInternetZoneManager;
use windows::Win32::System::Com::Urlmon::PARSEACTION;
use windows::Win32::System::Com::Urlmon::PSUACTION;
use windows::Win32::System::Com::Urlmon::QUERYOPTION;
use windows::Win32::System::Com::Urlmon::SOFTDISTINFO;
use windows::Win32::System::ComponentServices::IDispenserManager;
use windows::Win32::System::Console::HPCON;
use windows::Win32::System::Console::INPUT_RECORD;
use windows::Win32::System::Console::PHANDLER_ROUTINE;
use windows::Win32::System::Console::SMALL_RECT;
use windows::Win32::System::Console::STD_HANDLE;
use windows::Win32::System::DataExchange::HCONV;
use windows::Win32::System::DataExchange::HCONVLIST;
use windows::Win32::System::DataExchange::HDDEDATA;
use windows::Win32::System::DataExchange::PFNCALLBACK;
use windows::Win32::System::Diagnostics::Debug::IDataModelManager;
use windows::Win32::System::Diagnostics::Debug::IMAGEHLP_GET_TYPE_INFO_PARAMS;
use windows::Win32::System::Diagnostics::Debug::IMAGEHLP_LINE64;
use windows::Win32::System::Diagnostics::Debug::IMAGEHLP_LINEW64;
use windows::Win32::System::Diagnostics::Debug::IMAGEHLP_MODULE64;
use windows::Win32::System::Diagnostics::Debug::IMAGEHLP_MODULEW64;
use windows::Win32::System::Diagnostics::Debug::IMAGEHLP_SYMBOL64;
use windows::Win32::System::Diagnostics::Debug::IMAGE_LOAD_CONFIG_DIRECTORY32;
use windows::Win32::System::Diagnostics::Debug::KNONVOLATILE_CONTEXT_POINTERS;
use windows::Win32::System::Diagnostics::Debug::LDT_ENTRY;
use windows::Win32::System::Diagnostics::Debug::LOADED_IMAGE;
use windows::Win32::System::Diagnostics::Debug::LPCALL_BACK_USER_INTERRUPT_ROUTINE;
use windows::Win32::System::Diagnostics::Debug::LPTOP_LEVEL_EXCEPTION_FILTER;
use windows::Win32::System::Diagnostics::Debug::MINIDUMP_TYPE;
use windows::Win32::System::Diagnostics::Debug::OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS;
use windows::Win32::System::Diagnostics::Debug::PCOGETACTIVATIONSTATE;
use windows::Win32::System::Diagnostics::Debug::PCOGETCALLSTATE;
use windows::Win32::System::Diagnostics::Debug::PDBGHELP_CREATE_USER_DUMP_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PENUMDIRTREE_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PENUMDIRTREE_CALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PENUMLOADED_MODULES_CALLBACK64;
use windows::Win32::System::Diagnostics::Debug::PENUMLOADED_MODULES_CALLBACKW64;
use windows::Win32::System::Diagnostics::Debug::PENUMSOURCEFILETOKENSCALLBACK;
use windows::Win32::System::Diagnostics::Debug::PFINDFILEINPATHCALLBACK;
use windows::Win32::System::Diagnostics::Debug::PFINDFILEINPATHCALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PFIND_DEBUG_FILE_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PFIND_DEBUG_FILE_CALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PFIND_EXE_FILE_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PFIND_EXE_FILE_CALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PFUNCTION_TABLE_ACCESS_ROUTINE64;
use windows::Win32::System::Diagnostics::Debug::PGET_MODULE_BASE_ROUTINE64;
use windows::Win32::System::Diagnostics::Debug::PGET_RUNTIME_FUNCTION_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PIMAGEHLP_STATUS_ROUTINE;
use windows::Win32::System::Diagnostics::Debug::PREAD_PROCESS_MEMORY_ROUTINE64;
use windows::Win32::System::Diagnostics::Debug::PSYMBOL_FUNCENTRY_CALLBACK64;
use windows::Win32::System::Diagnostics::Debug::PSYMBOL_FUNCENTRY_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PSYMBOL_REGISTERED_CALLBACK64;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMERATESYMBOLS_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMERATESYMBOLS_CALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMLINES_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMLINES_CALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMMODULES_CALLBACK64;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMMODULES_CALLBACKW64;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMPROCESSES_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMSOURCEFILES_CALLBACK;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMSOURCEFILES_CALLBACKW;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMSYMBOLS_CALLBACK64;
use windows::Win32::System::Diagnostics::Debug::PSYM_ENUMSYMBOLS_CALLBACK64W;
use windows::Win32::System::Diagnostics::Debug::PTRANSLATE_ADDRESS_ROUTINE64;
use windows::Win32::System::Diagnostics::Debug::PVECTORED_EXCEPTION_HANDLER;
use windows::Win32::System::Diagnostics::Debug::PWAITCHAINCALLBACK;
use windows::Win32::System::Diagnostics::Debug::RTL_VIRTUAL_UNWIND_HANDLER_TYPE;
use windows::Win32::System::Diagnostics::Debug::STACKFRAME64;
use windows::Win32::System::Diagnostics::Debug::STACKFRAME_EX;
use windows::Win32::System::Diagnostics::Debug::SYMBOL_INFO;
use windows::Win32::System::Diagnostics::Debug::SYMBOL_INFOW;
use windows::Win32::System::Diagnostics::Debug::SYMSRV_INDEX_INFO;
use windows::Win32::System::Diagnostics::Debug::SYMSRV_INDEX_INFOW;
use windows::Win32::System::Diagnostics::Debug::SYM_FIND_ID_OPTION;
use windows::Win32::System::Diagnostics::Debug::SYM_LOAD_FLAGS;
use windows::Win32::System::Diagnostics::Debug::SYM_SRV_STORE_FILE_FLAGS;
use windows::Win32::System::Diagnostics::Debug::THREAD_ERROR_MODE;
use windows::Win32::System::Diagnostics::Debug::UNWIND_HISTORY_TABLE;
use windows::Win32::System::Diagnostics::Debug::WAITCHAIN_NODE_INFO;
use windows::Win32::System::Diagnostics::Debug::WAIT_CHAIN_THREAD_OPTIONS;
use windows::Win32::System::Diagnostics::Debug::WOW64_CONTEXT;
use windows::Win32::System::Diagnostics::Debug::WOW64_LDT_ENTRY;
use windows::Win32::System::Diagnostics::Etw::PENABLECALLBACK;
use windows::Win32::System::Diagnostics::Etw::PEVENT_CALLBACK;
use windows::Win32::System::Diagnostics::Etw::PROVIDER_ENUMERATION_INFO;
use windows::Win32::System::Diagnostics::Etw::PROVIDER_EVENT_INFO;
use windows::Win32::System::Diagnostics::Etw::PROVIDER_FIELD_INFOARRAY;
use windows::Win32::System::Diagnostics::Etw::TDH_CONTEXT;
use windows::Win32::System::Diagnostics::Etw::TDH_HANDLE;
use windows::Win32::System::Diagnostics::Etw::TRACE_EVENT_INFO;
use windows::Win32::System::Diagnostics::Etw::TRACE_MESSAGE_FLAGS;
use windows::Win32::System::Diagnostics::Etw::TRACE_QUERY_INFO_CLASS;
use windows::Win32::System::Diagnostics::Etw::WMIDPREQUEST;
use windows::Win32::System::Diagnostics::ProcessSnapshotting::PSS_CAPTURE_FLAGS;
use windows::Win32::System::Diagnostics::ProcessSnapshotting::PSS_DUPLICATE_FLAGS;
use windows::Win32::System::Diagnostics::ProcessSnapshotting::PSS_QUERY_INFORMATION_CLASS;
use windows::Win32::System::Diagnostics::ProcessSnapshotting::PSS_WALK_INFORMATION_CLASS;
use windows::Win32::System::Diagnostics::ToolHelp::HEAPENTRY32;
use windows::Win32::System::Diagnostics::ToolHelp::HEAPLIST32;
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32W;
use windows::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32;
use windows::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32W;
use windows::Win32::System::Diagnostics::ToolHelp::THREADENTRY32;
use windows::Win32::System::ErrorReporting::REPORT_STORE_TYPES;
use windows::Win32::System::ErrorReporting::WER_CONSENT;
use windows::Win32::System::ErrorReporting::WER_DUMP_TYPE;
use windows::Win32::System::ErrorReporting::WER_FAULT_REPORTING;
use windows::Win32::System::ErrorReporting::WER_FILE;
use windows::Win32::System::ErrorReporting::WER_FILE_TYPE;
use windows::Win32::System::ErrorReporting::WER_REGISTER_FILE_TYPE;
use windows::Win32::System::ErrorReporting::WER_REPORT_METADATA_V1;
use windows::Win32::System::ErrorReporting::WER_REPORT_METADATA_V2;
use windows::Win32::System::ErrorReporting::WER_REPORT_METADATA_V3;
use windows::Win32::System::ErrorReporting::WER_REPORT_TYPE;
use windows::Win32::System::ErrorReporting::WER_REPORT_UI;
use windows::Win32::System::ErrorReporting::WER_SUBMIT_FLAGS;
use windows::Win32::System::ErrorReporting::WER_SUBMIT_RESULT;
use windows::Win32::System::EventLog::READ_EVENT_LOG_READ_FLAGS;
use windows::Win32::System::EventLog::REPORT_EVENT_TYPE;
use windows::Win32::System::EventNotificationService::QOCINFO;
use windows::Win32::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE;
use windows::Win32::System::IO::OVERLAPPED;
use windows::Win32::System::IO::OVERLAPPED_ENTRY;
use windows::Win32::System::JobObjects::JOBOBJECTINFOCLASS;
use windows::Win32::System::Kernel::PROCESSOR_NUMBER;
use windows::Win32::System::Kernel::SLIST_ENTRY;
use windows::Win32::System::Kernel::SLIST_HEADER;
use windows::Win32::System::LibraryLoader::LOAD_LIBRARY_FLAGS;
use windows::Win32::System::Ole::ICreateErrorInfo;
use windows::Win32::System::Ole::ICreateTypeLib2;
use windows::Win32::System::Ole::ICreateTypeLib;
use windows::Win32::System::Ole::IEnumOLEVERB;
use windows::Win32::System::Ole::INTERFACEDATA;
use windows::Win32::System::Ole::IOleAdviseHolder;
use windows::Win32::System::Ole::IRecordInfo;
use windows::Win32::System::Ole::NUMPARSE;
use windows::Win32::System::Ole::OCPFIPARAMS;
use windows::Win32::System::Ole::OIFI;
use windows::Win32::System::Ole::OleMenuGroupWidths;
use windows::Win32::System::Ole::PICTDESC;
use windows::Win32::System::Ole::REGKIND;
use windows::Win32::System::Ole::UDATE;
use windows::Win32::System::Performance::HardwareCounterProfiling::PERFORMANCE_DATA;
use windows::Win32::System::Performance::PDH_COUNTER_INFO_A;
use windows::Win32::System::Performance::PDH_COUNTER_INFO_W;
use windows::Win32::System::Performance::PDH_COUNTER_PATH_ELEMENTS_A;
use windows::Win32::System::Performance::PDH_COUNTER_PATH_ELEMENTS_W;
use windows::Win32::System::Performance::PDH_DLL_VERSION;
use windows::Win32::System::Performance::PDH_FMT;
use windows::Win32::System::Performance::PDH_FMT_COUNTERVALUE;
use windows::Win32::System::Performance::PDH_FMT_COUNTERVALUE_ITEM_A;
use windows::Win32::System::Performance::PDH_FMT_COUNTERVALUE_ITEM_W;
use windows::Win32::System::Performance::PDH_LOG;
use windows::Win32::System::Performance::PDH_LOG_TYPE;
use windows::Win32::System::Performance::PDH_PATH_FLAGS;
use windows::Win32::System::Performance::PDH_RAW_COUNTER;
use windows::Win32::System::Performance::PDH_RAW_COUNTER_ITEM_A;
use windows::Win32::System::Performance::PDH_RAW_COUNTER_ITEM_W;
use windows::Win32::System::Performance::PDH_RAW_LOG_RECORD;
use windows::Win32::System::Performance::PDH_SELECT_DATA_SOURCE_FLAGS;
use windows::Win32::System::Performance::PDH_STATISTICS;
use windows::Win32::System::Performance::PDH_TIME_INFO;
use windows::Win32::System::Performance::PERFLIBREQUEST;
use windows::Win32::System::Performance::PERF_COUNTERSET_INFO;
use windows::Win32::System::Performance::PERF_COUNTERSET_INSTANCE;
use windows::Win32::System::Performance::PERF_COUNTER_IDENTIFIER;
use windows::Win32::System::Performance::PERF_DATA_HEADER;
use windows::Win32::System::Performance::PERF_INSTANCE_HEADER;
use windows::Win32::System::Performance::PerfProviderHandle;
use windows::Win32::System::Performance::PerfQueryHandle;
use windows::Win32::System::Performance::PerfRegInfoType;
use windows::Win32::System::Performance::REAL_TIME_DATA_SOURCE_ID_FLAGS;
use windows::Win32::System::Pipes::NAMED_PIPE_MODE;
use windows::Win32::System::Power::GLOBAL_POWER_POLICY;
use windows::Win32::System::Power::HPOWERNOTIFY;
use windows::Win32::System::Power::LATENCY_TIME;
use windows::Win32::System::Power::MACHINE_PROCESSOR_POWER_POLICY;
use windows::Win32::System::Power::POWER_DATA_ACCESSOR;
use windows::Win32::System::Power::POWER_INFORMATION_LEVEL;
use windows::Win32::System::Power::POWER_PLATFORM_ROLE_VERSION;
use windows::Win32::System::Power::POWER_POLICY;
use windows::Win32::System::Power::POWER_REQUEST_TYPE;
use windows::Win32::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS;
use windows::Win32::System::Power::PWRSCHEMESENUMPROC;
use windows::Win32::System::Power::SYSTEM_POWER_CAPABILITIES;
use windows::Win32::System::Power::SYSTEM_POWER_STATUS;
use windows::Win32::System::ProcessStatus::MODULEINFO;
use windows::Win32::System::ProcessStatus::PENUM_PAGE_FILE_CALLBACKA;
use windows::Win32::System::ProcessStatus::PENUM_PAGE_FILE_CALLBACKW;
use windows::Win32::System::ProcessStatus::PERFORMANCE_INFORMATION;
use windows::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS;
use windows::Win32::System::ProcessStatus::PSAPI_WS_WATCH_INFORMATION;
use windows::Win32::System::ProcessStatus::PSAPI_WS_WATCH_INFORMATION_EX;
use windows::Win32::System::Recovery::REGISTER_APPLICATION_RESTART_FLAGS;
use windows::Win32::System::Registry::REG_CREATE_KEY_DISPOSITION;
use windows::Win32::System::Registry::REG_NOTIFY_FILTER;
use windows::Win32::System::Registry::REG_OPEN_CREATE_OPTIONS;
use windows::Win32::System::Registry::REG_RESTORE_KEY_FLAGS;
use windows::Win32::System::Registry::REG_SAVE_FORMAT;
use windows::Win32::System::Registry::REG_VALUE_TYPE;
use windows::Win32::System::Registry::RRF_RT;
use windows::Win32::System::Registry::VALENTA;
use windows::Win32::System::Registry::VALENTW;
use windows::Win32::System::RemoteDesktop::WTSLISTENERCONFIGA;
use windows::Win32::System::RemoteDesktop::WTSLISTENERCONFIGW;
use windows::Win32::System::RemoteDesktop::WTS_CONFIG_CLASS;
use windows::Win32::System::RemoteDesktop::WTS_INFO_CLASS;
use windows::Win32::System::RemoteDesktop::WTS_TYPE_CLASS;
use windows::Win32::System::RemoteDesktop::WTS_VIRTUAL_CLASS;
use windows::Win32::System::RemoteManagement::WSMAN_API;
use windows::Win32::System::RemoteManagement::WSMAN_COMMAND;
use windows::Win32::System::RemoteManagement::WSMAN_DATA;
use windows::Win32::System::RemoteManagement::WSMAN_OPERATION;
use windows::Win32::System::RemoteManagement::WSMAN_SESSION;
use windows::Win32::System::RemoteManagement::WSMAN_SHELL;
use windows::Win32::System::RemoteManagement::WSManSessionOption;
use windows::Win32::System::RestartManager::RM_FILTER_ACTION;
use windows::Win32::System::RestartManager::RM_PROCESS_INFO;
use windows::Win32::System::RestartManager::RM_WRITE_STATUS_CALLBACK;
use windows::Win32::System::Restore::STATEMGRSTATUS;
use windows::Win32::System::Services::LPHANDLER_FUNCTION;
use windows::Win32::System::Services::LPHANDLER_FUNCTION_EX;
use windows::Win32::System::Services::QUERY_SERVICE_CONFIGA;
use windows::Win32::System::Services::QUERY_SERVICE_CONFIGW;
use windows::Win32::System::Services::QUERY_SERVICE_LOCK_STATUSA;
use windows::Win32::System::Services::QUERY_SERVICE_LOCK_STATUSW;
use windows::Win32::System::Services::SC_ENUM_TYPE;
use windows::Win32::System::Services::SC_STATUS_TYPE;
use windows::Win32::System::Services::SERVICE_CONFIG;
use windows::Win32::System::Services::SERVICE_DIRECTORY_TYPE;
use windows::Win32::System::Services::SERVICE_ERROR;
use windows::Win32::System::Services::SERVICE_NOTIFY;
use windows::Win32::System::Services::SERVICE_REGISTRY_STATE_TYPE;
use windows::Win32::System::Services::SERVICE_SHARED_DIRECTORY_TYPE;
use windows::Win32::System::Services::SERVICE_SHARED_REGISTRY_STATE_TYPE;
use windows::Win32::System::Services::SERVICE_START_TYPE;
use windows::Win32::System::Services::SERVICE_STATUS;
use windows::Win32::System::Services::SERVICE_STATUS_HANDLE;
use windows::Win32::System::SetupAndMigration::OOBE_COMPLETED_CALLBACK;
use windows::Win32::System::StationsAndDesktops::HDESK;
use windows::Win32::System::StationsAndDesktops::USER_OBJECT_INFORMATION_INDEX;
use windows::Win32::System::StationsAndDesktops::WINSTAENUMPROCA;
use windows::Win32::System::StationsAndDesktops::WINSTAENUMPROCW;
use windows::Win32::System::SystemInformation::GROUP_AFFINITY;
use windows::Win32::System::Threading::GET_GUI_RESOURCES_FLAGS;
use windows::Win32::System::Threading::IO_COUNTERS;
use windows::Win32::System::Threading::LPFIBER_START_ROUTINE;
use windows::Win32::System::Threading::LPPROC_THREAD_ATTRIBUTE_LIST;
use windows::Win32::System::Threading::MACHINE_ATTRIBUTES;
use windows::Win32::System::Threading::NamespaceHandle;
use windows::Win32::System::Threading::PFLS_CALLBACK_FUNCTION;
use windows::Win32::System::Threading::PINIT_ONCE_FN;
use windows::Win32::System::Threading::PROCESSINFOCLASS;
use windows::Win32::System::Threading::PROCESSOR_FEATURE_ID;
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;
use windows::Win32::System::Threading::PROCESS_AFFINITY_AUTO_UPDATE_FLAGS;
use windows::Win32::System::Threading::PROCESS_CREATION_FLAGS;
use windows::Win32::System::Threading::PROCESS_DEP_FLAGS;
use windows::Win32::System::Threading::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET;
use windows::Win32::System::Threading::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE;
use windows::Win32::System::Threading::PROCESS_INFORMATION;
use windows::Win32::System::Threading::PROCESS_INFORMATION_CLASS;
use windows::Win32::System::Threading::PROCESS_MITIGATION_POLICY;
use windows::Win32::System::Threading::PROCESS_NAME_FORMAT;
use windows::Win32::System::Threading::PTIMERAPCROUTINE;
use windows::Win32::System::Threading::PTP_POOL;
use windows::Win32::System::Threading::PTP_SIMPLE_CALLBACK;
use windows::Win32::System::Threading::PTP_TIMER_CALLBACK;
use windows::Win32::System::Threading::PTP_WAIT_CALLBACK;
use windows::Win32::System::Threading::PTP_WIN32_IO_CALLBACK;
use windows::Win32::System::Threading::PTP_WORK_CALLBACK;
use windows::Win32::System::Threading::QUEUE_USER_APC_FLAGS;
use windows::Win32::System::Threading::RTL_BARRIER;
use windows::Win32::System::Threading::RTL_CONDITION_VARIABLE;
use windows::Win32::System::Threading::RTL_CRITICAL_SECTION;
use windows::Win32::System::Threading::RTL_RUN_ONCE;
use windows::Win32::System::Threading::RTL_SRWLOCK;
use windows::Win32::System::Threading::RTL_UMS_THREAD_INFO_CLASS;
use windows::Win32::System::Threading::STARTUPINFOA;
use windows::Win32::System::Threading::STARTUPINFOW;
use windows::Win32::System::Threading::THREADINFOCLASS;
use windows::Win32::System::Threading::THREAD_ACCESS_RIGHTS;
use windows::Win32::System::Threading::THREAD_CREATION_FLAGS;
use windows::Win32::System::Threading::THREAD_INFORMATION_CLASS;
use windows::Win32::System::Threading::THREAD_PRIORITY;
use windows::Win32::System::Threading::TP_CALLBACK_INSTANCE;
use windows::Win32::System::Threading::TP_IO;
use windows::Win32::System::Threading::TP_POOL_STACK_INFORMATION;
use windows::Win32::System::Threading::TP_TIMER;
use windows::Win32::System::Threading::TP_WAIT;
use windows::Win32::System::Threading::TP_WORK;
use windows::Win32::System::Threading::UMS_SYSTEM_THREAD_INFORMATION;
use windows::Win32::System::Threading::WAITORTIMERCALLBACK;
use windows::Win32::System::Threading::WORKER_THREAD_FLAGS;
use windows::Win32::System::Time::TIME_ZONE_INFORMATION;
use windows::Win32::System::TpmBaseServices::TBS_COMMAND_LOCALITY;
use windows::Win32::System::TpmBaseServices::TBS_COMMAND_PRIORITY;
use windows::Win32::System::WinRT::IAgileReference;
use windows::Win32::System::WinRT::IRestrictedErrorInfo;
use windows::Win32::System::WinRT::PINSPECT_HSTRING_CALLBACK2;
use windows::Win32::System::WinRT::PINSPECT_HSTRING_CALLBACK;
use windows::Win32::System::WinRT::PINSPECT_MEMORY_CALLBACK;
use windows::Win32::System::WinRT::Pdf::IPdfRendererNative;
use windows::Win32::System::WinRT::ROPARAMIIDHANDLE;
use windows::Win32::System::WinRT::RO_INIT_TYPE;
use windows::Win32::System::WinRT::ServerInformation;
use windows::Win32::System::WindowsProgramming::PIO_APC_ROUTINE;
use windows::Win32::System::Wmi::MI_Application;
use windows::Win32::UI::Accessibility::IAccessible;
use windows::Win32::UI::Accessibility::IRawElementProviderSimple;
use windows::Win32::UI::Accessibility::NavigateDirection;
use windows::Win32::UI::Accessibility::NormalizeState;
use windows::Win32::UI::Accessibility::NotificationKind;
use windows::Win32::UI::Accessibility::NotificationProcessing;
use windows::Win32::UI::Accessibility::ScrollAmount;
use windows::Win32::UI::Accessibility::StructureChangeType;
use windows::Win32::UI::Accessibility::SupportedTextSelection;
use windows::Win32::UI::Accessibility::SynchronizedInputType;
use windows::Win32::UI::Accessibility::TextEditChangeType;
use windows::Win32::UI::Accessibility::TextPatternRangeEndpoint;
use windows::Win32::UI::Accessibility::TextUnit;
use windows::Win32::UI::Accessibility::TreeScope;
use windows::Win32::UI::Accessibility::UiaCacheRequest;
use windows::Win32::UI::Accessibility::UiaChangeInfo;
use windows::Win32::UI::Accessibility::UiaCondition;
use windows::Win32::UI::Accessibility::UiaEventCallback;
use windows::Win32::UI::Accessibility::UiaFindParams;
use windows::Win32::UI::Accessibility::UiaPoint;
use windows::Win32::UI::Accessibility::UiaProviderCallback;
use windows::Win32::UI::Accessibility::WINEVENTPROC;
use windows::Win32::UI::Accessibility::WindowVisualState;
use windows::Win32::UI::Controls::Dialogs::OPENFILENAMEA;
use windows::Win32::UI::Controls::Dialogs::OPENFILENAMEW;
use windows::Win32::UI::Controls::Dialogs::PAGESETUPDLGA;
use windows::Win32::UI::Controls::Dialogs::PAGESETUPDLGW;
use windows::Win32::UI::Controls::Dialogs::PRINTDLGA;
use windows::Win32::UI::Controls::Dialogs::PRINTDLGEXA;
use windows::Win32::UI::Controls::Dialogs::PRINTDLGEXW;
use windows::Win32::UI::Controls::Dialogs::PRINTDLGW;
use windows::Win32::UI::Controls::GET_THEME_BITMAP_FLAGS;
use windows::Win32::UI::Controls::HDPA;
use windows::Win32::UI::Controls::HDSA;
use windows::Win32::UI::Controls::IMAGEINFO;
use windows::Win32::UI::Controls::INTLIST;
use windows::Win32::UI::Controls::LPFNSVADDPROPSHEETPAGE;
use windows::Win32::UI::Controls::MARGINS;
use windows::Win32::UI::Controls::OPEN_THEME_DATA_FLAGS;
use windows::Win32::UI::Controls::PFNDACOMPARE;
use windows::Win32::UI::Controls::PFNDAENUMCALLBACK;
use windows::Win32::UI::Controls::PFNDPAMERGE;
use windows::Win32::UI::Controls::PFNDPASTREAM;
use windows::Win32::UI::Controls::POINTER_DEVICE_CURSOR_INFO;
use windows::Win32::UI::Controls::POINTER_DEVICE_INFO;
use windows::Win32::UI::Controls::POINTER_DEVICE_PROPERTY;
use windows::Win32::UI::Controls::POINTER_FEEDBACK_MODE;
use windows::Win32::UI::Controls::PROPERTYORIGIN;
use windows::Win32::UI::Controls::PROPSHEETHEADERA_V2;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2;
use windows::Win32::UI::Controls::PROPSHEETPAGEA;
use windows::Win32::UI::Controls::PROPSHEETPAGEW;
use windows::Win32::UI::Controls::TASKDIALOG_COMMON_BUTTON_FLAGS;
use windows::Win32::UI::Controls::TA_PROPERTY;
use windows::Win32::UI::Controls::TA_TIMINGFUNCTION;
use windows::Win32::UI::Controls::TA_TRANSFORM;
use windows::Win32::UI::Controls::TBBUTTON;
use windows::Win32::UI::Controls::THEMESIZE;
use windows::Win32::UI::Controls::THEME_PROPERTY_SYMBOL_ID;
use windows::Win32::UI::Controls::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION;
use windows::Win32::UI::Controls::WINDOWTHEMEATTRIBUTETYPE;
use windows::Win32::UI::Controls::WSB_PROP;
use windows::Win32::UI::HiDpi::MONITOR_DPI_TYPE;
use windows::Win32::UI::HiDpi::PROCESS_DPI_AWARENESS;
use windows::Win32::UI::Input::INPUT_MESSAGE_SOURCE;
use windows::Win32::UI::Input::Ime::GET_CONVERSION_LIST_FLAG;
use windows::Win32::UI::Input::Ime::GET_GUIDE_LINE_TYPE;
use windows::Win32::UI::Input::Ime::IMEMENUITEMINFOA;
use windows::Win32::UI::Input::Ime::IMEMENUITEMINFOW;
use windows::Win32::UI::Input::Ime::NOTIFY_IME_ACTION;
use windows::Win32::UI::Input::Ime::NOTIFY_IME_INDEX;
use windows::Win32::UI::Input::Ime::REGISTERWORDENUMPROCA;
use windows::Win32::UI::Input::Ime::REGISTERWORDENUMPROCW;
use windows::Win32::UI::Input::Ime::SET_COMPOSITION_STRING_TYPE;
use windows::Win32::UI::Input::Ime::STYLEBUFA;
use windows::Win32::UI::Input::Ime::STYLEBUFW;
use windows::Win32::UI::Input::KeyboardAndMouse::GET_MOUSE_MOVE_POINTS_EX_RESOLUTION;
use windows::Win32::UI::Input::KeyboardAndMouse::HOT_KEY_MODIFIERS;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS;
use windows::Win32::UI::Input::KeyboardAndMouse::LASTINPUTINFO;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEMOVEPOINT;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSE_EVENT_FLAGS;
use windows::Win32::UI::Input::KeyboardAndMouse::TRACKMOUSEEVENT;
use windows::Win32::UI::Input::Pointer::INPUT_TRANSFORM;
use windows::Win32::UI::Input::Pointer::POINTER_INFO;
use windows::Win32::UI::Input::Pointer::POINTER_PEN_INFO;
use windows::Win32::UI::Input::Pointer::POINTER_TOUCH_INFO;
use windows::Win32::UI::Input::Pointer::TOUCH_FEEDBACK_MODE;
use windows::Win32::UI::Input::RAWINPUT;
use windows::Win32::UI::Input::RAWINPUTDEVICE;
use windows::Win32::UI::Input::RAWINPUTDEVICELIST;
use windows::Win32::UI::Input::RAW_INPUT_DATA_COMMAND_FLAGS;
use windows::Win32::UI::Input::RAW_INPUT_DEVICE_INFO_COMMAND;
use windows::Win32::UI::Input::Touch::GESTUREINFO;
use windows::Win32::UI::Input::Touch::HGESTUREINFO;
use windows::Win32::UI::Input::Touch::REGISTER_TOUCH_WINDOW_FLAGS;
use windows::Win32::UI::Input::Touch::TOUCHINPUT;
use windows::Win32::UI::InteractionContext::HOLD_PARAMETER;
use windows::Win32::UI::InteractionContext::INTERACTION_CONTEXT_CONFIGURATION;
use windows::Win32::UI::InteractionContext::INTERACTION_STATE;
use windows::Win32::UI::InteractionContext::MOUSE_WHEEL_PARAMETER;
use windows::Win32::UI::InteractionContext::TAP_PARAMETER;
use windows::Win32::UI::InteractionContext::TRANSLATION_PARAMETER;
use windows::Win32::UI::Shell::Common::PERCEIVED;
use windows::Win32::UI::Shell::Common::STRRET;
use windows::Win32::UI::Shell::IContextMenu;
use windows::Win32::UI::Shell::IEnumAssocHandlers;
use windows::Win32::UI::Shell::IFileOperation;
use windows::Win32::UI::Shell::IShellFolder;
use windows::Win32::UI::Shell::IShellItem;
use windows::Win32::UI::Shell::IShellItemArray;
use windows::Win32::UI::Shell::IShellView;
use windows::Win32::UI::Shell::KNOWN_FOLDER_FLAG;
use windows::Win32::UI::Shell::LIBRARYMANAGEDIALOGOPTIONS;
use windows::Win32::UI::Shell::LPFNDFMCALLBACK;
use windows::Win32::UI::Shell::MM_FLAGS;
use windows::Win32::UI::Shell::NOTIFY_ICON_MESSAGE;
use windows::Win32::UI::Shell::OS;
use windows::Win32::UI::Shell::PAPPCONSTRAIN_CHANGE_ROUTINE;
use windows::Win32::UI::Shell::PAPPSTATE_CHANGE_ROUTINE;
use windows::Win32::UI::Shell::PARSEDURLA;
use windows::Win32::UI::Shell::PARSEDURLW;
use windows::Win32::UI::Shell::PRF_FLAGS;
use windows::Win32::UI::Shell::PROFILEINFOA;
use windows::Win32::UI::Shell::PROFILEINFOW;
use windows::Win32::UI::Shell::PropertiesSystem::PKA_FLAGS;
use windows::Win32::UI::Shell::PropertiesSystem::PROPDESC_ENUMFILTER;
use windows::Win32::UI::Shell::PropertiesSystem::PROPDESC_FORMAT_FLAGS;
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;
use windows::Win32::UI::Shell::PropertiesSystem::PROPVAR_CHANGE_FLAGS;
use windows::Win32::UI::Shell::PropertiesSystem::PROPVAR_COMPARE_FLAGS;
use windows::Win32::UI::Shell::PropertiesSystem::PROPVAR_COMPARE_UNIT;
use windows::Win32::UI::Shell::PropertiesSystem::PSTIME_FLAGS;
use windows::Win32::UI::Shell::QUERY_USER_NOTIFICATION_STATE;
use windows::Win32::UI::Shell::RESTRICTIONS;
use windows::Win32::UI::Shell::SCNRT_STATUS;
use windows::Win32::UI::Shell::SFBS_FLAGS;
use windows::Win32::UI::Shell::SHCNE_ID;
use windows::Win32::UI::Shell::SHCNF_FLAGS;
use windows::Win32::UI::Shell::SHCNRF_SOURCE;
use windows::Win32::UI::Shell::SHCREATEPROCESSINFOW;
use windows::Win32::UI::Shell::SHELLEXECUTEINFOA;
use windows::Win32::UI::Shell::SHELLEXECUTEINFOW;
use windows::Win32::UI::Shell::SHELLFLAGSTATE;
use windows::Win32::UI::Shell::SHELLSTATEA;
use windows::Win32::UI::Shell::SHELL_UI_COMPONENT;
use windows::Win32::UI::Shell::SHFILEINFOA;
use windows::Win32::UI::Shell::SHFILEINFOW;
use windows::Win32::UI::Shell::SHFILEOPSTRUCTA;
use windows::Win32::UI::Shell::SHFILEOPSTRUCTW;
use windows::Win32::UI::Shell::SHFMT_ID;
use windows::Win32::UI::Shell::SHFMT_OPT;
use windows::Win32::UI::Shell::SHFOLDERCUSTOMSETTINGS;
use windows::Win32::UI::Shell::SHGDFIL_FORMAT;
use windows::Win32::UI::Shell::SHGFI_FLAGS;
use windows::Win32::UI::Shell::SHGLOBALCOUNTER;
use windows::Win32::UI::Shell::SHOP_TYPE;
use windows::Win32::UI::Shell::SHQUERYRBINFO;
use windows::Win32::UI::Shell::SHREGDEL_FLAGS;
use windows::Win32::UI::Shell::SHREGENUM_FLAGS;
use windows::Win32::UI::Shell::SHSTOCKICONID;
use windows::Win32::UI::Shell::SHSTOCKICONINFO;
use windows::Win32::UI::Shell::SIGDN;
use windows::Win32::UI::Shell::SSF_MASK;
use windows::Win32::UI::Shell::SUBCLASSPROC;
use windows::Win32::UI::Shell::URLIS;
use windows::Win32::UI::Shell::VALIDATEUNC_OPTION;
use windows::Win32::UI::TabletPC::RECO_ATTRS;
use windows::Win32::UI::WindowsAndMessaging::HACCEL;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_RESULT;
use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::SCROLLBAR_CONSTANTS;
use windows::Win32::UI::WindowsAndMessaging::SCROLLINFO;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WNDENUMPROC;





pub fn get_hwnd() -> (HWND, String) {    (HWND(0), "HWND(0)".to_string())}
pub fn get_guid() -> (GUID, String) {    (GUID::zeroed(), "GUID::zeroed()".to_string())}
pub fn get_file_handle() -> (HANDLE, String) {    (HANDLE(0), "HANDLE(0)".to_string())}

pub fn get_strange_ABC() -> (ABC, String) { (ABC::default(), "ABC::default()".to_string()) }
pub fn get_strange_ABCFLOAT() -> (ABCFLOAT, String) { (ABCFLOAT::default(), "ABCFLOAT::default()".to_string()) }
pub fn get_strange_ABORTPROC() -> (ABORTPROC, String) { (ABORTPROC::default(), "ABORTPROC::default()".to_string()) }
pub fn get_strange_ACCESS_MODE() -> (ACCESS_MODE, String) { (ACCESS_MODE::default(), "ACCESS_MODE::default()".to_string()) }
pub fn get_strange_ACC_UTILITY_STATE_FLAGS() -> (ACC_UTILITY_STATE_FLAGS, String) { (ACC_UTILITY_STATE_FLAGS::default(), "ACC_UTILITY_STATE_FLAGS::default()".to_string()) }
pub fn get_strange_ACE_FLAGS() -> (ACE_FLAGS, String) { (ACE_FLAGS::default(), "ACE_FLAGS::default()".to_string()) }
pub fn get_strange_ACE_REVISION() -> (ACE_REVISION, String) { (ACE_REVISION::default(), "ACE_REVISION::default()".to_string()) }
pub fn get_strange_ACL() -> (ACL, String) {    (ACL::default(), "ACL::default()".to_string())}
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
pub fn get_strange_AsnObjectIdentifier() -> (AsnObjectIdentifier, String) {    (AsnObjectIdentifier::default(), "AsnObjectIdentifier::default()".to_string())}
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
pub fn get_strange_BOOL() -> (BOOL, String) {    (BOOL::default(), "BOOL::default()".to_string())}
pub fn get_strange_BOOLEAN() -> (BOOLEAN, String) {    (BOOLEAN::default(), "BOOLEAN::default()".to_string())}
pub fn get_strange_BP_BUFFERFORMAT() -> (BP_BUFFERFORMAT, String) { (BP_BUFFERFORMAT::default(), "BP_BUFFERFORMAT::default()".to_string()) }
pub fn get_strange_BROADCAST_SYSTEM_MESSAGE_FLAGS() -> (BROADCAST_SYSTEM_MESSAGE_FLAGS, String) { (BROADCAST_SYSTEM_MESSAGE_FLAGS::default(), "BROADCAST_SYSTEM_MESSAGE_FLAGS::default()".to_string()) }
pub fn get_strange_BROADCAST_SYSTEM_MESSAGE_INFO() -> (BROADCAST_SYSTEM_MESSAGE_INFO, String) { (BROADCAST_SYSTEM_MESSAGE_INFO::default(), "BROADCAST_SYSTEM_MESSAGE_INFO::default()".to_string()) }
pub fn get_strange_BRUSHOBJ() -> (BRUSHOBJ, String) { (BRUSHOBJ::default(), "BRUSHOBJ::default()".to_string()) }
pub fn get_strange_BSMINFO() -> (BSMINFO, String) { (BSMINFO::default(), "BSMINFO::default()".to_string()) }
pub fn get_strange_BSOS_OPTIONS() -> (BSOS_OPTIONS, String) { (BSOS_OPTIONS::default(), "BSOS_OPTIONS::default()".to_string()) }
pub fn get_strange_BSTR() -> (BSTR, String) {    (BSTR::default(), "BSTR::default()".to_string())}
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
pub fn get_strange_CHAR() -> (CHAR, String) {    (CHAR::default(), "CHAR::default()".to_string())}
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
pub fn get_strange_COORD() -> (COORD, String) {    (COORD::default(), "COORD::default()".to_string())}
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
pub fn get_strange_CY() -> (CY, String) {    (CY::default(), "CY::default()".to_string())}
pub fn get_strange_CreatePackageDependencyOptions() -> (CreatePackageDependencyOptions, String) { (CreatePackageDependencyOptions::default(), "CreatePackageDependencyOptions::default()".to_string()) }
pub fn get_strange_CreatedHDC() -> (CreatedHDC, String) { (CreatedHDC::default(), "CreatedHDC::default()".to_string()) }
pub fn get_strange_D2D1_COLOR_SPACE() -> (D2D1_COLOR_SPACE, String) { (D2D1_COLOR_SPACE::default(), "D2D1_COLOR_SPACE::default()".to_string()) }
pub fn get_strange_D2D1_FACTORY_TYPE() -> (D2D1_FACTORY_TYPE, String) { (D2D1_FACTORY_TYPE::default(), "D2D1_FACTORY_TYPE::default()".to_string()) }
pub fn get_strange_D2D_POINT_2F() -> (D2D_POINT_2F, String) { (D2D_POINT_2F::default(), "D2D_POINT_2F::default()".to_string()) }
pub fn get_strange_D3D10_DEVICE_STATE_TYPES() -> (D3D10_DEVICE_STATE_TYPES, String) { (D3D10_DEVICE_STATE_TYPES::default(), "D3D10_DEVICE_STATE_TYPES::default()".to_string()) }
pub fn get_strange_D3D10_DRIVER_TYPE() -> (D3D10_DRIVER_TYPE, String) { (D3D10_DRIVER_TYPE::default(), "D3D10_DRIVER_TYPE::default()".to_string()) }
pub fn get_strange_D3D10_FEATURE_LEVEL1() -> (D3D10_FEATURE_LEVEL1, String) { (D3D10_FEATURE_LEVEL1::default(), "D3D10_FEATURE_LEVEL1::default()".to_string()) }
pub fn get_strange_D3D10_STATE_BLOCK_MASK() -> (D3D10_STATE_BLOCK_MASK, String) { (D3D10_STATE_BLOCK_MASK::default(), "D3D10_STATE_BLOCK_MASK::default()".to_string()) }
pub fn get_strange_D3D11_CREATE_DEVICE_FLAG() -> (D3D11_CREATE_DEVICE_FLAG, String) { (D3D11_CREATE_DEVICE_FLAG::default(), "D3D11_CREATE_DEVICE_FLAG::default()".to_string()) }
pub fn get_strange_D3D9ON12_ARGS() -> (D3D9ON12_ARGS, String) { (D3D9ON12_ARGS::default(), "D3D9ON12_ARGS::default()".to_string()) }
pub fn get_strange_D3DX11_FFT_BUFFER_INFO() -> (D3DX11_FFT_BUFFER_INFO, String) { (D3DX11_FFT_BUFFER_INFO::default(), "D3DX11_FFT_BUFFER_INFO::default()".to_string()) }
pub fn get_strange_D3D_BLOB_PART() -> (D3D_BLOB_PART, String) { (D3D_BLOB_PART::default(), "D3D_BLOB_PART::default()".to_string()) }
pub fn get_strange_D3D_DRIVER_TYPE() -> (D3D_DRIVER_TYPE, String) { (D3D_DRIVER_TYPE::default(), "D3D_DRIVER_TYPE::default()".to_string()) }
pub fn get_strange_D3D_FEATURE_LEVEL() -> (D3D_FEATURE_LEVEL, String) { (D3D_FEATURE_LEVEL::default(), "D3D_FEATURE_LEVEL::default()".to_string()) }
pub fn get_strange_D3D_ROOT_SIGNATURE_VERSION() -> (D3D_ROOT_SIGNATURE_VERSION, String) { (D3D_ROOT_SIGNATURE_VERSION::default(), "D3D_ROOT_SIGNATURE_VERSION::default()".to_string()) }
pub fn get_strange_DBGPRINT() -> (DBGPRINT, String) { (DBGPRINT::default(), "DBGPRINT::default()".to_string()) }
pub fn get_strange_DC_LAYOUT() -> (DC_LAYOUT, String) { (DC_LAYOUT::default(), "DC_LAYOUT::default()".to_string()) }
pub fn get_strange_DDE_CLIENT_TRANSACTION_TYPE() -> (DDE_CLIENT_TRANSACTION_TYPE, String) { (DDE_CLIENT_TRANSACTION_TYPE::default(), "DDE_CLIENT_TRANSACTION_TYPE::default()".to_string()) }
pub fn get_strange_DDE_ENABLE_CALLBACK_CMD() -> (DDE_ENABLE_CALLBACK_CMD, String) { (DDE_ENABLE_CALLBACK_CMD::default(), "DDE_ENABLE_CALLBACK_CMD::default()".to_string()) }
pub fn get_strange_DDE_INITIALIZE_COMMAND() -> (DDE_INITIALIZE_COMMAND, String) { (DDE_INITIALIZE_COMMAND::default(), "DDE_INITIALIZE_COMMAND::default()".to_string()) }
pub fn get_strange_DDE_NAME_SERVICE_CMD() -> (DDE_NAME_SERVICE_CMD, String) { (DDE_NAME_SERVICE_CMD::default(), "DDE_NAME_SERVICE_CMD::default()".to_string()) }
pub fn get_strange_DECIMAL() -> (DECIMAL, String) {    (DECIMAL::default(), "DECIMAL::default()".to_string())}
pub fn get_strange_DECODING_SOURCE() -> (DECODING_SOURCE, String) { (DECODING_SOURCE::default(), "DECODING_SOURCE::default()".to_string()) }
pub fn get_strange_DELETE_SNAPSHOT_VHDSET_FLAG() -> (DELETE_SNAPSHOT_VHDSET_FLAG, String) { (DELETE_SNAPSHOT_VHDSET_FLAG::default(), "DELETE_SNAPSHOT_VHDSET_FLAG::default()".to_string()) }
pub fn get_strange_DESKTOPENUMPROCA() -> (DESKTOPENUMPROCA, String) { (DESKTOPENUMPROCA::default(), "DESKTOPENUMPROCA::default()".to_string()) }
pub fn get_strange_DESKTOPENUMPROCW() -> (DESKTOPENUMPROCW, String) { (DESKTOPENUMPROCW::default(), "DESKTOPENUMPROCW::default()".to_string()) }
pub fn get_strange_DETACH_VIRTUAL_DISK_FLAG() -> (DETACH_VIRTUAL_DISK_FLAG, String) { (DETACH_VIRTUAL_DISK_FLAG::default(), "DETACH_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_DEVICE_CAPABILITIES() -> (DEVICE_CAPABILITIES, String) { (DEVICE_CAPABILITIES::default(), "DEVICE_CAPABILITIES::default()".to_string()) }
pub fn get_strange_DEVICE_SCALE_FACTOR() -> (DEVICE_SCALE_FACTOR, String) { (DEVICE_SCALE_FACTOR::default(), "DEVICE_SCALE_FACTOR::default()".to_string()) }
pub fn get_strange_DEVMODEA() -> (DEVMODEA, String) {    (DEVMODEA::default(), "DEVMODEA::default()".to_string())}
pub fn get_strange_DEVMODEW() -> (DEVMODEW, String) {    (DEVMODEW::default(), "DEVMODEW::default()".to_string())}
pub fn get_strange_DEVPROPKEY() -> (DEVPROPKEY, String) { (DEVPROPKEY::default(), "DEVPROPKEY::default()".to_string()) }
pub fn get_strange_DEVPROPSTORE() -> (DEVPROPSTORE, String) { (DEVPROPSTORE::default(), "DEVPROPSTORE::default()".to_string()) }
pub fn get_strange_DEVQUERYPRINT_INFO() -> (DEVQUERYPRINT_INFO, String) { (DEVQUERYPRINT_INFO::default(), "DEVQUERYPRINT_INFO::default()".to_string()) }
pub fn get_strange_DEV_OBJECT_TYPE() -> (DEV_OBJECT_TYPE, String) { (DEV_OBJECT_TYPE::default(), "DEV_OBJECT_TYPE::default()".to_string()) }
pub fn get_strange_DFCS_STATE() -> (DFCS_STATE, String) { (DFCS_STATE::default(), "DFCS_STATE::default()".to_string()) }
pub fn get_strange_DFC_TYPE() -> (DFC_TYPE, String) { (DFC_TYPE::default(), "DFC_TYPE::default()".to_string()) }
pub fn get_strange_DFS_NAMESPACE_VERSION_ORIGIN() -> (DFS_NAMESPACE_VERSION_ORIGIN, String) { (DFS_NAMESPACE_VERSION_ORIGIN::default(), "DFS_NAMESPACE_VERSION_ORIGIN::default()".to_string()) }
pub fn get_strange_DHSURF() -> (DHSURF, String) { (DHSURF::default(), "DHSURF::default()".to_string()) }
pub fn get_strange_DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS() -> (DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, String) { (DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS::default(), "DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS::default()".to_string()) }
pub fn get_strange_DIALOG_DPI_CHANGE_BEHAVIORS() -> (DIALOG_DPI_CHANGE_BEHAVIORS, String) { (DIALOG_DPI_CHANGE_BEHAVIORS::default(), "DIALOG_DPI_CHANGE_BEHAVIORS::default()".to_string()) }
pub fn get_strange_DIB_USAGE() -> (DIB_USAGE, String) { (DIB_USAGE::default(), "DIB_USAGE::default()".to_string()) }
pub fn get_strange_DIGEST_FUNCTION() -> (DIGEST_FUNCTION, String) { (DIGEST_FUNCTION::default(), "DIGEST_FUNCTION::default()".to_string()) }
pub fn get_strange_DISPLAYCONFIG_DEVICE_INFO_HEADER() -> (DISPLAYCONFIG_DEVICE_INFO_HEADER, String) { (DISPLAYCONFIG_DEVICE_INFO_HEADER::default(), "DISPLAYCONFIG_DEVICE_INFO_HEADER::default()".to_string()) }
pub fn get_strange_DISPLAYCONFIG_MODE_INFO() -> (DISPLAYCONFIG_MODE_INFO, String) { (DISPLAYCONFIG_MODE_INFO::default(), "DISPLAYCONFIG_MODE_INFO::default()".to_string()) }
pub fn get_strange_DISPLAYCONFIG_PATH_INFO() -> (DISPLAYCONFIG_PATH_INFO, String) { (DISPLAYCONFIG_PATH_INFO::default(), "DISPLAYCONFIG_PATH_INFO::default()".to_string()) }
pub fn get_strange_DISPLAYCONFIG_TOPOLOGY_ID() -> (DISPLAYCONFIG_TOPOLOGY_ID, String) { (DISPLAYCONFIG_TOPOLOGY_ID::default(), "DISPLAYCONFIG_TOPOLOGY_ID::default()".to_string()) }
pub fn get_strange_DISPLAY_DEVICEA() -> (DISPLAY_DEVICEA, String) { (DISPLAY_DEVICEA::default(), "DISPLAY_DEVICEA::default()".to_string()) }
pub fn get_strange_DISPLAY_DEVICEW() -> (DISPLAY_DEVICEW, String) { (DISPLAY_DEVICEW::default(), "DISPLAY_DEVICEW::default()".to_string()) }
pub fn get_strange_DISPLAY_DEVICE_TYPE() -> (DISPLAY_DEVICE_TYPE, String) { (DISPLAY_DEVICE_TYPE::default(), "DISPLAY_DEVICE_TYPE::default()".to_string()) }
pub fn get_strange_DISPPARAMS() -> (DISPPARAMS, String) { (DISPPARAMS::default(), "DISPPARAMS::default()".to_string()) }
pub fn get_strange_DLG_BUTTON_CHECK_STATE() -> (DLG_BUTTON_CHECK_STATE, String) { (DLG_BUTTON_CHECK_STATE::default(), "DLG_BUTTON_CHECK_STATE::default()".to_string()) }
pub fn get_strange_DLG_DIR_LIST_FILE_TYPE() -> (DLG_DIR_LIST_FILE_TYPE, String) { (DLG_DIR_LIST_FILE_TYPE::default(), "DLG_DIR_LIST_FILE_TYPE::default()".to_string()) }
pub fn get_strange_DML_CREATE_DEVICE_FLAGS() -> (DML_CREATE_DEVICE_FLAGS, String) { (DML_CREATE_DEVICE_FLAGS::default(), "DML_CREATE_DEVICE_FLAGS::default()".to_string()) }
pub fn get_strange_DML_FEATURE_LEVEL() -> (DML_FEATURE_LEVEL, String) { (DML_FEATURE_LEVEL::default(), "DML_FEATURE_LEVEL::default()".to_string()) }
pub fn get_strange_DMO_MEDIA_TYPE() -> (DMO_MEDIA_TYPE, String) { (DMO_MEDIA_TYPE::default(), "DMO_MEDIA_TYPE::default()".to_string()) }
pub fn get_strange_DMO_PARTIAL_MEDIATYPE() -> (DMO_PARTIAL_MEDIATYPE, String) { (DMO_PARTIAL_MEDIATYPE::default(), "DMO_PARTIAL_MEDIATYPE::default()".to_string()) }
pub fn get_strange_DNS_APPLICATION_SETTINGS() -> (DNS_APPLICATION_SETTINGS, String) { (DNS_APPLICATION_SETTINGS::default(), "DNS_APPLICATION_SETTINGS::default()".to_string()) }
pub fn get_strange_DNS_CHARSET() -> (DNS_CHARSET, String) { (DNS_CHARSET::default(), "DNS_CHARSET::default()".to_string()) }
pub fn get_strange_DNS_CONFIG_TYPE() -> (DNS_CONFIG_TYPE, String) { (DNS_CONFIG_TYPE::default(), "DNS_CONFIG_TYPE::default()".to_string()) }
pub fn get_strange_DNS_CONNECTION_NAME_LIST() -> (DNS_CONNECTION_NAME_LIST, String) { (DNS_CONNECTION_NAME_LIST::default(), "DNS_CONNECTION_NAME_LIST::default()".to_string()) }
pub fn get_strange_DNS_CONNECTION_POLICY_TAG() -> (DNS_CONNECTION_POLICY_TAG, String) { (DNS_CONNECTION_POLICY_TAG::default(), "DNS_CONNECTION_POLICY_TAG::default()".to_string()) }
pub fn get_strange_DNS_CONNECTION_PROXY_INFO() -> (DNS_CONNECTION_PROXY_INFO, String) { (DNS_CONNECTION_PROXY_INFO::default(), "DNS_CONNECTION_PROXY_INFO::default()".to_string()) }
pub fn get_strange_DNS_CONNECTION_PROXY_INFO_EX() -> (DNS_CONNECTION_PROXY_INFO_EX, String) { (DNS_CONNECTION_PROXY_INFO_EX::default(), "DNS_CONNECTION_PROXY_INFO_EX::default()".to_string()) }
pub fn get_strange_DNS_CONNECTION_PROXY_LIST() -> (DNS_CONNECTION_PROXY_LIST, String) { (DNS_CONNECTION_PROXY_LIST::default(), "DNS_CONNECTION_PROXY_LIST::default()".to_string()) }
pub fn get_strange_DNS_CONNECTION_PROXY_TYPE() -> (DNS_CONNECTION_PROXY_TYPE, String) { (DNS_CONNECTION_PROXY_TYPE::default(), "DNS_CONNECTION_PROXY_TYPE::default()".to_string()) }
pub fn get_strange_DNS_FREE_TYPE() -> (DNS_FREE_TYPE, String) { (DNS_FREE_TYPE::default(), "DNS_FREE_TYPE::default()".to_string()) }
pub fn get_strange_DNS_INTERFACE_SETTINGS() -> (DNS_INTERFACE_SETTINGS, String) { (DNS_INTERFACE_SETTINGS::default(), "DNS_INTERFACE_SETTINGS::default()".to_string()) }
pub fn get_strange_DNS_MESSAGE_BUFFER() -> (DNS_MESSAGE_BUFFER, String) { (DNS_MESSAGE_BUFFER::default(), "DNS_MESSAGE_BUFFER::default()".to_string()) }
pub fn get_strange_DNS_NAME_FORMAT() -> (DNS_NAME_FORMAT, String) { (DNS_NAME_FORMAT::default(), "DNS_NAME_FORMAT::default()".to_string()) }
pub fn get_strange_DNS_PROXY_COMPLETION_ROUTINE() -> (DNS_PROXY_COMPLETION_ROUTINE, String) { (DNS_PROXY_COMPLETION_ROUTINE::default(), "DNS_PROXY_COMPLETION_ROUTINE::default()".to_string()) }
pub fn get_strange_DNS_PROXY_INFORMATION() -> (DNS_PROXY_INFORMATION, String) { (DNS_PROXY_INFORMATION::default(), "DNS_PROXY_INFORMATION::default()".to_string()) }
pub fn get_strange_DNS_QUERY_CANCEL() -> (DNS_QUERY_CANCEL, String) { (DNS_QUERY_CANCEL::default(), "DNS_QUERY_CANCEL::default()".to_string()) }
pub fn get_strange_DNS_QUERY_RESULT() -> (DNS_QUERY_RESULT, String) { (DNS_QUERY_RESULT::default(), "DNS_QUERY_RESULT::default()".to_string()) }
pub fn get_strange_DNS_RECORDA() -> (DNS_RECORDA, String) { (DNS_RECORDA::default(), "DNS_RECORDA::default()".to_string()) }
pub fn get_strange_DNS_SERVICE_CANCEL() -> (DNS_SERVICE_CANCEL, String) { (DNS_SERVICE_CANCEL::default(), "DNS_SERVICE_CANCEL::default()".to_string()) }
pub fn get_strange_DNS_SETTINGS() -> (DNS_SETTINGS, String) { (DNS_SETTINGS::default(), "DNS_SETTINGS::default()".to_string()) }
pub fn get_strange_DOCINFOW() -> (DOCINFOW, String) { (DOCINFOW::default(), "DOCINFOW::default()".to_string()) }
pub fn get_strange_DOT11_BSS_TYPE() -> (DOT11_BSS_TYPE, String) { (DOT11_BSS_TYPE::default(), "DOT11_BSS_TYPE::default()".to_string()) }
pub fn get_strange_DPI_AWARENESS_CONTEXT() -> (DPI_AWARENESS_CONTEXT, String) { (DPI_AWARENESS_CONTEXT::default(), "DPI_AWARENESS_CONTEXT::default()".to_string()) }
pub fn get_strange_DPI_HOSTING_BEHAVIOR() -> (DPI_HOSTING_BEHAVIOR, String) { (DPI_HOSTING_BEHAVIOR::default(), "DPI_HOSTING_BEHAVIOR::default()".to_string()) }
pub fn get_strange_DRAWDIBTIME() -> (DRAWDIBTIME, String) { (DRAWDIBTIME::default(), "DRAWDIBTIME::default()".to_string()) }
pub fn get_strange_DRAWEDGE_FLAGS() -> (DRAWEDGE_FLAGS, String) { (DRAWEDGE_FLAGS::default(), "DRAWEDGE_FLAGS::default()".to_string()) }
pub fn get_strange_DRAWSTATEPROC() -> (DRAWSTATEPROC, String) { (DRAWSTATEPROC::default(), "DRAWSTATEPROC::default()".to_string()) }
pub fn get_strange_DRAWSTATE_FLAGS() -> (DRAWSTATE_FLAGS, String) { (DRAWSTATE_FLAGS::default(), "DRAWSTATE_FLAGS::default()".to_string()) }
pub fn get_strange_DRAW_CAPTION_FLAGS() -> (DRAW_CAPTION_FLAGS, String) { (DRAW_CAPTION_FLAGS::default(), "DRAW_CAPTION_FLAGS::default()".to_string()) }
pub fn get_strange_DRAW_EDGE_FLAGS() -> (DRAW_EDGE_FLAGS, String) { (DRAW_EDGE_FLAGS::default(), "DRAW_EDGE_FLAGS::default()".to_string()) }
pub fn get_strange_DRAW_TEXT_FORMAT() -> (DRAW_TEXT_FORMAT, String) { (DRAW_TEXT_FORMAT::default(), "DRAW_TEXT_FORMAT::default()".to_string()) }
pub fn get_strange_DRAW_THEME_PARENT_BACKGROUND_FLAGS() -> (DRAW_THEME_PARENT_BACKGROUND_FLAGS, String) { (DRAW_THEME_PARENT_BACKGROUND_FLAGS::default(), "DRAW_THEME_PARENT_BACKGROUND_FLAGS::default()".to_string()) }
pub fn get_strange_DRIVERMSGPROC() -> (DRIVERMSGPROC, String) { (DRIVERMSGPROC::default(), "DRIVERMSGPROC::default()".to_string()) }
pub fn get_strange_DRMATTESTTYPE() -> (DRMATTESTTYPE, String) { (DRMATTESTTYPE::default(), "DRMATTESTTYPE::default()".to_string()) }
pub fn get_strange_DRMBOUNDLICENSEPARAMS() -> (DRMBOUNDLICENSEPARAMS, String) { (DRMBOUNDLICENSEPARAMS::default(), "DRMBOUNDLICENSEPARAMS::default()".to_string()) }
pub fn get_strange_DRMCALLBACK() -> (DRMCALLBACK, String) { (DRMCALLBACK::default(), "DRMCALLBACK::default()".to_string()) }
pub fn get_strange_DRMENCODINGTYPE() -> (DRMENCODINGTYPE, String) { (DRMENCODINGTYPE::default(), "DRMENCODINGTYPE::default()".to_string()) }
pub fn get_strange_DRMGLOBALOPTIONS() -> (DRMGLOBALOPTIONS, String) { (DRMGLOBALOPTIONS::default(), "DRMGLOBALOPTIONS::default()".to_string()) }
pub fn get_strange_DRMID() -> (DRMID, String) { (DRMID::default(), "DRMID::default()".to_string()) }
pub fn get_strange_DRMSECURITYPROVIDERTYPE() -> (DRMSECURITYPROVIDERTYPE, String) { (DRMSECURITYPROVIDERTYPE::default(), "DRMSECURITYPROVIDERTYPE::default()".to_string()) }
pub fn get_strange_DRMSPECTYPE() -> (DRMSPECTYPE, String) { (DRMSPECTYPE::default(), "DRMSPECTYPE::default()".to_string()) }
pub fn get_strange_DRMTIMETYPE() -> (DRMTIMETYPE, String) { (DRMTIMETYPE::default(), "DRMTIMETYPE::default()".to_string()) }
pub fn get_strange_DRM_ACTSERV_INFO() -> (DRM_ACTSERV_INFO, String) { (DRM_ACTSERV_INFO::default(), "DRM_ACTSERV_INFO::default()".to_string()) }
pub fn get_strange_DRM_CLIENT_VERSION_INFO() -> (DRM_CLIENT_VERSION_INFO, String) { (DRM_CLIENT_VERSION_INFO::default(), "DRM_CLIENT_VERSION_INFO::default()".to_string()) }
pub fn get_strange_DRM_USAGEPOLICY_TYPE() -> (DRM_USAGEPOLICY_TYPE, String) { (DRM_USAGEPOLICY_TYPE::default(), "DRM_USAGEPOLICY_TYPE::default()".to_string()) }
pub fn get_strange_DUPLICATE_HANDLE_OPTIONS() -> (DUPLICATE_HANDLE_OPTIONS, String) { (DUPLICATE_HANDLE_OPTIONS::default(), "DUPLICATE_HANDLE_OPTIONS::default()".to_string()) }
pub fn get_strange_DWMTRANSITION_OWNEDWINDOW_TARGET() -> (DWMTRANSITION_OWNEDWINDOW_TARGET, String) { (DWMTRANSITION_OWNEDWINDOW_TARGET::default(), "DWMTRANSITION_OWNEDWINDOW_TARGET::default()".to_string()) }
pub fn get_strange_DWMWINDOWATTRIBUTE() -> (DWMWINDOWATTRIBUTE, String) { (DWMWINDOWATTRIBUTE::default(), "DWMWINDOWATTRIBUTE::default()".to_string()) }
pub fn get_strange_DWM_PRESENT_PARAMETERS() -> (DWM_PRESENT_PARAMETERS, String) { (DWM_PRESENT_PARAMETERS::default(), "DWM_PRESENT_PARAMETERS::default()".to_string()) }
pub fn get_strange_DWM_SHOWCONTACT() -> (DWM_SHOWCONTACT, String) { (DWM_SHOWCONTACT::default(), "DWM_SHOWCONTACT::default()".to_string()) }
pub fn get_strange_DWM_TAB_WINDOW_REQUIREMENTS() -> (DWM_TAB_WINDOW_REQUIREMENTS, String) { (DWM_TAB_WINDOW_REQUIREMENTS::default(), "DWM_TAB_WINDOW_REQUIREMENTS::default()".to_string()) }
pub fn get_strange_DWM_TIMING_INFO() -> (DWM_TIMING_INFO, String) { (DWM_TIMING_INFO::default(), "DWM_TIMING_INFO::default()".to_string()) }
pub fn get_strange_DWRITE_FACTORY_TYPE() -> (DWRITE_FACTORY_TYPE, String) { (DWRITE_FACTORY_TYPE::default(), "DWRITE_FACTORY_TYPE::default()".to_string()) }
pub fn get_strange_DXGI_FORMAT() -> (DXGI_FORMAT, String) { (DXGI_FORMAT::default(), "DXGI_FORMAT::default()".to_string()) }
pub fn get_strange_DXVAHD_DEVICE_USAGE() -> (DXVAHD_DEVICE_USAGE, String) { (DXVAHD_DEVICE_USAGE::default(), "DXVAHD_DEVICE_USAGE::default()".to_string()) }
pub fn get_strange_DYNAMIC_TIME_ZONE_INFORMATION() -> (DYNAMIC_TIME_ZONE_INFORMATION, String) { (DYNAMIC_TIME_ZONE_INFORMATION::default(), "DYNAMIC_TIME_ZONE_INFORMATION::default()".to_string()) }
pub fn get_strange_DispatcherQueueOptions() -> (DispatcherQueueOptions, String) { (DispatcherQueueOptions::default(), "DispatcherQueueOptions::default()".to_string()) }
pub fn get_strange_DnsContextHandle() -> (DnsContextHandle, String) { (DnsContextHandle::default(), "DnsContextHandle::default()".to_string()) }
pub fn get_strange_DockPosition() -> (DockPosition, String) { (DockPosition::default(), "DockPosition::default()".to_string()) }
pub fn get_strange_EAP_METHOD_TYPE() -> (EAP_METHOD_TYPE, String) { (EAP_METHOD_TYPE::default(), "EAP_METHOD_TYPE::default()".to_string()) }
pub fn get_strange_EAllocationType() -> (EAllocationType, String) { (EAllocationType::default(), "EAllocationType::default()".to_string()) }
pub fn get_strange_EDefaultDevmodeType() -> (EDefaultDevmodeType, String) { (EDefaultDevmodeType::default(), "EDefaultDevmodeType::default()".to_string()) }
pub fn get_strange_EFFECTIVE_POWER_MODE_CALLBACK() -> (EFFECTIVE_POWER_MODE_CALLBACK, String) { (EFFECTIVE_POWER_MODE_CALLBACK::default(), "EFFECTIVE_POWER_MODE_CALLBACK::default()".to_string()) }
pub fn get_strange_EMBEDDED_FONT_PRIV_STATUS() -> (EMBEDDED_FONT_PRIV_STATUS, String) { (EMBEDDED_FONT_PRIV_STATUS::default(), "EMBEDDED_FONT_PRIV_STATUS::default()".to_string()) }
pub fn get_strange_EMBED_FONT_CHARSET() -> (EMBED_FONT_CHARSET, String) { (EMBED_FONT_CHARSET::default(), "EMBED_FONT_CHARSET::default()".to_string()) }
pub fn get_strange_EMFINFO() -> (EMFINFO, String) { (EMFINFO::default(), "EMFINFO::default()".to_string()) }
pub fn get_strange_ENABLE_SCROLL_BAR_ARROWS() -> (ENABLE_SCROLL_BAR_ARROWS, String) { (ENABLE_SCROLL_BAR_ARROWS::default(), "ENABLE_SCROLL_BAR_ARROWS::default()".to_string()) }
pub fn get_strange_ENCLAVE_IDENTITY() -> (ENCLAVE_IDENTITY, String) { (ENCLAVE_IDENTITY::default(), "ENCLAVE_IDENTITY::default()".to_string()) }
pub fn get_strange_ENCLAVE_INFORMATION() -> (ENCLAVE_INFORMATION, String) { (ENCLAVE_INFORMATION::default(), "ENCLAVE_INFORMATION::default()".to_string()) }
pub fn get_strange_ENCLAVE_SEALING_IDENTITY_POLICY() -> (ENCLAVE_SEALING_IDENTITY_POLICY, String) { (ENCLAVE_SEALING_IDENTITY_POLICY::default(), "ENCLAVE_SEALING_IDENTITY_POLICY::default()".to_string()) }
pub fn get_strange_ENG_TIME_FIELDS() -> (ENG_TIME_FIELDS, String) { (ENG_TIME_FIELDS::default(), "ENG_TIME_FIELDS::default()".to_string()) }
pub fn get_strange_ENHMETAHEADER() -> (ENHMETAHEADER, String) { (ENHMETAHEADER::default(), "ENHMETAHEADER::default()".to_string()) }
pub fn get_strange_ENHMFENUMPROC() -> (ENHMFENUMPROC, String) { (ENHMFENUMPROC::default(), "ENHMFENUMPROC::default()".to_string()) }
pub fn get_strange_ENUMERATION_BUFFER() -> (ENUMERATION_BUFFER, String) { (ENUMERATION_BUFFER::default(), "ENUMERATION_BUFFER::default()".to_string()) }
pub fn get_strange_ENUMRESLANGPROCA() -> (ENUMRESLANGPROCA, String) { (ENUMRESLANGPROCA::default(), "ENUMRESLANGPROCA::default()".to_string()) }
pub fn get_strange_ENUMRESLANGPROCW() -> (ENUMRESLANGPROCW, String) { (ENUMRESLANGPROCW::default(), "ENUMRESLANGPROCW::default()".to_string()) }
pub fn get_strange_ENUMRESNAMEPROCA() -> (ENUMRESNAMEPROCA, String) { (ENUMRESNAMEPROCA::default(), "ENUMRESNAMEPROCA::default()".to_string()) }
pub fn get_strange_ENUMRESNAMEPROCW() -> (ENUMRESNAMEPROCW, String) { (ENUMRESNAMEPROCW::default(), "ENUMRESNAMEPROCW::default()".to_string()) }
pub fn get_strange_ENUMRESTYPEPROCA() -> (ENUMRESTYPEPROCA, String) { (ENUMRESTYPEPROCA::default(), "ENUMRESTYPEPROCA::default()".to_string()) }
pub fn get_strange_ENUMRESTYPEPROCW() -> (ENUMRESTYPEPROCW, String) { (ENUMRESTYPEPROCW::default(), "ENUMRESTYPEPROCW::default()".to_string()) }
pub fn get_strange_ENUM_DISPLAY_SETTINGS_MODE() -> (ENUM_DISPLAY_SETTINGS_MODE, String) { (ENUM_DISPLAY_SETTINGS_MODE::default(), "ENUM_DISPLAY_SETTINGS_MODE::default()".to_string()) }
pub fn get_strange_ENUM_PROCESS_MODULES_EX_FLAGS() -> (ENUM_PROCESS_MODULES_EX_FLAGS, String) { (ENUM_PROCESS_MODULES_EX_FLAGS::default(), "ENUM_PROCESS_MODULES_EX_FLAGS::default()".to_string()) }
pub fn get_strange_ENUM_SERVICE_STATE() -> (ENUM_SERVICE_STATE, String) { (ENUM_SERVICE_STATE::default(), "ENUM_SERVICE_STATE::default()".to_string()) }
pub fn get_strange_ENUM_SERVICE_STATUSA() -> (ENUM_SERVICE_STATUSA, String) { (ENUM_SERVICE_STATUSA::default(), "ENUM_SERVICE_STATUSA::default()".to_string()) }
pub fn get_strange_ENUM_SERVICE_STATUSW() -> (ENUM_SERVICE_STATUSW, String) { (ENUM_SERVICE_STATUSW::default(), "ENUM_SERVICE_STATUSW::default()".to_string()) }
pub fn get_strange_ENUM_SERVICE_TYPE() -> (ENUM_SERVICE_TYPE, String) { (ENUM_SERVICE_TYPE::default(), "ENUM_SERVICE_TYPE::default()".to_string()) }
pub fn get_strange_EOLE_AUTHENTICATION_CAPABILITIES() -> (EOLE_AUTHENTICATION_CAPABILITIES, String) { (EOLE_AUTHENTICATION_CAPABILITIES::default(), "EOLE_AUTHENTICATION_CAPABILITIES::default()".to_string()) }
pub fn get_strange_EPrintTicketScope() -> (EPrintTicketScope, String) { (EPrintTicketScope::default(), "EPrintTicketScope::default()".to_string()) }
pub fn get_strange_EPrintXPSJobOperation() -> (EPrintXPSJobOperation, String) { (EPrintXPSJobOperation::default(), "EPrintXPSJobOperation::default()".to_string()) }
pub fn get_strange_EPrintXPSJobProgress() -> (EPrintXPSJobProgress, String) { (EPrintXPSJobProgress::default(), "EPrintXPSJobProgress::default()".to_string()) }
pub fn get_strange_ERF() -> (ERF, String) { (ERF::default(), "ERF::default()".to_string()) }
pub fn get_strange_ERole() -> (ERole, String) { (ERole::default(), "ERole::default()".to_string()) }
pub fn get_strange_ETO_OPTIONS() -> (ETO_OPTIONS, String) { (ETO_OPTIONS::default(), "ETO_OPTIONS::default()".to_string()) }
pub fn get_strange_ETW_PROCESS_HANDLE_INFO_TYPE() -> (ETW_PROCESS_HANDLE_INFO_TYPE, String) { (ETW_PROCESS_HANDLE_INFO_TYPE::default(), "ETW_PROCESS_HANDLE_INFO_TYPE::default()".to_string()) }
pub fn get_strange_EVENT_FIELD_TYPE() -> (EVENT_FIELD_TYPE, String) { (EVENT_FIELD_TYPE::default(), "EVENT_FIELD_TYPE::default()".to_string()) }
pub fn get_strange_EVENT_FILTER_DESCRIPTOR() -> (EVENT_FILTER_DESCRIPTOR, String) { (EVENT_FILTER_DESCRIPTOR::default(), "EVENT_FILTER_DESCRIPTOR::default()".to_string()) }
pub fn get_strange_EVENT_INFO_CLASS() -> (EVENT_INFO_CLASS, String) { (EVENT_INFO_CLASS::default(), "EVENT_INFO_CLASS::default()".to_string()) }
pub fn get_strange_EVENT_INSTANCE_INFO() -> (EVENT_INSTANCE_INFO, String) { (EVENT_INSTANCE_INFO::default(), "EVENT_INSTANCE_INFO::default()".to_string()) }
pub fn get_strange_EVENT_MAP_INFO() -> (EVENT_MAP_INFO, String) { (EVENT_MAP_INFO::default(), "EVENT_MAP_INFO::default()".to_string()) }
pub fn get_strange_EVENT_TRACE_CONTROL() -> (EVENT_TRACE_CONTROL, String) { (EVENT_TRACE_CONTROL::default(), "EVENT_TRACE_CONTROL::default()".to_string()) }
pub fn get_strange_EVENT_TRACE_LOGFILEA() -> (EVENT_TRACE_LOGFILEA, String) { (EVENT_TRACE_LOGFILEA::default(), "EVENT_TRACE_LOGFILEA::default()".to_string()) }
pub fn get_strange_EVENT_TRACE_LOGFILEW() -> (EVENT_TRACE_LOGFILEW, String) { (EVENT_TRACE_LOGFILEW::default(), "EVENT_TRACE_LOGFILEW::default()".to_string()) }
pub fn get_strange_EVENT_TRACE_PROPERTIES() -> (EVENT_TRACE_PROPERTIES, String) { (EVENT_TRACE_PROPERTIES::default(), "EVENT_TRACE_PROPERTIES::default()".to_string()) }
pub fn get_strange_EVT_CHANNEL_CONFIG_PROPERTY_ID() -> (EVT_CHANNEL_CONFIG_PROPERTY_ID, String) { (EVT_CHANNEL_CONFIG_PROPERTY_ID::default(), "EVT_CHANNEL_CONFIG_PROPERTY_ID::default()".to_string()) }
pub fn get_strange_EVT_EVENT_METADATA_PROPERTY_ID() -> (EVT_EVENT_METADATA_PROPERTY_ID, String) { (EVT_EVENT_METADATA_PROPERTY_ID::default(), "EVT_EVENT_METADATA_PROPERTY_ID::default()".to_string()) }
pub fn get_strange_EVT_EVENT_PROPERTY_ID() -> (EVT_EVENT_PROPERTY_ID, String) { (EVT_EVENT_PROPERTY_ID::default(), "EVT_EVENT_PROPERTY_ID::default()".to_string()) }
pub fn get_strange_EVT_LOGIN_CLASS() -> (EVT_LOGIN_CLASS, String) { (EVT_LOGIN_CLASS::default(), "EVT_LOGIN_CLASS::default()".to_string()) }
pub fn get_strange_EVT_LOG_PROPERTY_ID() -> (EVT_LOG_PROPERTY_ID, String) { (EVT_LOG_PROPERTY_ID::default(), "EVT_LOG_PROPERTY_ID::default()".to_string()) }
pub fn get_strange_EVT_PUBLISHER_METADATA_PROPERTY_ID() -> (EVT_PUBLISHER_METADATA_PROPERTY_ID, String) { (EVT_PUBLISHER_METADATA_PROPERTY_ID::default(), "EVT_PUBLISHER_METADATA_PROPERTY_ID::default()".to_string()) }
pub fn get_strange_EVT_QUERY_PROPERTY_ID() -> (EVT_QUERY_PROPERTY_ID, String) { (EVT_QUERY_PROPERTY_ID::default(), "EVT_QUERY_PROPERTY_ID::default()".to_string()) }
pub fn get_strange_EVT_SUBSCRIBE_CALLBACK() -> (EVT_SUBSCRIBE_CALLBACK, String) { (EVT_SUBSCRIBE_CALLBACK::default(), "EVT_SUBSCRIBE_CALLBACK::default()".to_string()) }
pub fn get_strange_EVT_VARIANT() -> (EVT_VARIANT, String) { (EVT_VARIANT::default(), "EVT_VARIANT::default()".to_string()) }
pub fn get_strange_EXCEPINFO() -> (EXCEPINFO, String) { (EXCEPINFO::default(), "EXCEPINFO::default()".to_string()) }
pub fn get_strange_EXECUTION_STATE() -> (EXECUTION_STATE, String) { (EXECUTION_STATE::default(), "EXECUTION_STATE::default()".to_string()) }
pub fn get_strange_EXPAND_VIRTUAL_DISK_FLAG() -> (EXPAND_VIRTUAL_DISK_FLAG, String) { (EXPAND_VIRTUAL_DISK_FLAG::default(), "EXPAND_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_EXPLICIT_ACCESS_A() -> (EXPLICIT_ACCESS_A, String) { (EXPLICIT_ACCESS_A::default(), "EXPLICIT_ACCESS_A::default()".to_string()) }
pub fn get_strange_EXPLICIT_ACCESS_W() -> (EXPLICIT_ACCESS_W, String) { (EXPLICIT_ACCESS_W::default(), "EXPLICIT_ACCESS_W::default()".to_string()) }
pub fn get_strange_EXT_FLOOD_FILL_TYPE() -> (EXT_FLOOD_FILL_TYPE, String) { (EXT_FLOOD_FILL_TYPE::default(), "EXT_FLOOD_FILL_TYPE::default()".to_string()) }
pub fn get_strange_EventLogHandle() -> (EventLogHandle, String) { (EventLogHandle::default(), "EventLogHandle::default()".to_string()) }
pub fn get_strange_EventSourceHandle() -> (EventSourceHandle, String) { (EventSourceHandle::default(), "EventSourceHandle::default()".to_string()) }
pub fn get_strange_FARPROC() -> (FARPROC, String) { (FARPROC::default(), "FARPROC::default()".to_string()) }
pub fn get_strange_FDICABINETINFO() -> (FDICABINETINFO, String) { (FDICABINETINFO::default(), "FDICABINETINFO::default()".to_string()) }
pub fn get_strange_FDICREATE_CPU_TYPE() -> (FDICREATE_CPU_TYPE, String) { (FDICREATE_CPU_TYPE::default(), "FDICREATE_CPU_TYPE::default()".to_string()) }
pub fn get_strange_FEEDBACK_TYPE() -> (FEEDBACK_TYPE, String) { (FEEDBACK_TYPE::default(), "FEEDBACK_TYPE::default()".to_string()) }
pub fn get_strange_FH_SERVICE_PIPE_HANDLE() -> (FH_SERVICE_PIPE_HANDLE, String) {    (FH_SERVICE_PIPE_HANDLE::default(), "FH_SERVICE_PIPE_HANDLE::default()".to_string())}
pub fn get_strange_FILETIME() -> (FILETIME, String) {    (FILETIME::default(), "FILETIME::default()".to_string())}
pub fn get_strange_FILE_FLAGS_AND_ATTRIBUTES() -> (FILE_FLAGS_AND_ATTRIBUTES, String) { (FILE_FLAGS_AND_ATTRIBUTES::default(), "FILE_FLAGS_AND_ATTRIBUTES::default()".to_string()) }
pub fn get_strange_FILTER_INFORMATION_CLASS() -> (FILTER_INFORMATION_CLASS, String) { (FILTER_INFORMATION_CLASS::default(), "FILTER_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_FILTER_MESSAGE_HEADER() -> (FILTER_MESSAGE_HEADER, String) { (FILTER_MESSAGE_HEADER::default(), "FILTER_MESSAGE_HEADER::default()".to_string()) }
pub fn get_strange_FILTER_VOLUME_INFORMATION_CLASS() -> (FILTER_VOLUME_INFORMATION_CLASS, String) { (FILTER_VOLUME_INFORMATION_CLASS::default(), "FILTER_VOLUME_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_FINDREPLACEA() -> (FINDREPLACEA, String) { (FINDREPLACEA::default(), "FINDREPLACEA::default()".to_string()) }
pub fn get_strange_FINDREPLACEW() -> (FINDREPLACEW, String) { (FINDREPLACEW::default(), "FINDREPLACEW::default()".to_string()) }
pub fn get_strange_FIXED_INFO_W2KSP1() -> (FIXED_INFO_W2KSP1, String) { (FIXED_INFO_W2KSP1::default(), "FIXED_INFO_W2KSP1::default()".to_string()) }
pub fn get_strange_FN_PROGRESS() -> (FN_PROGRESS, String) { (FN_PROGRESS::default(), "FN_PROGRESS::default()".to_string()) }
pub fn get_strange_FONTDESC() -> (FONTDESC, String) { (FONTDESC::default(), "FONTDESC::default()".to_string()) }
pub fn get_strange_FONTENUMPROCA() -> (FONTENUMPROCA, String) { (FONTENUMPROCA::default(), "FONTENUMPROCA::default()".to_string()) }
pub fn get_strange_FONTENUMPROCW() -> (FONTENUMPROCW, String) { (FONTENUMPROCW::default(), "FONTENUMPROCW::default()".to_string()) }
pub fn get_strange_FONTINFO() -> (FONTINFO, String) { (FONTINFO::default(), "FONTINFO::default()".to_string()) }
pub fn get_strange_FONTOBJ() -> (FONTOBJ, String) { (FONTOBJ::default(), "FONTOBJ::default()".to_string()) }
pub fn get_strange_FONT_CLIP_PRECISION() -> (FONT_CLIP_PRECISION, String) { (FONT_CLIP_PRECISION::default(), "FONT_CLIP_PRECISION::default()".to_string()) }
pub fn get_strange_FONT_LICENSE_PRIVS() -> (FONT_LICENSE_PRIVS, String) { (FONT_LICENSE_PRIVS::default(), "FONT_LICENSE_PRIVS::default()".to_string()) }
pub fn get_strange_FONT_OUTPUT_PRECISION() -> (FONT_OUTPUT_PRECISION, String) { (FONT_OUTPUT_PRECISION::default(), "FONT_OUTPUT_PRECISION::default()".to_string()) }
pub fn get_strange_FONT_PITCH_AND_FAMILY() -> (FONT_PITCH_AND_FAMILY, String) { (FONT_PITCH_AND_FAMILY::default(), "FONT_PITCH_AND_FAMILY::default()".to_string()) }
pub fn get_strange_FONT_QUALITY() -> (FONT_QUALITY, String) { (FONT_QUALITY::default(), "FONT_QUALITY::default()".to_string()) }
pub fn get_strange_FONT_RESOURCE_CHARACTERISTICS() -> (FONT_RESOURCE_CHARACTERISTICS, String) { (FONT_RESOURCE_CHARACTERISTICS::default(), "FONT_RESOURCE_CHARACTERISTICS::default()".to_string()) }
pub fn get_strange_FORCE_LEVEL_FLAGS() -> (FORCE_LEVEL_FLAGS, String) { (FORCE_LEVEL_FLAGS::default(), "FORCE_LEVEL_FLAGS::default()".to_string()) }
pub fn get_strange_FORK_VIRTUAL_DISK_FLAG() -> (FORK_VIRTUAL_DISK_FLAG, String) { (FORK_VIRTUAL_DISK_FLAG::default(), "FORK_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_FORMATETC() -> (FORMATETC, String) { (FORMATETC::default(), "FORMATETC::default()".to_string()) }
pub fn get_strange_FTP_FLAGS() -> (FTP_FLAGS, String) { (FTP_FLAGS::default(), "FTP_FLAGS::default()".to_string()) }
pub fn get_strange_FWPM_CALLOUT_CHANGE_CALLBACK0() -> (FWPM_CALLOUT_CHANGE_CALLBACK0, String) { (FWPM_CALLOUT_CHANGE_CALLBACK0::default(), "FWPM_CALLOUT_CHANGE_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_CONNECTION_CALLBACK0() -> (FWPM_CONNECTION_CALLBACK0, String) { (FWPM_CONNECTION_CALLBACK0::default(), "FWPM_CONNECTION_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_DYNAMIC_KEYWORD_CALLBACK0() -> (FWPM_DYNAMIC_KEYWORD_CALLBACK0, String) { (FWPM_DYNAMIC_KEYWORD_CALLBACK0::default(), "FWPM_DYNAMIC_KEYWORD_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_ENGINE_OPTION() -> (FWPM_ENGINE_OPTION, String) { (FWPM_ENGINE_OPTION::default(), "FWPM_ENGINE_OPTION::default()".to_string()) }
pub fn get_strange_FWPM_FILTER_CHANGE_CALLBACK0() -> (FWPM_FILTER_CHANGE_CALLBACK0, String) { (FWPM_FILTER_CHANGE_CALLBACK0::default(), "FWPM_FILTER_CHANGE_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_NET_EVENT_CALLBACK0() -> (FWPM_NET_EVENT_CALLBACK0, String) { (FWPM_NET_EVENT_CALLBACK0::default(), "FWPM_NET_EVENT_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_NET_EVENT_CALLBACK1() -> (FWPM_NET_EVENT_CALLBACK1, String) { (FWPM_NET_EVENT_CALLBACK1::default(), "FWPM_NET_EVENT_CALLBACK1::default()".to_string()) }
pub fn get_strange_FWPM_NET_EVENT_CALLBACK2() -> (FWPM_NET_EVENT_CALLBACK2, String) { (FWPM_NET_EVENT_CALLBACK2::default(), "FWPM_NET_EVENT_CALLBACK2::default()".to_string()) }
pub fn get_strange_FWPM_NET_EVENT_CALLBACK3() -> (FWPM_NET_EVENT_CALLBACK3, String) { (FWPM_NET_EVENT_CALLBACK3::default(), "FWPM_NET_EVENT_CALLBACK3::default()".to_string()) }
pub fn get_strange_FWPM_NET_EVENT_CALLBACK4() -> (FWPM_NET_EVENT_CALLBACK4, String) { (FWPM_NET_EVENT_CALLBACK4::default(), "FWPM_NET_EVENT_CALLBACK4::default()".to_string()) }
pub fn get_strange_FWPM_PROVIDER_CHANGE_CALLBACK0() -> (FWPM_PROVIDER_CHANGE_CALLBACK0, String) { (FWPM_PROVIDER_CHANGE_CALLBACK0::default(), "FWPM_PROVIDER_CHANGE_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0() -> (FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0, String) { (FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0::default(), "FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_SUBLAYER_CHANGE_CALLBACK0() -> (FWPM_SUBLAYER_CHANGE_CALLBACK0, String) { (FWPM_SUBLAYER_CHANGE_CALLBACK0::default(), "FWPM_SUBLAYER_CHANGE_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_SYSTEM_PORTS_CALLBACK0() -> (FWPM_SYSTEM_PORTS_CALLBACK0, String) { (FWPM_SYSTEM_PORTS_CALLBACK0::default(), "FWPM_SYSTEM_PORTS_CALLBACK0::default()".to_string()) }
pub fn get_strange_FWPM_VSWITCH_EVENT_CALLBACK0() -> (FWPM_VSWITCH_EVENT_CALLBACK0, String) { (FWPM_VSWITCH_EVENT_CALLBACK0::default(), "FWPM_VSWITCH_EVENT_CALLBACK0::default()".to_string()) }
pub fn get_strange_FilterFindHandle() -> (FilterFindHandle, String) { (FilterFindHandle::default(), "FilterFindHandle::default()".to_string()) }
pub fn get_strange_FilterInstanceFindHandle() -> (FilterInstanceFindHandle, String) { (FilterInstanceFindHandle::default(), "FilterInstanceFindHandle::default()".to_string()) }
pub fn get_strange_FilterVolumeFindHandle() -> (FilterVolumeFindHandle, String) { (FilterVolumeFindHandle::default(), "FilterVolumeFindHandle::default()".to_string()) }
pub fn get_strange_FilterVolumeInstanceFindHandle() -> (FilterVolumeInstanceFindHandle, String) { (FilterVolumeInstanceFindHandle::default(), "FilterVolumeInstanceFindHandle::default()".to_string()) }
pub fn get_strange_GCP_RESULTSA() -> (GCP_RESULTSA, String) { (GCP_RESULTSA::default(), "GCP_RESULTSA::default()".to_string()) }
pub fn get_strange_GCP_RESULTSW() -> (GCP_RESULTSW, String) { (GCP_RESULTSW::default(), "GCP_RESULTSW::default()".to_string()) }
pub fn get_strange_GESTURECONFIG() -> (GESTURECONFIG, String) { (GESTURECONFIG::default(), "GESTURECONFIG::default()".to_string()) }
pub fn get_strange_GESTUREINFO() -> (GESTUREINFO, String) { (GESTUREINFO::default(), "GESTUREINFO::default()".to_string()) }
pub fn get_strange_GESTURE_TYPE() -> (GESTURE_TYPE, String) { (GESTURE_TYPE::default(), "GESTURE_TYPE::default()".to_string()) }
pub fn get_strange_GET_ADAPTERS_ADDRESSES_FLAGS() -> (GET_ADAPTERS_ADDRESSES_FLAGS, String) { (GET_ADAPTERS_ADDRESSES_FLAGS::default(), "GET_ADAPTERS_ADDRESSES_FLAGS::default()".to_string()) }
pub fn get_strange_GET_CHARACTER_PLACEMENT_FLAGS() -> (GET_CHARACTER_PLACEMENT_FLAGS, String) { (GET_CHARACTER_PLACEMENT_FLAGS::default(), "GET_CHARACTER_PLACEMENT_FLAGS::default()".to_string()) }
pub fn get_strange_GET_CONVERSION_LIST_FLAG() -> (GET_CONVERSION_LIST_FLAG, String) { (GET_CONVERSION_LIST_FLAG::default(), "GET_CONVERSION_LIST_FLAG::default()".to_string()) }
pub fn get_strange_GET_DCX_FLAGS() -> (GET_DCX_FLAGS, String) { (GET_DCX_FLAGS::default(), "GET_DCX_FLAGS::default()".to_string()) }
pub fn get_strange_GET_DEVICE_CAPS_INDEX() -> (GET_DEVICE_CAPS_INDEX, String) { (GET_DEVICE_CAPS_INDEX::default(), "GET_DEVICE_CAPS_INDEX::default()".to_string()) }
pub fn get_strange_GET_GLYPH_OUTLINE_FORMAT() -> (GET_GLYPH_OUTLINE_FORMAT, String) { (GET_GLYPH_OUTLINE_FORMAT::default(), "GET_GLYPH_OUTLINE_FORMAT::default()".to_string()) }
pub fn get_strange_GET_GUIDE_LINE_TYPE() -> (GET_GUIDE_LINE_TYPE, String) { (GET_GUIDE_LINE_TYPE::default(), "GET_GUIDE_LINE_TYPE::default()".to_string()) }
pub fn get_strange_GET_GUI_RESOURCES_FLAGS() -> (GET_GUI_RESOURCES_FLAGS, String) { (GET_GUI_RESOURCES_FLAGS::default(), "GET_GUI_RESOURCES_FLAGS::default()".to_string()) }
pub fn get_strange_GET_MOUSE_MOVE_POINTS_EX_RESOLUTION() -> (GET_MOUSE_MOVE_POINTS_EX_RESOLUTION, String) { (GET_MOUSE_MOVE_POINTS_EX_RESOLUTION::default(), "GET_MOUSE_MOVE_POINTS_EX_RESOLUTION::default()".to_string()) }
pub fn get_strange_GET_STOCK_OBJECT_FLAGS() -> (GET_STOCK_OBJECT_FLAGS, String) { (GET_STOCK_OBJECT_FLAGS::default(), "GET_STOCK_OBJECT_FLAGS::default()".to_string()) }
pub fn get_strange_GET_STORAGE_DEPENDENCY_FLAG() -> (GET_STORAGE_DEPENDENCY_FLAG, String) { (GET_STORAGE_DEPENDENCY_FLAG::default(), "GET_STORAGE_DEPENDENCY_FLAG::default()".to_string()) }
pub fn get_strange_GET_THEME_BITMAP_FLAGS() -> (GET_THEME_BITMAP_FLAGS, String) { (GET_THEME_BITMAP_FLAGS::default(), "GET_THEME_BITMAP_FLAGS::default()".to_string()) }
pub fn get_strange_GET_VIRTUAL_DISK_INFO() -> (GET_VIRTUAL_DISK_INFO, String) { (GET_VIRTUAL_DISK_INFO::default(), "GET_VIRTUAL_DISK_INFO::default()".to_string()) }
pub fn get_strange_GLOBAL_FILTER() -> (GLOBAL_FILTER, String) { (GLOBAL_FILTER::default(), "GLOBAL_FILTER::default()".to_string()) }
pub fn get_strange_GLOBAL_POWER_POLICY() -> (GLOBAL_POWER_POLICY, String) { (GLOBAL_POWER_POLICY::default(), "GLOBAL_POWER_POLICY::default()".to_string()) }
pub fn get_strange_GLYPHMETRICS() -> (GLYPHMETRICS, String) { (GLYPHMETRICS::default(), "GLYPHMETRICS::default()".to_string()) }
pub fn get_strange_GLYPHMETRICSFLOAT() -> (GLYPHMETRICSFLOAT, String) { (GLYPHMETRICSFLOAT::default(), "GLYPHMETRICSFLOAT::default()".to_string()) }
pub fn get_strange_GLYPHSET() -> (GLYPHSET, String) { (GLYPHSET::default(), "GLYPHSET::default()".to_string()) }
pub fn get_strange_GOBJENUMPROC() -> (GOBJENUMPROC, String) { (GOBJENUMPROC::default(), "GOBJENUMPROC::default()".to_string()) }
pub fn get_strange_GOPHER_ATTRIBUTE_ENUMERATOR() -> (GOPHER_ATTRIBUTE_ENUMERATOR, String) { (GOPHER_ATTRIBUTE_ENUMERATOR::default(), "GOPHER_ATTRIBUTE_ENUMERATOR::default()".to_string()) }
pub fn get_strange_GOPHER_FIND_DATAA() -> (GOPHER_FIND_DATAA, String) { (GOPHER_FIND_DATAA::default(), "GOPHER_FIND_DATAA::default()".to_string()) }
pub fn get_strange_GOPHER_FIND_DATAW() -> (GOPHER_FIND_DATAW, String) { (GOPHER_FIND_DATAW::default(), "GOPHER_FIND_DATAW::default()".to_string()) }
pub fn get_strange_GRADIENT_FILL() -> (GRADIENT_FILL, String) { (GRADIENT_FILL::default(), "GRADIENT_FILL::default()".to_string()) }
pub fn get_strange_GRAPHICS_MODE() -> (GRAPHICS_MODE, String) { (GRAPHICS_MODE::default(), "GRAPHICS_MODE::default()".to_string()) }
pub fn get_strange_GRAYSTRINGPROC() -> (GRAYSTRINGPROC, String) { (GRAYSTRINGPROC::default(), "GRAYSTRINGPROC::default()".to_string()) }
pub fn get_strange_GROUP_AFFINITY() -> (GROUP_AFFINITY, String) { (GROUP_AFFINITY::default(), "GROUP_AFFINITY::default()".to_string()) }
pub fn get_strange_GUID() -> (GUID, String) {    (GUID::default(), "GUID::default()".to_string())}
pub fn get_strange_HACCEL() -> (HACCEL, String) { (HACCEL::default(), "HACCEL::default()".to_string()) }
pub fn get_strange_HACMDRIVER() -> (HACMDRIVER, String) {    (HACMDRIVER::default(), "HACMDRIVER::default()".to_string())}
pub fn get_strange_HACMDRIVERID() -> (HACMDRIVERID, String) { (HACMDRIVERID::default(), "HACMDRIVERID::default()".to_string()) }
pub fn get_strange_HACMOBJ() -> (HACMOBJ, String) { (HACMOBJ::default(), "HACMOBJ::default()".to_string()) }
pub fn get_strange_HACMSTREAM() -> (HACMSTREAM, String) { (HACMSTREAM::default(), "HACMSTREAM::default()".to_string()) }
pub fn get_strange_HAMSICONTEXT() -> (HAMSICONTEXT, String) { (HAMSICONTEXT::default(), "HAMSICONTEXT::default()".to_string()) }
pub fn get_strange_HAMSISESSION() -> (HAMSISESSION, String) { (HAMSISESSION::default(), "HAMSISESSION::default()".to_string()) }
pub fn get_strange_HANDLE() -> (HANDLE, String) {    (HANDLE::default(), "HANDLE::default()".to_string())}
pub fn get_strange_HANDLE_FLAGS() -> (HANDLE_FLAGS, String) { (HANDLE_FLAGS::default(), "HANDLE_FLAGS::default()".to_string()) }
pub fn get_strange_HATCH_BRUSH_STYLE() -> (HATCH_BRUSH_STYLE, String) { (HATCH_BRUSH_STYLE::default(), "HATCH_BRUSH_STYLE::default()".to_string()) }
pub fn get_strange_HBITMAP() -> (HBITMAP, String) {    (HBITMAP::default(), "HBITMAP::default()".to_string())}
pub fn get_strange_HBRUSH() -> (HBRUSH, String) { (HBRUSH::default(), "HBRUSH::default()".to_string()) }
pub fn get_strange_HCERTSTORE() -> (HCERTSTORE, String) { (HCERTSTORE::default(), "HCERTSTORE::default()".to_string()) }
pub fn get_strange_HCMNOTIFICATION() -> (HCMNOTIFICATION, String) { (HCMNOTIFICATION::default(), "HCMNOTIFICATION::default()".to_string()) }
pub fn get_strange_HCOMDB() -> (HCOMDB, String) { (HCOMDB::default(), "HCOMDB::default()".to_string()) }
pub fn get_strange_HCONV() -> (HCONV, String) { (HCONV::default(), "HCONV::default()".to_string()) }
pub fn get_strange_HCONVLIST() -> (HCONVLIST, String) { (HCONVLIST::default(), "HCONVLIST::default()".to_string()) }
pub fn get_strange_HDC() -> (HDC, String) {    (HDC::default(), "HDC::default()".to_string())}
pub fn get_strange_HDC_MAP_MODE() -> (HDC_MAP_MODE, String) { (HDC_MAP_MODE::default(), "HDC_MAP_MODE::default()".to_string()) }
pub fn get_strange_HDDEDATA() -> (HDDEDATA, String) { (HDDEDATA::default(), "HDDEDATA::default()".to_string()) }
pub fn get_strange_HDESK() -> (HDESK, String) { (HDESK::default(), "HDESK::default()".to_string()) }
pub fn get_strange_HDEV() -> (HDEV, String) { (HDEV::default(), "HDEV::default()".to_string()) }
pub fn get_strange_HDIAGNOSTIC_DATA_QUERY_SESSION() -> (HDIAGNOSTIC_DATA_QUERY_SESSION, String) {    (        HDIAGNOSTIC_DATA_QUERY_SESSION::default(),        "HDIAGNOSTIC_DATA_QUERY_SESSION::default()".to_string(),    )}
pub fn get_strange_HDPA() -> (HDPA, String) { (HDPA::default(), "HDPA::default()".to_string()) }
pub fn get_strange_HDRVR() -> (HDRVR, String) { (HDRVR::default(), "HDRVR::default()".to_string()) }
pub fn get_strange_HDSA() -> (HDSA, String) { (HDSA::default(), "HDSA::default()".to_string()) }
pub fn get_strange_HEAPENTRY32() -> (HEAPENTRY32, String) { (HEAPENTRY32::default(), "HEAPENTRY32::default()".to_string()) }
pub fn get_strange_HEAPLIST32() -> (HEAPLIST32, String) { (HEAPLIST32::default(), "HEAPLIST32::default()".to_string()) }
pub fn get_strange_HENHMETAFILE() -> (HENHMETAFILE, String) { (HENHMETAFILE::default(), "HENHMETAFILE::default()".to_string()) }
pub fn get_strange_HFILTER() -> (HFILTER, String) { (HFILTER::default(), "HFILTER::default()".to_string()) }
pub fn get_strange_HFILTER_INSTANCE() -> (HFILTER_INSTANCE, String) { (HFILTER_INSTANCE::default(), "HFILTER_INSTANCE::default()".to_string()) }
pub fn get_strange_HGDIOBJ() -> (HGDIOBJ, String) { (HGDIOBJ::default(), "HGDIOBJ::default()".to_string()) }
pub fn get_strange_HGESTUREINFO() -> (HGESTUREINFO, String) { (HGESTUREINFO::default(), "HGESTUREINFO::default()".to_string()) }
pub fn get_strange_HGLRC() -> (HGLRC, String) { (HGLRC::default(), "HGLRC::default()".to_string()) }
pub fn get_strange_HIC() -> (HIC, String) { (HIC::default(), "HIC::default()".to_string()) }
pub fn get_strange_HICON() -> (HICON, String) { (HICON::default(), "HICON::default()".to_string()) }
pub fn get_strange_HIDD_ATTRIBUTES() -> (HIDD_ATTRIBUTES, String) { (HIDD_ATTRIBUTES::default(), "HIDD_ATTRIBUTES::default()".to_string()) }
pub fn get_strange_HIDD_CONFIGURATION() -> (HIDD_CONFIGURATION, String) { (HIDD_CONFIGURATION::default(), "HIDD_CONFIGURATION::default()".to_string()) }
pub fn get_strange_HIDP_BUTTON_ARRAY_DATA() -> (HIDP_BUTTON_ARRAY_DATA, String) { (HIDP_BUTTON_ARRAY_DATA::default(), "HIDP_BUTTON_ARRAY_DATA::default()".to_string()) }
pub fn get_strange_HIDP_BUTTON_CAPS() -> (HIDP_BUTTON_CAPS, String) { (HIDP_BUTTON_CAPS::default(), "HIDP_BUTTON_CAPS::default()".to_string()) }
pub fn get_strange_HIDP_CAPS() -> (HIDP_CAPS, String) { (HIDP_CAPS::default(), "HIDP_CAPS::default()".to_string()) }
pub fn get_strange_HIDP_DATA() -> (HIDP_DATA, String) { (HIDP_DATA::default(), "HIDP_DATA::default()".to_string()) }
pub fn get_strange_HIDP_EXTENDED_ATTRIBUTES() -> (HIDP_EXTENDED_ATTRIBUTES, String) { (HIDP_EXTENDED_ATTRIBUTES::default(), "HIDP_EXTENDED_ATTRIBUTES::default()".to_string()) }
pub fn get_strange_HIDP_KEYBOARD_DIRECTION() -> (HIDP_KEYBOARD_DIRECTION, String) { (HIDP_KEYBOARD_DIRECTION::default(), "HIDP_KEYBOARD_DIRECTION::default()".to_string()) }
pub fn get_strange_HIDP_KEYBOARD_MODIFIER_STATE() -> (HIDP_KEYBOARD_MODIFIER_STATE, String) { (HIDP_KEYBOARD_MODIFIER_STATE::default(), "HIDP_KEYBOARD_MODIFIER_STATE::default()".to_string()) }
pub fn get_strange_HIDP_LINK_COLLECTION_NODE() -> (HIDP_LINK_COLLECTION_NODE, String) { (HIDP_LINK_COLLECTION_NODE::default(), "HIDP_LINK_COLLECTION_NODE::default()".to_string()) }
pub fn get_strange_HIDP_REPORT_TYPE() -> (HIDP_REPORT_TYPE, String) { (HIDP_REPORT_TYPE::default(), "HIDP_REPORT_TYPE::default()".to_string()) }
pub fn get_strange_HIDP_VALUE_CAPS() -> (HIDP_VALUE_CAPS, String) { (HIDP_VALUE_CAPS::default(), "HIDP_VALUE_CAPS::default()".to_string()) }
pub fn get_strange_HIFTIMESTAMPCHANGE() -> (HIFTIMESTAMPCHANGE, String) { (HIFTIMESTAMPCHANGE::default(), "HIFTIMESTAMPCHANGE::default()".to_string()) }
pub fn get_strange_HIMAGELIST() -> (HIMAGELIST, String) {    (HIMAGELIST::default(), "HIMAGELIST::default()".to_string())}
pub fn get_strange_HIMC() -> (HIMC, String) {    (HIMC::default(), "HIMC::default()".to_string())}
pub fn get_strange_HIMCC() -> (HIMCC, String) {    (HIMCC::default(), "HIMCC::default()".to_string())}
pub fn get_strange_HINSTANCE() -> (HINSTANCE, String) {    (HINSTANCE::default(), "HINSTANCE::default()".to_string())}
pub fn get_strange_HINTERACTIONCONTEXT() -> (HINTERACTIONCONTEXT, String) {    (HINTERACTIONCONTEXT::default(), "HINTERACTIONCONTEXT::default()".to_string())}
pub fn get_strange_HKEY() -> (HKEY, String) {    (HKEY::default(), "HKEY::default()".to_string())}
pub fn get_strange_HKL() -> (HKL, String) {    (HKL::default(), "HKL::default()".to_string())}
pub fn get_strange_HLOG() -> (HLOG, String) { (HLOG::default(), "HLOG::default()".to_string()) }
pub fn get_strange_HMENU() -> (HMENU, String) {    (HMENU::default(), "HMENU::default()".to_string())}
pub fn get_strange_HMETAFILE() -> (HMETAFILE, String) { (HMETAFILE::default(), "HMETAFILE::default()".to_string()) }
pub fn get_strange_HMIDI() -> (HMIDI, String) { (HMIDI::default(), "HMIDI::default()".to_string()) }
pub fn get_strange_HMIDIIN() -> (HMIDIIN, String) { (HMIDIIN::default(), "HMIDIIN::default()".to_string()) }
pub fn get_strange_HMIDIOUT() -> (HMIDIOUT, String) { (HMIDIOUT::default(), "HMIDIOUT::default()".to_string()) }
pub fn get_strange_HMIDISTRM() -> (HMIDISTRM, String) { (HMIDISTRM::default(), "HMIDISTRM::default()".to_string()) }
pub fn get_strange_HMIXER() -> (HMIXER, String) { (HMIXER::default(), "HMIXER::default()".to_string()) }
pub fn get_strange_HMIXEROBJ() -> (HMIXEROBJ, String) { (HMIXEROBJ::default(), "HMIXEROBJ::default()".to_string()) }
pub fn get_strange_HMMIO() -> (HMMIO, String) { (HMMIO::default(), "HMMIO::default()".to_string()) }
pub fn get_strange_HMONITOR() -> (HMONITOR, String) {    (HMONITOR::default(), "HMONITOR::default()".to_string())}
pub fn get_strange_HOLD_PARAMETER() -> (HOLD_PARAMETER, String) { (HOLD_PARAMETER::default(), "HOLD_PARAMETER::default()".to_string()) }
pub fn get_strange_HOT_KEY_MODIFIERS() -> (HOT_KEY_MODIFIERS, String) { (HOT_KEY_MODIFIERS::default(), "HOT_KEY_MODIFIERS::default()".to_string()) }
pub fn get_strange_HPALETTE() -> (HPALETTE, String) { (HPALETTE::default(), "HPALETTE::default()".to_string()) }
pub fn get_strange_HPCON() -> (HPCON, String) { (HPCON::default(), "HPCON::default()".to_string()) }
pub fn get_strange_HPOWERNOTIFY() -> (HPOWERNOTIFY, String) { (HPOWERNOTIFY::default(), "HPOWERNOTIFY::default()".to_string()) }
pub fn get_strange_HPROPSHEETPAGE() -> (HPROPSHEETPAGE, String) { (HPROPSHEETPAGE::default(), "HPROPSHEETPAGE::default()".to_string()) }
pub fn get_strange_HPSS() -> (HPSS, String) { (HPSS::default(), "HPSS::default()".to_string()) }
pub fn get_strange_HPSSWALK() -> (HPSSWALK, String) { (HPSSWALK::default(), "HPSSWALK::default()".to_string()) }
pub fn get_strange_HPTPROVIDER() -> (HPTPROVIDER, String) { (HPTPROVIDER::default(), "HPTPROVIDER::default()".to_string()) }
pub fn get_strange_HRAWINPUT() -> (HRAWINPUT, String) { (HRAWINPUT::default(), "HRAWINPUT::default()".to_string()) }
pub fn get_strange_HRECOCONTEXT() -> (HRECOCONTEXT, String) { (HRECOCONTEXT::default(), "HRECOCONTEXT::default()".to_string()) }
pub fn get_strange_HRECOGNIZER() -> (HRECOGNIZER, String) { (HRECOGNIZER::default(), "HRECOGNIZER::default()".to_string()) }
pub fn get_strange_HRECOWORDLIST() -> (HRECOWORDLIST, String) { (HRECOWORDLIST::default(), "HRECOWORDLIST::default()".to_string()) }
pub fn get_strange_HREPORT() -> (HREPORT, String) { (HREPORT::default(), "HREPORT::default()".to_string()) }
pub fn get_strange_HREPORTSTORE() -> (HREPORTSTORE, String) { (HREPORTSTORE::default(), "HREPORTSTORE::default()".to_string()) }
pub fn get_strange_HRESULT() -> (HRESULT, String) { (HRESULT::default(), "HRESULT::default()".to_string()) }
pub fn get_strange_HRGN() -> (HRGN, String) {    (HRGN::default(), "HRGN::default()".to_string())}
pub fn get_strange_HRSRC() -> (HRSRC, String) { (HRSRC::default(), "HRSRC::default()".to_string()) }
pub fn get_strange_HSEMAPHORE() -> (HSEMAPHORE, String) { (HSEMAPHORE::default(), "HSEMAPHORE::default()".to_string()) }
pub fn get_strange_HSE_VERSION_INFO() -> (HSE_VERSION_INFO, String) { (HSE_VERSION_INFO::default(), "HSE_VERSION_INFO::default()".to_string()) }
pub fn get_strange_HSTRING() -> (HSTRING, String) {    (HSTRING::default(), "HSTRING::default()".to_string())}
pub fn get_strange_HSTRING_BUFFER() -> (HSTRING_BUFFER, String) { (HSTRING_BUFFER::default(), "HSTRING_BUFFER::default()".to_string()) }
pub fn get_strange_HSTRING_HEADER() -> (HSTRING_HEADER, String) { (HSTRING_HEADER::default(), "HSTRING_HEADER::default()".to_string()) }
pub fn get_strange_HSURF() -> (HSURF, String) { (HSURF::default(), "HSURF::default()".to_string()) }
pub fn get_strange_HSWDEVICE() -> (HSWDEVICE, String) { (HSWDEVICE::default(), "HSWDEVICE::default()".to_string()) }
pub fn get_strange_HSYNTHETICPOINTERDEVICE() -> (HSYNTHETICPOINTERDEVICE, String) { (HSYNTHETICPOINTERDEVICE::default(), "HSYNTHETICPOINTERDEVICE::default()".to_string()) }
pub fn get_strange_HSZ() -> (HSZ, String) { (HSZ::default(), "HSZ::default()".to_string()) }
pub fn get_strange_HTOUCHINPUT() -> (HTOUCHINPUT, String) { (HTOUCHINPUT::default(), "HTOUCHINPUT::default()".to_string()) }
pub fn get_strange_HTTPAPI_VERSION() -> (HTTPAPI_VERSION, String) { (HTTPAPI_VERSION::default(), "HTTPAPI_VERSION::default()".to_string()) }
pub fn get_strange_HTTP_ADDREQ_FLAG() -> (HTTP_ADDREQ_FLAG, String) { (HTTP_ADDREQ_FLAG::default(), "HTTP_ADDREQ_FLAG::default()".to_string()) }
pub fn get_strange_HTTP_BYTE_RANGE() -> (HTTP_BYTE_RANGE, String) { (HTTP_BYTE_RANGE::default(), "HTTP_BYTE_RANGE::default()".to_string()) }
pub fn get_strange_HTTP_CACHE_POLICY() -> (HTTP_CACHE_POLICY, String) { (HTTP_CACHE_POLICY::default(), "HTTP_CACHE_POLICY::default()".to_string()) }
pub fn get_strange_HTTP_DATA_CHUNK() -> (HTTP_DATA_CHUNK, String) { (HTTP_DATA_CHUNK::default(), "HTTP_DATA_CHUNK::default()".to_string()) }
pub fn get_strange_HTTP_FEATURE_ID() -> (HTTP_FEATURE_ID, String) { (HTTP_FEATURE_ID::default(), "HTTP_FEATURE_ID::default()".to_string()) }
pub fn get_strange_HTTP_FILTER_CONTEXT() -> (HTTP_FILTER_CONTEXT, String) { (HTTP_FILTER_CONTEXT::default(), "HTTP_FILTER_CONTEXT::default()".to_string()) }
pub fn get_strange_HTTP_FILTER_VERSION() -> (HTTP_FILTER_VERSION, String) { (HTTP_FILTER_VERSION::default(), "HTTP_FILTER_VERSION::default()".to_string()) }
pub fn get_strange_HTTP_INITIALIZE() -> (HTTP_INITIALIZE, String) { (HTTP_INITIALIZE::default(), "HTTP_INITIALIZE::default()".to_string()) }
pub fn get_strange_HTTP_LOG_DATA() -> (HTTP_LOG_DATA, String) { (HTTP_LOG_DATA::default(), "HTTP_LOG_DATA::default()".to_string()) }
pub fn get_strange_HTTP_PUSH_NOTIFICATION_STATUS() -> (HTTP_PUSH_NOTIFICATION_STATUS, String) { (HTTP_PUSH_NOTIFICATION_STATUS::default(), "HTTP_PUSH_NOTIFICATION_STATUS::default()".to_string()) }
pub fn get_strange_HTTP_PUSH_WAIT_HANDLE() -> (HTTP_PUSH_WAIT_HANDLE, String) { (HTTP_PUSH_WAIT_HANDLE::default(), "HTTP_PUSH_WAIT_HANDLE::default()".to_string()) }
pub fn get_strange_HTTP_PUSH_WAIT_TYPE() -> (HTTP_PUSH_WAIT_TYPE, String) { (HTTP_PUSH_WAIT_TYPE::default(), "HTTP_PUSH_WAIT_TYPE::default()".to_string()) }
pub fn get_strange_HTTP_RECEIVE_HTTP_REQUEST_FLAGS() -> (HTTP_RECEIVE_HTTP_REQUEST_FLAGS, String) { (HTTP_RECEIVE_HTTP_REQUEST_FLAGS::default(), "HTTP_RECEIVE_HTTP_REQUEST_FLAGS::default()".to_string()) }
pub fn get_strange_HTTP_REQUEST_PROPERTY() -> (HTTP_REQUEST_PROPERTY, String) { (HTTP_REQUEST_PROPERTY::default(), "HTTP_REQUEST_PROPERTY::default()".to_string()) }
pub fn get_strange_HTTP_REQUEST_V2() -> (HTTP_REQUEST_V2, String) { (HTTP_REQUEST_V2::default(), "HTTP_REQUEST_V2::default()".to_string()) }
pub fn get_strange_HTTP_RESPONSE_V2() -> (HTTP_RESPONSE_V2, String) { (HTTP_RESPONSE_V2::default(), "HTTP_RESPONSE_V2::default()".to_string()) }
pub fn get_strange_HTTP_SERVER_PROPERTY() -> (HTTP_SERVER_PROPERTY, String) { (HTTP_SERVER_PROPERTY::default(), "HTTP_SERVER_PROPERTY::default()".to_string()) }
pub fn get_strange_HTTP_SERVICE_CONFIG_ID() -> (HTTP_SERVICE_CONFIG_ID, String) { (HTTP_SERVICE_CONFIG_ID::default(), "HTTP_SERVICE_CONFIG_ID::default()".to_string()) }
pub fn get_strange_HTTP_SSL_CLIENT_CERT_INFO() -> (HTTP_SSL_CLIENT_CERT_INFO, String) { (HTTP_SSL_CLIENT_CERT_INFO::default(), "HTTP_SSL_CLIENT_CERT_INFO::default()".to_string()) }
pub fn get_strange_HTTP_VERB() -> (HTTP_VERB, String) { (HTTP_VERB::default(), "HTTP_VERB::default()".to_string()) }
pub fn get_strange_HTTP_WEB_SOCKET_BUFFER_TYPE() -> (HTTP_WEB_SOCKET_BUFFER_TYPE, String) { (HTTP_WEB_SOCKET_BUFFER_TYPE::default(), "HTTP_WEB_SOCKET_BUFFER_TYPE::default()".to_string()) }
pub fn get_strange_HUIAEVENT() -> (HUIAEVENT, String) { (HUIAEVENT::default(), "HUIAEVENT::default()".to_string()) }
pub fn get_strange_HUIANODE() -> (HUIANODE, String) { (HUIANODE::default(), "HUIANODE::default()".to_string()) }
pub fn get_strange_HUIAPATTERNOBJECT() -> (HUIAPATTERNOBJECT, String) {    (HUIAPATTERNOBJECT::default(), "HUIAPATTERNOBJECT::default()".to_string())}
pub fn get_strange_HUIATEXTRANGE() -> (HUIATEXTRANGE, String) { (HUIATEXTRANGE::default(), "HUIATEXTRANGE::default()".to_string()) }
pub fn get_strange_HWAVEIN() -> (HWAVEIN, String) { (HWAVEIN::default(), "HWAVEIN::default()".to_string()) }
pub fn get_strange_HWAVEOUT() -> (HWAVEOUT, String) {    (HWAVEOUT::default(), "HWAVEOUT::default()".to_string())}
pub fn get_strange_HWINEVENTHOOK() -> (HWINEVENTHOOK, String) { (HWINEVENTHOOK::default(), "HWINEVENTHOOK::default()".to_string()) }
pub fn get_strange_HWINSTA() -> (HWINSTA, String) { (HWINSTA::default(), "HWINSTA::default()".to_string()) }
pub fn get_strange_HWND() -> (HWND, String) {    (HWND::default(), "HWND::default()".to_string())}
pub fn get_strange_HWProfileInfo_sA() -> (HWProfileInfo_sA, String) { (HWProfileInfo_sA::default(), "HWProfileInfo_sA::default()".to_string()) }
pub fn get_strange_HWProfileInfo_sW() -> (HWProfileInfo_sW, String) { (HWProfileInfo_sW::default(), "HWProfileInfo_sW::default()".to_string()) }
pub fn get_strange_ICINFO() -> (ICINFO, String) { (ICINFO::default(), "ICINFO::default()".to_string()) }
pub fn get_strange_IKEEXT_STATISTICS0() -> (IKEEXT_STATISTICS0, String) { (IKEEXT_STATISTICS0::default(), "IKEEXT_STATISTICS0::default()".to_string()) }
pub fn get_strange_IKEEXT_STATISTICS1() -> (IKEEXT_STATISTICS1, String) { (IKEEXT_STATISTICS1::default(), "IKEEXT_STATISTICS1::default()".to_string()) }
pub fn get_strange_IMAGEINFO() -> (IMAGEINFO, String) { (IMAGEINFO::default(), "IMAGEINFO::default()".to_string()) }
pub fn get_strange_IMEMENUITEMINFOA() -> (IMEMENUITEMINFOA, String) { (IMEMENUITEMINFOA::default(), "IMEMENUITEMINFOA::default()".to_string()) }
pub fn get_strange_IMEMENUITEMINFOW() -> (IMEMENUITEMINFOW, String) { (IMEMENUITEMINFOW::default(), "IMEMENUITEMINFOW::default()".to_string()) }
pub fn get_strange_INFCONTEXT() -> (INFCONTEXT, String) { (INFCONTEXT::default(), "INFCONTEXT::default()".to_string()) }
pub fn get_strange_INHERITED_FROMA() -> (INHERITED_FROMA, String) { (INHERITED_FROMA::default(), "INHERITED_FROMA::default()".to_string()) }
pub fn get_strange_INHERITED_FROMW() -> (INHERITED_FROMW, String) { (INHERITED_FROMW::default(), "INHERITED_FROMW::default()".to_string()) }
pub fn get_strange_INPUT_MESSAGE_SOURCE() -> (INPUT_MESSAGE_SOURCE, String) { (INPUT_MESSAGE_SOURCE::default(), "INPUT_MESSAGE_SOURCE::default()".to_string()) }
pub fn get_strange_INPUT_RECORD() -> (INPUT_RECORD, String) { (INPUT_RECORD::default(), "INPUT_RECORD::default()".to_string()) }
pub fn get_strange_INPUT_TRANSFORM() -> (INPUT_TRANSFORM, String) { (INPUT_TRANSFORM::default(), "INPUT_TRANSFORM::default()".to_string()) }
pub fn get_strange_INTERACTION_CONTEXT_CONFIGURATION() -> (INTERACTION_CONTEXT_CONFIGURATION, String) { (INTERACTION_CONTEXT_CONFIGURATION::default(), "INTERACTION_CONTEXT_CONFIGURATION::default()".to_string()) }
pub fn get_strange_INTERACTION_STATE() -> (INTERACTION_STATE, String) { (INTERACTION_STATE::default(), "INTERACTION_STATE::default()".to_string()) }
pub fn get_strange_INTERFACEDATA() -> (INTERFACEDATA, String) { (INTERFACEDATA::default(), "INTERFACEDATA::default()".to_string()) }
pub fn get_strange_INTERFACE_HARDWARE_CROSSTIMESTAMP() -> (INTERFACE_HARDWARE_CROSSTIMESTAMP, String) { (INTERFACE_HARDWARE_CROSSTIMESTAMP::default(), "INTERFACE_HARDWARE_CROSSTIMESTAMP::default()".to_string()) }
pub fn get_strange_INTERFACE_TIMESTAMP_CAPABILITIES() -> (INTERFACE_TIMESTAMP_CAPABILITIES, String) { (INTERFACE_TIMESTAMP_CAPABILITIES::default(), "INTERFACE_TIMESTAMP_CAPABILITIES::default()".to_string()) }
pub fn get_strange_INTERNET_BUFFERSA() -> (INTERNET_BUFFERSA, String) { (INTERNET_BUFFERSA::default(), "INTERNET_BUFFERSA::default()".to_string()) }
pub fn get_strange_INTERNET_BUFFERSW() -> (INTERNET_BUFFERSW, String) { (INTERNET_BUFFERSW::default(), "INTERNET_BUFFERSW::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_CONFIG_INFOA() -> (INTERNET_CACHE_CONFIG_INFOA, String) { (INTERNET_CACHE_CONFIG_INFOA::default(), "INTERNET_CACHE_CONFIG_INFOA::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_CONFIG_INFOW() -> (INTERNET_CACHE_CONFIG_INFOW, String) { (INTERNET_CACHE_CONFIG_INFOW::default(), "INTERNET_CACHE_CONFIG_INFOW::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_CONTAINER_INFOA() -> (INTERNET_CACHE_CONTAINER_INFOA, String) { (INTERNET_CACHE_CONTAINER_INFOA::default(), "INTERNET_CACHE_CONTAINER_INFOA::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_CONTAINER_INFOW() -> (INTERNET_CACHE_CONTAINER_INFOW, String) { (INTERNET_CACHE_CONTAINER_INFOW::default(), "INTERNET_CACHE_CONTAINER_INFOW::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_ENTRY_INFOA() -> (INTERNET_CACHE_ENTRY_INFOA, String) { (INTERNET_CACHE_ENTRY_INFOA::default(), "INTERNET_CACHE_ENTRY_INFOA::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_ENTRY_INFOW() -> (INTERNET_CACHE_ENTRY_INFOW, String) { (INTERNET_CACHE_ENTRY_INFOW::default(), "INTERNET_CACHE_ENTRY_INFOW::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_GROUP_INFOA() -> (INTERNET_CACHE_GROUP_INFOA, String) { (INTERNET_CACHE_GROUP_INFOA::default(), "INTERNET_CACHE_GROUP_INFOA::default()".to_string()) }
pub fn get_strange_INTERNET_CACHE_GROUP_INFOW() -> (INTERNET_CACHE_GROUP_INFOW, String) { (INTERNET_CACHE_GROUP_INFOW::default(), "INTERNET_CACHE_GROUP_INFOW::default()".to_string()) }
pub fn get_strange_INTERNET_CONNECTION() -> (INTERNET_CONNECTION, String) { (INTERNET_CONNECTION::default(), "INTERNET_CONNECTION::default()".to_string()) }
pub fn get_strange_INTERNET_COOKIE2() -> (INTERNET_COOKIE2, String) { (INTERNET_COOKIE2::default(), "INTERNET_COOKIE2::default()".to_string()) }
pub fn get_strange_INTLIST() -> (INTLIST, String) { (INTLIST::default(), "INTLIST::default()".to_string()) }
pub fn get_strange_IO_COUNTERS() -> (IO_COUNTERS, String) { (IO_COUNTERS::default(), "IO_COUNTERS::default()".to_string()) }
pub fn get_strange_IPSEC_DOSP_STATISTICS0() -> (IPSEC_DOSP_STATISTICS0, String) { (IPSEC_DOSP_STATISTICS0::default(), "IPSEC_DOSP_STATISTICS0::default()".to_string()) }
pub fn get_strange_IPSEC_STATISTICS0() -> (IPSEC_STATISTICS0, String) { (IPSEC_STATISTICS0::default(), "IPSEC_STATISTICS0::default()".to_string()) }
pub fn get_strange_IPSEC_STATISTICS1() -> (IPSEC_STATISTICS1, String) { (IPSEC_STATISTICS1::default(), "IPSEC_STATISTICS1::default()".to_string()) }
pub fn get_strange_IP_ADAPTER_ADDRESSES_LH() -> (IP_ADAPTER_ADDRESSES_LH, String) { (IP_ADAPTER_ADDRESSES_LH::default(), "IP_ADAPTER_ADDRESSES_LH::default()".to_string()) }
pub fn get_strange_IP_ADAPTER_INFO() -> (IP_ADAPTER_INFO, String) { (IP_ADAPTER_INFO::default(), "IP_ADAPTER_INFO::default()".to_string()) }
pub fn get_strange_IP_INTERFACE_INFO() -> (IP_INTERFACE_INFO, String) { (IP_INTERFACE_INFO::default(), "IP_INTERFACE_INFO::default()".to_string()) }
pub fn get_strange_JET_API_PTR() -> (JET_API_PTR, String) { (JET_API_PTR::default(), "JET_API_PTR::default()".to_string()) }
pub fn get_strange_JET_CALLBACK() -> (JET_CALLBACK, String) { (JET_CALLBACK::default(), "JET_CALLBACK::default()".to_string()) }
pub fn get_strange_JET_COMMIT_ID() -> (JET_COMMIT_ID, String) { (JET_COMMIT_ID::default(), "JET_COMMIT_ID::default()".to_string()) }
pub fn get_strange_JET_HANDLE() -> (JET_HANDLE, String) { (JET_HANDLE::default(), "JET_HANDLE::default()".to_string()) }
pub fn get_strange_JET_INSTANCE() -> (JET_INSTANCE, String) {    (JET_INSTANCE::default(), "JET_INSTANCE::default()".to_string())}
pub fn get_strange_JET_LOGINFO_A() -> (JET_LOGINFO_A, String) { (JET_LOGINFO_A::default(), "JET_LOGINFO_A::default()".to_string()) }
pub fn get_strange_JET_LOGINFO_W() -> (JET_LOGINFO_W, String) { (JET_LOGINFO_W::default(), "JET_LOGINFO_W::default()".to_string()) }
pub fn get_strange_JET_LS() -> (JET_LS, String) { (JET_LS::default(), "JET_LS::default()".to_string()) }
pub fn get_strange_JET_OSSNAPID() -> (JET_OSSNAPID, String) { (JET_OSSNAPID::default(), "JET_OSSNAPID::default()".to_string()) }
pub fn get_strange_JET_PFNREALLOC() -> (JET_PFNREALLOC, String) { (JET_PFNREALLOC::default(), "JET_PFNREALLOC::default()".to_string()) }
pub fn get_strange_JET_PFNSTATUS() -> (JET_PFNSTATUS, String) { (JET_PFNSTATUS::default(), "JET_PFNSTATUS::default()".to_string()) }
pub fn get_strange_JET_RECORDLIST() -> (JET_RECORDLIST, String) { (JET_RECORDLIST::default(), "JET_RECORDLIST::default()".to_string()) }
pub fn get_strange_JET_RECPOS() -> (JET_RECPOS, String) { (JET_RECPOS::default(), "JET_RECPOS::default()".to_string()) }
pub fn get_strange_JET_RECSIZE() -> (JET_RECSIZE, String) { (JET_RECSIZE::default(), "JET_RECSIZE::default()".to_string()) }
pub fn get_strange_JET_RECSIZE2() -> (JET_RECSIZE2, String) { (JET_RECSIZE2::default(), "JET_RECSIZE2::default()".to_string()) }
pub fn get_strange_JET_RETINFO() -> (JET_RETINFO, String) { (JET_RETINFO::default(), "JET_RETINFO::default()".to_string()) }
pub fn get_strange_JET_RETRIEVECOLUMN() -> (JET_RETRIEVECOLUMN, String) { (JET_RETRIEVECOLUMN::default(), "JET_RETRIEVECOLUMN::default()".to_string()) }
pub fn get_strange_JET_SESID() -> (JET_SESID, String) {    (JET_SESID::default(), "JET_SESID::default()".to_string())}
pub fn get_strange_JET_TABLECREATE2_A() -> (JET_TABLECREATE2_A, String) { (JET_TABLECREATE2_A::default(), "JET_TABLECREATE2_A::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE2_W() -> (JET_TABLECREATE2_W, String) { (JET_TABLECREATE2_W::default(), "JET_TABLECREATE2_W::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE3_A() -> (JET_TABLECREATE3_A, String) { (JET_TABLECREATE3_A::default(), "JET_TABLECREATE3_A::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE3_W() -> (JET_TABLECREATE3_W, String) { (JET_TABLECREATE3_W::default(), "JET_TABLECREATE3_W::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE4_A() -> (JET_TABLECREATE4_A, String) { (JET_TABLECREATE4_A::default(), "JET_TABLECREATE4_A::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE4_W() -> (JET_TABLECREATE4_W, String) { (JET_TABLECREATE4_W::default(), "JET_TABLECREATE4_W::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE_A() -> (JET_TABLECREATE_A, String) { (JET_TABLECREATE_A::default(), "JET_TABLECREATE_A::default()".to_string()) }
pub fn get_strange_JET_TABLECREATE_W() -> (JET_TABLECREATE_W, String) { (JET_TABLECREATE_W::default(), "JET_TABLECREATE_W::default()".to_string()) }
pub fn get_strange_JET_TABLEID() -> (JET_TABLEID, String) {    (JET_TABLEID::default(), "JET_TABLEID::default()".to_string())}
pub fn get_strange_JOBOBJECTINFOCLASS() -> (JOBOBJECTINFOCLASS, String) { (JOBOBJECTINFOCLASS::default(), "JOBOBJECTINFOCLASS::default()".to_string()) }
pub fn get_strange_JOYCAPSA() -> (JOYCAPSA, String) { (JOYCAPSA::default(), "JOYCAPSA::default()".to_string()) }
pub fn get_strange_JOYCAPSW() -> (JOYCAPSW, String) { (JOYCAPSW::default(), "JOYCAPSW::default()".to_string()) }
pub fn get_strange_JOYINFO() -> (JOYINFO, String) { (JOYINFO::default(), "JOYINFO::default()".to_string()) }
pub fn get_strange_JOYINFOEX() -> (JOYINFOEX, String) { (JOYINFOEX::default(), "JOYINFOEX::default()".to_string()) }
pub fn get_strange_KERNINGPAIR() -> (KERNINGPAIR, String) { (KERNINGPAIR::default(), "KERNINGPAIR::default()".to_string()) }
pub fn get_strange_KEYBD_EVENT_FLAGS() -> (KEYBD_EVENT_FLAGS, String) { (KEYBD_EVENT_FLAGS::default(), "KEYBD_EVENT_FLAGS::default()".to_string()) }
pub fn get_strange_KeyCredentialManagerOperationErrorStates() -> (KeyCredentialManagerOperationErrorStates, String) { (KeyCredentialManagerOperationErrorStates::default(), "KeyCredentialManagerOperationErrorStates::default()".to_string()) }
pub fn get_strange_KeyCredentialManagerOperationType() -> (KeyCredentialManagerOperationType, String) { (KeyCredentialManagerOperationType::default(), "KeyCredentialManagerOperationType::default()".to_string()) }
pub fn get_strange_LASTINPUTINFO() -> (LASTINPUTINFO, String) { (LASTINPUTINFO::default(), "LASTINPUTINFO::default()".to_string()) }
pub fn get_strange_LATENCY_TIME() -> (LATENCY_TIME, String) { (LATENCY_TIME::default(), "LATENCY_TIME::default()".to_string()) }
pub fn get_strange_LAYERPLANEDESCRIPTOR() -> (LAYERPLANEDESCRIPTOR, String) { (LAYERPLANEDESCRIPTOR::default(), "LAYERPLANEDESCRIPTOR::default()".to_string()) }
pub fn get_strange_LDAPMessage() -> (LDAPMessage, String) { (LDAPMessage::default(), "LDAPMessage::default()".to_string()) }
pub fn get_strange_LDAP_BERVAL() -> (LDAP_BERVAL, String) { (LDAP_BERVAL::default(), "LDAP_BERVAL::default()".to_string()) }
pub fn get_strange_LDAP_TIMEVAL() -> (LDAP_TIMEVAL, String) { (LDAP_TIMEVAL::default(), "LDAP_TIMEVAL::default()".to_string()) }
pub fn get_strange_LINEADDRESSCAPS() -> (LINEADDRESSCAPS, String) { (LINEADDRESSCAPS::default(), "LINEADDRESSCAPS::default()".to_string()) }
pub fn get_strange_LINEADDRESSSTATUS() -> (LINEADDRESSSTATUS, String) { (LINEADDRESSSTATUS::default(), "LINEADDRESSSTATUS::default()".to_string()) }
pub fn get_strange_LINEAGENTACTIVITYLIST() -> (LINEAGENTACTIVITYLIST, String) { (LINEAGENTACTIVITYLIST::default(), "LINEAGENTACTIVITYLIST::default()".to_string()) }
pub fn get_strange_LINEAGENTCAPS() -> (LINEAGENTCAPS, String) { (LINEAGENTCAPS::default(), "LINEAGENTCAPS::default()".to_string()) }
pub fn get_strange_LINEAGENTGROUPLIST() -> (LINEAGENTGROUPLIST, String) { (LINEAGENTGROUPLIST::default(), "LINEAGENTGROUPLIST::default()".to_string()) }
pub fn get_strange_LINEAGENTINFO() -> (LINEAGENTINFO, String) { (LINEAGENTINFO::default(), "LINEAGENTINFO::default()".to_string()) }
pub fn get_strange_LINEAGENTSESSIONINFO() -> (LINEAGENTSESSIONINFO, String) { (LINEAGENTSESSIONINFO::default(), "LINEAGENTSESSIONINFO::default()".to_string()) }
pub fn get_strange_LINEAGENTSESSIONLIST() -> (LINEAGENTSESSIONLIST, String) { (LINEAGENTSESSIONLIST::default(), "LINEAGENTSESSIONLIST::default()".to_string()) }
pub fn get_strange_LINEAGENTSTATUS() -> (LINEAGENTSTATUS, String) { (LINEAGENTSTATUS::default(), "LINEAGENTSTATUS::default()".to_string()) }
pub fn get_strange_LINEATTRS() -> (LINEATTRS, String) { (LINEATTRS::default(), "LINEATTRS::default()".to_string()) }
pub fn get_strange_LINECALLBACK() -> (LINECALLBACK, String) { (LINECALLBACK::default(), "LINECALLBACK::default()".to_string()) }
pub fn get_strange_LINECALLINFO() -> (LINECALLINFO, String) { (LINECALLINFO::default(), "LINECALLINFO::default()".to_string()) }
pub fn get_strange_LINECALLLIST() -> (LINECALLLIST, String) { (LINECALLLIST::default(), "LINECALLLIST::default()".to_string()) }
pub fn get_strange_LINECALLSTATUS() -> (LINECALLSTATUS, String) { (LINECALLSTATUS::default(), "LINECALLSTATUS::default()".to_string()) }
pub fn get_strange_LINECOUNTRYLIST() -> (LINECOUNTRYLIST, String) { (LINECOUNTRYLIST::default(), "LINECOUNTRYLIST::default()".to_string()) }
pub fn get_strange_LINEDDAPROC() -> (LINEDDAPROC, String) { (LINEDDAPROC::default(), "LINEDDAPROC::default()".to_string()) }
pub fn get_strange_LINEDEVCAPS() -> (LINEDEVCAPS, String) { (LINEDEVCAPS::default(), "LINEDEVCAPS::default()".to_string()) }
pub fn get_strange_LINEDEVSTATUS() -> (LINEDEVSTATUS, String) { (LINEDEVSTATUS::default(), "LINEDEVSTATUS::default()".to_string()) }
pub fn get_strange_LINEEXTENSIONID() -> (LINEEXTENSIONID, String) { (LINEEXTENSIONID::default(), "LINEEXTENSIONID::default()".to_string()) }
pub fn get_strange_LINEINITIALIZEEXPARAMS() -> (LINEINITIALIZEEXPARAMS, String) { (LINEINITIALIZEEXPARAMS::default(), "LINEINITIALIZEEXPARAMS::default()".to_string()) }
pub fn get_strange_LINEMESSAGE() -> (LINEMESSAGE, String) { (LINEMESSAGE::default(), "LINEMESSAGE::default()".to_string()) }
pub fn get_strange_LINEPROVIDERLIST() -> (LINEPROVIDERLIST, String) { (LINEPROVIDERLIST::default(), "LINEPROVIDERLIST::default()".to_string()) }
pub fn get_strange_LINEPROXYREQUEST() -> (LINEPROXYREQUEST, String) { (LINEPROXYREQUEST::default(), "LINEPROXYREQUEST::default()".to_string()) }
pub fn get_strange_LINEPROXYREQUESTLIST() -> (LINEPROXYREQUESTLIST, String) { (LINEPROXYREQUESTLIST::default(), "LINEPROXYREQUESTLIST::default()".to_string()) }
pub fn get_strange_LINEQUEUEINFO() -> (LINEQUEUEINFO, String) { (LINEQUEUEINFO::default(), "LINEQUEUEINFO::default()".to_string()) }
pub fn get_strange_LINEQUEUELIST() -> (LINEQUEUELIST, String) { (LINEQUEUELIST::default(), "LINEQUEUELIST::default()".to_string()) }
pub fn get_strange_LINETRANSLATECAPS() -> (LINETRANSLATECAPS, String) { (LINETRANSLATECAPS::default(), "LINETRANSLATECAPS::default()".to_string()) }
pub fn get_strange_LINETRANSLATEOUTPUT() -> (LINETRANSLATEOUTPUT, String) { (LINETRANSLATEOUTPUT::default(), "LINETRANSLATEOUTPUT::default()".to_string()) }
pub fn get_strange_LOADED_IMAGE() -> (LOADED_IMAGE, String) { (LOADED_IMAGE::default(), "LOADED_IMAGE::default()".to_string()) }
pub fn get_strange_LOAD_LIBRARY_FLAGS() -> (LOAD_LIBRARY_FLAGS, String) { (LOAD_LIBRARY_FLAGS::default(), "LOAD_LIBRARY_FLAGS::default()".to_string()) }
pub fn get_strange_LOGFONTA() -> (LOGFONTA, String) { (LOGFONTA::default(), "LOGFONTA::default()".to_string()) }
pub fn get_strange_LOGFONTW() -> (LOGFONTW, String) { (LOGFONTW::default(), "LOGFONTW::default()".to_string()) }
pub fn get_strange_LOGON32_LOGON() -> (LOGON32_LOGON, String) { (LOGON32_LOGON::default(), "LOGON32_LOGON::default()".to_string()) }
pub fn get_strange_LOGON32_PROVIDER() -> (LOGON32_PROVIDER, String) { (LOGON32_PROVIDER::default(), "LOGON32_PROVIDER::default()".to_string()) }
pub fn get_strange_LPARAM() -> (LPARAM, String) {    (LPARAM::default(), "LPARAM::default()".to_string())}
pub fn get_strange_LPCALL_BACK_USER_INTERRUPT_ROUTINE() -> (LPCALL_BACK_USER_INTERRUPT_ROUTINE, String) { (LPCALL_BACK_USER_INTERRUPT_ROUTINE::default(), "LPCALL_BACK_USER_INTERRUPT_ROUTINE::default()".to_string()) }
pub fn get_strange_LPDDENUMCALLBACKA() -> (LPDDENUMCALLBACKA, String) { (LPDDENUMCALLBACKA::default(), "LPDDENUMCALLBACKA::default()".to_string()) }
pub fn get_strange_LPDDENUMCALLBACKEXA() -> (LPDDENUMCALLBACKEXA, String) { (LPDDENUMCALLBACKEXA::default(), "LPDDENUMCALLBACKEXA::default()".to_string()) }
pub fn get_strange_LPDDENUMCALLBACKEXW() -> (LPDDENUMCALLBACKEXW, String) { (LPDDENUMCALLBACKEXW::default(), "LPDDENUMCALLBACKEXW::default()".to_string()) }
pub fn get_strange_LPDDENUMCALLBACKW() -> (LPDDENUMCALLBACKW, String) { (LPDDENUMCALLBACKW::default(), "LPDDENUMCALLBACKW::default()".to_string()) }
pub fn get_strange_LPDSENUMCALLBACKA() -> (LPDSENUMCALLBACKA, String) { (LPDSENUMCALLBACKA::default(), "LPDSENUMCALLBACKA::default()".to_string()) }
pub fn get_strange_LPDSENUMCALLBACKW() -> (LPDSENUMCALLBACKW, String) { (LPDSENUMCALLBACKW::default(), "LPDSENUMCALLBACKW::default()".to_string()) }
pub fn get_strange_LPFIBER_START_ROUTINE() -> (LPFIBER_START_ROUTINE, String) { (LPFIBER_START_ROUTINE::default(), "LPFIBER_START_ROUTINE::default()".to_string()) }
pub fn get_strange_LPFNSVADDPROPSHEETPAGE() -> (LPFNSVADDPROPSHEETPAGE, String) { (LPFNSVADDPROPSHEETPAGE::default(), "LPFNSVADDPROPSHEETPAGE::default()".to_string()) }
pub fn get_strange_LPHANDLER_FUNCTION() -> (LPHANDLER_FUNCTION, String) { (LPHANDLER_FUNCTION::default(), "LPHANDLER_FUNCTION::default()".to_string()) }
pub fn get_strange_LPHANDLER_FUNCTION_EX() -> (LPHANDLER_FUNCTION_EX, String) { (LPHANDLER_FUNCTION_EX::default(), "LPHANDLER_FUNCTION_EX::default()".to_string()) }
pub fn get_strange_LPINTERNET_STATUS_CALLBACK() -> (LPINTERNET_STATUS_CALLBACK, String) { (LPINTERNET_STATUS_CALLBACK::default(), "LPINTERNET_STATUS_CALLBACK::default()".to_string()) }
pub fn get_strange_LPMMIOPROC() -> (LPMMIOPROC, String) { (LPMMIOPROC::default(), "LPMMIOPROC::default()".to_string()) }
pub fn get_strange_LPOVERLAPPED_COMPLETION_ROUTINE() -> (LPOVERLAPPED_COMPLETION_ROUTINE, String) { (LPOVERLAPPED_COMPLETION_ROUTINE::default(), "LPOVERLAPPED_COMPLETION_ROUTINE::default()".to_string()) }
pub fn get_strange_LPPROC_THREAD_ATTRIBUTE_LIST() -> (LPPROC_THREAD_ATTRIBUTE_LIST, String) { (LPPROC_THREAD_ATTRIBUTE_LIST::default(), "LPPROC_THREAD_ATTRIBUTE_LIST::default()".to_string()) }
pub fn get_strange_LPTASKCALLBACK() -> (LPTASKCALLBACK, String) { (LPTASKCALLBACK::default(), "LPTASKCALLBACK::default()".to_string()) }
pub fn get_strange_LPTHREAD_START_ROUTINE() -> (LPTHREAD_START_ROUTINE, String) { (LPTHREAD_START_ROUTINE::default(), "LPTHREAD_START_ROUTINE::default()".to_string()) }
pub fn get_strange_LPTIMECALLBACK() -> (LPTIMECALLBACK, String) { (LPTIMECALLBACK::default(), "LPTIMECALLBACK::default()".to_string()) }
pub fn get_strange_LPTOP_LEVEL_EXCEPTION_FILTER() -> (LPTOP_LEVEL_EXCEPTION_FILTER, String) { (LPTOP_LEVEL_EXCEPTION_FILTER::default(), "LPTOP_LEVEL_EXCEPTION_FILTER::default()".to_string()) }
pub fn get_strange_LRESULT() -> (LRESULT, String) { (LRESULT::default(), "LRESULT::default()".to_string()) }
pub fn get_strange_LUID() -> (LUID, String) { (LUID::default(), "LUID::default()".to_string()) }
pub fn get_strange_LicenseProtectionStatus() -> (LicenseProtectionStatus, String) { (LicenseProtectionStatus::default(), "LicenseProtectionStatus::default()".to_string()) }
pub fn get_strange_MACHINE_ATTRIBUTES() -> (MACHINE_ATTRIBUTES, String) { (MACHINE_ATTRIBUTES::default(), "MACHINE_ATTRIBUTES::default()".to_string()) }
pub fn get_strange_MACHINE_PROCESSOR_POWER_POLICY() -> (MACHINE_PROCESSOR_POWER_POLICY, String) { (MACHINE_PROCESSOR_POWER_POLICY::default(), "MACHINE_PROCESSOR_POWER_POLICY::default()".to_string()) }
pub fn get_strange_MARGINS() -> (MARGINS, String) { (MARGINS::default(), "MARGINS::default()".to_string()) }
pub fn get_strange_MCAST_CLIENT_UID() -> (MCAST_CLIENT_UID, String) { (MCAST_CLIENT_UID::default(), "MCAST_CLIENT_UID::default()".to_string()) }
pub fn get_strange_MCAST_LEASE_REQUEST() -> (MCAST_LEASE_REQUEST, String) { (MCAST_LEASE_REQUEST::default(), "MCAST_LEASE_REQUEST::default()".to_string()) }
pub fn get_strange_MCAST_LEASE_RESPONSE() -> (MCAST_LEASE_RESPONSE, String) { (MCAST_LEASE_RESPONSE::default(), "MCAST_LEASE_RESPONSE::default()".to_string()) }
pub fn get_strange_MCAST_SCOPE_CTX() -> (MCAST_SCOPE_CTX, String) { (MCAST_SCOPE_CTX::default(), "MCAST_SCOPE_CTX::default()".to_string()) }
pub fn get_strange_MCAST_SCOPE_ENTRY() -> (MCAST_SCOPE_ENTRY, String) { (MCAST_SCOPE_ENTRY::default(), "MCAST_SCOPE_ENTRY::default()".to_string()) }
pub fn get_strange_MC_COLOR_TEMPERATURE() -> (MC_COLOR_TEMPERATURE, String) { (MC_COLOR_TEMPERATURE::default(), "MC_COLOR_TEMPERATURE::default()".to_string()) }
pub fn get_strange_MC_DISPLAY_TECHNOLOGY_TYPE() -> (MC_DISPLAY_TECHNOLOGY_TYPE, String) { (MC_DISPLAY_TECHNOLOGY_TYPE::default(), "MC_DISPLAY_TECHNOLOGY_TYPE::default()".to_string()) }
pub fn get_strange_MC_DRIVE_TYPE() -> (MC_DRIVE_TYPE, String) { (MC_DRIVE_TYPE::default(), "MC_DRIVE_TYPE::default()".to_string()) }
pub fn get_strange_MC_GAIN_TYPE() -> (MC_GAIN_TYPE, String) { (MC_GAIN_TYPE::default(), "MC_GAIN_TYPE::default()".to_string()) }
pub fn get_strange_MC_POSITION_TYPE() -> (MC_POSITION_TYPE, String) { (MC_POSITION_TYPE::default(), "MC_POSITION_TYPE::default()".to_string()) }
pub fn get_strange_MC_SIZE_TYPE() -> (MC_SIZE_TYPE, String) { (MC_SIZE_TYPE::default(), "MC_SIZE_TYPE::default()".to_string()) }
pub fn get_strange_MC_TIMING_REPORT() -> (MC_TIMING_REPORT, String) { (MC_TIMING_REPORT::default(), "MC_TIMING_REPORT::default()".to_string()) }
pub fn get_strange_MC_VCP_CODE_TYPE() -> (MC_VCP_CODE_TYPE, String) { (MC_VCP_CODE_TYPE::default(), "MC_VCP_CODE_TYPE::default()".to_string()) }
pub fn get_strange_MDNS_QUERY_HANDLE() -> (MDNS_QUERY_HANDLE, String) { (MDNS_QUERY_HANDLE::default(), "MDNS_QUERY_HANDLE::default()".to_string()) }
pub fn get_strange_MENU_ITEM_FLAGS() -> (MENU_ITEM_FLAGS, String) {    (MENU_ITEM_FLAGS::default(), "MENU_ITEM_FLAGS::default()".to_string())}
pub fn get_strange_MERGE_VIRTUAL_DISK_FLAG() -> (MERGE_VIRTUAL_DISK_FLAG, String) { (MERGE_VIRTUAL_DISK_FLAG::default(), "MERGE_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_MESSAGEBOX_RESULT() -> (MESSAGEBOX_RESULT, String) { (MESSAGEBOX_RESULT::default(), "MESSAGEBOX_RESULT::default()".to_string()) }
pub fn get_strange_MESSAGEBOX_STYLE() -> (MESSAGEBOX_STYLE, String) { (MESSAGEBOX_STYLE::default(), "MESSAGEBOX_STYLE::default()".to_string()) }
pub fn get_strange_MFASYNC_WORKQUEUE_TYPE() -> (MFASYNC_WORKQUEUE_TYPE, String) { (MFASYNC_WORKQUEUE_TYPE::default(), "MFASYNC_WORKQUEUE_TYPE::default()".to_string()) }
pub fn get_strange_MFCameraIntrinsic_DistortionModelType() -> (MFCameraIntrinsic_DistortionModelType, String) { (MFCameraIntrinsic_DistortionModelType::default(), "MFCameraIntrinsic_DistortionModelType::default()".to_string()) }
pub fn get_strange_MFENUMPROC() -> (MFENUMPROC, String) { (MFENUMPROC::default(), "MFENUMPROC::default()".to_string()) }
pub fn get_strange_MIB_ANYCASTIPADDRESS_ROW() -> (MIB_ANYCASTIPADDRESS_ROW, String) { (MIB_ANYCASTIPADDRESS_ROW::default(), "MIB_ANYCASTIPADDRESS_ROW::default()".to_string()) }
pub fn get_strange_MIB_ICMP() -> (MIB_ICMP, String) { (MIB_ICMP::default(), "MIB_ICMP::default()".to_string()) }
pub fn get_strange_MIB_ICMP_EX_XPSP1() -> (MIB_ICMP_EX_XPSP1, String) { (MIB_ICMP_EX_XPSP1::default(), "MIB_ICMP_EX_XPSP1::default()".to_string()) }
pub fn get_strange_MIB_IFROW() -> (MIB_IFROW, String) { (MIB_IFROW::default(), "MIB_IFROW::default()".to_string()) }
pub fn get_strange_MIB_IFTABLE() -> (MIB_IFTABLE, String) { (MIB_IFTABLE::default(), "MIB_IFTABLE::default()".to_string()) }
pub fn get_strange_MIB_IF_ENTRY_LEVEL() -> (MIB_IF_ENTRY_LEVEL, String) { (MIB_IF_ENTRY_LEVEL::default(), "MIB_IF_ENTRY_LEVEL::default()".to_string()) }
pub fn get_strange_MIB_IF_ROW2() -> (MIB_IF_ROW2, String) { (MIB_IF_ROW2::default(), "MIB_IF_ROW2::default()".to_string()) }
pub fn get_strange_MIB_IF_TABLE_LEVEL() -> (MIB_IF_TABLE_LEVEL, String) { (MIB_IF_TABLE_LEVEL::default(), "MIB_IF_TABLE_LEVEL::default()".to_string()) }
pub fn get_strange_MIB_IPADDRTABLE() -> (MIB_IPADDRTABLE, String) { (MIB_IPADDRTABLE::default(), "MIB_IPADDRTABLE::default()".to_string()) }
pub fn get_strange_MIB_IPFORWARDROW() -> (MIB_IPFORWARDROW, String) { (MIB_IPFORWARDROW::default(), "MIB_IPFORWARDROW::default()".to_string()) }
pub fn get_strange_MIB_IPFORWARDTABLE() -> (MIB_IPFORWARDTABLE, String) { (MIB_IPFORWARDTABLE::default(), "MIB_IPFORWARDTABLE::default()".to_string()) }
pub fn get_strange_MIB_IPFORWARD_ROW2() -> (MIB_IPFORWARD_ROW2, String) { (MIB_IPFORWARD_ROW2::default(), "MIB_IPFORWARD_ROW2::default()".to_string()) }
pub fn get_strange_MIB_IPINTERFACE_ROW() -> (MIB_IPINTERFACE_ROW, String) { (MIB_IPINTERFACE_ROW::default(), "MIB_IPINTERFACE_ROW::default()".to_string()) }
pub fn get_strange_MIB_IPNETTABLE() -> (MIB_IPNETTABLE, String) { (MIB_IPNETTABLE::default(), "MIB_IPNETTABLE::default()".to_string()) }
pub fn get_strange_MIB_IPNET_ROW2() -> (MIB_IPNET_ROW2, String) { (MIB_IPNET_ROW2::default(), "MIB_IPNET_ROW2::default()".to_string()) }
pub fn get_strange_MIB_IPPATH_ROW() -> (MIB_IPPATH_ROW, String) { (MIB_IPPATH_ROW::default(), "MIB_IPPATH_ROW::default()".to_string()) }
pub fn get_strange_MIB_IPSTATS_LH() -> (MIB_IPSTATS_LH, String) { (MIB_IPSTATS_LH::default(), "MIB_IPSTATS_LH::default()".to_string()) }
pub fn get_strange_MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES() -> (MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES, String) { (MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES::default(), "MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES::default()".to_string()) }
pub fn get_strange_MIB_MULTICASTIPADDRESS_ROW() -> (MIB_MULTICASTIPADDRESS_ROW, String) { (MIB_MULTICASTIPADDRESS_ROW::default(), "MIB_MULTICASTIPADDRESS_ROW::default()".to_string()) }
pub fn get_strange_MIB_TCP6TABLE() -> (MIB_TCP6TABLE, String) { (MIB_TCP6TABLE::default(), "MIB_TCP6TABLE::default()".to_string()) }
pub fn get_strange_MIB_TCP6TABLE2() -> (MIB_TCP6TABLE2, String) { (MIB_TCP6TABLE2::default(), "MIB_TCP6TABLE2::default()".to_string()) }
pub fn get_strange_MIB_TCPSTATS2() -> (MIB_TCPSTATS2, String) { (MIB_TCPSTATS2::default(), "MIB_TCPSTATS2::default()".to_string()) }
pub fn get_strange_MIB_TCPSTATS_LH() -> (MIB_TCPSTATS_LH, String) { (MIB_TCPSTATS_LH::default(), "MIB_TCPSTATS_LH::default()".to_string()) }
pub fn get_strange_MIB_TCPTABLE() -> (MIB_TCPTABLE, String) { (MIB_TCPTABLE::default(), "MIB_TCPTABLE::default()".to_string()) }
pub fn get_strange_MIB_TCPTABLE2() -> (MIB_TCPTABLE2, String) { (MIB_TCPTABLE2::default(), "MIB_TCPTABLE2::default()".to_string()) }
pub fn get_strange_MIB_UDP6TABLE() -> (MIB_UDP6TABLE, String) { (MIB_UDP6TABLE::default(), "MIB_UDP6TABLE::default()".to_string()) }
pub fn get_strange_MIB_UDPSTATS() -> (MIB_UDPSTATS, String) { (MIB_UDPSTATS::default(), "MIB_UDPSTATS::default()".to_string()) }
pub fn get_strange_MIB_UDPSTATS2() -> (MIB_UDPSTATS2, String) { (MIB_UDPSTATS2::default(), "MIB_UDPSTATS2::default()".to_string()) }
pub fn get_strange_MIB_UDPTABLE() -> (MIB_UDPTABLE, String) { (MIB_UDPTABLE::default(), "MIB_UDPTABLE::default()".to_string()) }
pub fn get_strange_MIB_UNICASTIPADDRESS_ROW() -> (MIB_UNICASTIPADDRESS_ROW, String) { (MIB_UNICASTIPADDRESS_ROW::default(), "MIB_UNICASTIPADDRESS_ROW::default()".to_string()) }
pub fn get_strange_MIDIHDR() -> (MIDIHDR, String) { (MIDIHDR::default(), "MIDIHDR::default()".to_string()) }
pub fn get_strange_MIDIINCAPSA() -> (MIDIINCAPSA, String) { (MIDIINCAPSA::default(), "MIDIINCAPSA::default()".to_string()) }
pub fn get_strange_MIDIINCAPSW() -> (MIDIINCAPSW, String) { (MIDIINCAPSW::default(), "MIDIINCAPSW::default()".to_string()) }
pub fn get_strange_MIDIOUTCAPSA() -> (MIDIOUTCAPSA, String) { (MIDIOUTCAPSA::default(), "MIDIOUTCAPSA::default()".to_string()) }
pub fn get_strange_MIDIOUTCAPSW() -> (MIDIOUTCAPSW, String) { (MIDIOUTCAPSW::default(), "MIDIOUTCAPSW::default()".to_string()) }
pub fn get_strange_MIDI_WAVE_OPEN_TYPE() -> (MIDI_WAVE_OPEN_TYPE, String) { (MIDI_WAVE_OPEN_TYPE::default(), "MIDI_WAVE_OPEN_TYPE::default()".to_string()) }
pub fn get_strange_MINIDUMP_TYPE() -> (MINIDUMP_TYPE, String) { (MINIDUMP_TYPE::default(), "MINIDUMP_TYPE::default()".to_string()) }
pub fn get_strange_MIRROR_VIRTUAL_DISK_FLAG() -> (MIRROR_VIRTUAL_DISK_FLAG, String) { (MIRROR_VIRTUAL_DISK_FLAG::default(), "MIRROR_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_MIXERCAPSA() -> (MIXERCAPSA, String) { (MIXERCAPSA::default(), "MIXERCAPSA::default()".to_string()) }
pub fn get_strange_MIXERCAPSW() -> (MIXERCAPSW, String) { (MIXERCAPSW::default(), "MIXERCAPSW::default()".to_string()) }
pub fn get_strange_MIXERCONTROLDETAILS() -> (MIXERCONTROLDETAILS, String) { (MIXERCONTROLDETAILS::default(), "MIXERCONTROLDETAILS::default()".to_string()) }
pub fn get_strange_MIXERLINEA() -> (MIXERLINEA, String) { (MIXERLINEA::default(), "MIXERLINEA::default()".to_string()) }
pub fn get_strange_MIXERLINECONTROLSA() -> (MIXERLINECONTROLSA, String) { (MIXERLINECONTROLSA::default(), "MIXERLINECONTROLSA::default()".to_string()) }
pub fn get_strange_MIXERLINECONTROLSW() -> (MIXERLINECONTROLSW, String) { (MIXERLINECONTROLSW::default(), "MIXERLINECONTROLSW::default()".to_string()) }
pub fn get_strange_MIXERLINEW() -> (MIXERLINEW, String) { (MIXERLINEW::default(), "MIXERLINEW::default()".to_string()) }
pub fn get_strange_MI_Application() -> (MI_Application, String) { (MI_Application::default(), "MI_Application::default()".to_string()) }
pub fn get_strange_MMCKINFO() -> (MMCKINFO, String) { (MMCKINFO::default(), "MMCKINFO::default()".to_string()) }
pub fn get_strange_MMIOINFO() -> (MMIOINFO, String) { (MMIOINFO::default(), "MMIOINFO::default()".to_string()) }
pub fn get_strange_MMTIME() -> (MMTIME, String) { (MMTIME::default(), "MMTIME::default()".to_string()) }
pub fn get_strange_MODIFY_VHDSET_FLAG() -> (MODIFY_VHDSET_FLAG, String) { (MODIFY_VHDSET_FLAG::default(), "MODIFY_VHDSET_FLAG::default()".to_string()) }
pub fn get_strange_MODIFY_WORLD_TRANSFORM_MODE() -> (MODIFY_WORLD_TRANSFORM_MODE, String) { (MODIFY_WORLD_TRANSFORM_MODE::default(), "MODIFY_WORLD_TRANSFORM_MODE::default()".to_string()) }
pub fn get_strange_MODULEENTRY32() -> (MODULEENTRY32, String) { (MODULEENTRY32::default(), "MODULEENTRY32::default()".to_string()) }
pub fn get_strange_MODULEENTRY32W() -> (MODULEENTRY32W, String) { (MODULEENTRY32W::default(), "MODULEENTRY32W::default()".to_string()) }
pub fn get_strange_MODULEINFO() -> (MODULEINFO, String) { (MODULEINFO::default(), "MODULEINFO::default()".to_string()) }
pub fn get_strange_MONITORENUMPROC() -> (MONITORENUMPROC, String) { (MONITORENUMPROC::default(), "MONITORENUMPROC::default()".to_string()) }
pub fn get_strange_MONITORINFO() -> (MONITORINFO, String) { (MONITORINFO::default(), "MONITORINFO::default()".to_string()) }
pub fn get_strange_MONITOR_DPI_TYPE() -> (MONITOR_DPI_TYPE, String) { (MONITOR_DPI_TYPE::default(), "MONITOR_DPI_TYPE::default()".to_string()) }
pub fn get_strange_MONITOR_FROM_FLAGS() -> (MONITOR_FROM_FLAGS, String) { (MONITOR_FROM_FLAGS::default(), "MONITOR_FROM_FLAGS::default()".to_string()) }
pub fn get_strange_MOUSEMOVEPOINT() -> (MOUSEMOVEPOINT, String) { (MOUSEMOVEPOINT::default(), "MOUSEMOVEPOINT::default()".to_string()) }
pub fn get_strange_MOUSE_EVENT_FLAGS() -> (MOUSE_EVENT_FLAGS, String) { (MOUSE_EVENT_FLAGS::default(), "MOUSE_EVENT_FLAGS::default()".to_string()) }
pub fn get_strange_MOUSE_WHEEL_PARAMETER() -> (MOUSE_WHEEL_PARAMETER, String) { (MOUSE_WHEEL_PARAMETER::default(), "MOUSE_WHEEL_PARAMETER::default()".to_string()) }
pub fn get_strange_MSG() -> (MSG, String) { (MSG::default(), "MSG::default()".to_string()) }
pub fn get_strange_MULTI_QI() -> (MULTI_QI, String) { (MULTI_QI::default(), "MULTI_QI::default()".to_string()) }
pub fn get_strange_MilMatrix3x2D() -> (MilMatrix3x2D, String) { (MilMatrix3x2D::default(), "MilMatrix3x2D::default()".to_string()) }
pub fn get_strange_MrmPlatformVersion() -> (MrmPlatformVersion, String) {    (MrmPlatformVersion::default(), "MrmPlatformVersion::default()".to_string())}
pub fn get_strange_NAMED_PIPE_MODE() -> (NAMED_PIPE_MODE, String) { (NAMED_PIPE_MODE::default(), "NAMED_PIPE_MODE::default()".to_string()) }
pub fn get_strange_NCB() -> (NCB, String) { (NCB::default(), "NCB::default()".to_string()) }
pub fn get_strange_NETSETUP_JOIN_STATUS() -> (NETSETUP_JOIN_STATUS, String) { (NETSETUP_JOIN_STATUS::default(), "NETSETUP_JOIN_STATUS::default()".to_string()) }
pub fn get_strange_NETSETUP_NAME_TYPE() -> (NETSETUP_NAME_TYPE, String) { (NETSETUP_NAME_TYPE::default(), "NETSETUP_NAME_TYPE::default()".to_string()) }
pub fn get_strange_NETSETUP_PROVISION() -> (NETSETUP_PROVISION, String) { (NETSETUP_PROVISION::default(), "NETSETUP_PROVISION::default()".to_string()) }
pub fn get_strange_NET_COMPUTER_NAME_TYPE() -> (NET_COMPUTER_NAME_TYPE, String) { (NET_COMPUTER_NAME_TYPE::default(), "NET_COMPUTER_NAME_TYPE::default()".to_string()) }
pub fn get_strange_NET_JOIN_DOMAIN_JOIN_OPTIONS() -> (NET_JOIN_DOMAIN_JOIN_OPTIONS, String) { (NET_JOIN_DOMAIN_JOIN_OPTIONS::default(), "NET_JOIN_DOMAIN_JOIN_OPTIONS::default()".to_string()) }
pub fn get_strange_NET_LUID_LH() -> (NET_LUID_LH, String) { (NET_LUID_LH::default(), "NET_LUID_LH::default()".to_string()) }
pub fn get_strange_NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS() -> (NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, String) { (NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS::default(), "NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS::default()".to_string()) }
pub fn get_strange_NET_REQUEST_PROVISION_OPTIONS() -> (NET_REQUEST_PROVISION_OPTIONS, String) { (NET_REQUEST_PROVISION_OPTIONS::default(), "NET_REQUEST_PROVISION_OPTIONS::default()".to_string()) }
pub fn get_strange_NET_SERVER_TYPE() -> (NET_SERVER_TYPE, String) { (NET_SERVER_TYPE::default(), "NET_SERVER_TYPE::default()".to_string()) }
pub fn get_strange_NET_USER_ENUM_FILTER_FLAGS() -> (NET_USER_ENUM_FILTER_FLAGS, String) { (NET_USER_ENUM_FILTER_FLAGS::default(), "NET_USER_ENUM_FILTER_FLAGS::default()".to_string()) }
pub fn get_strange_NET_VALIDATE_PASSWORD_TYPE() -> (NET_VALIDATE_PASSWORD_TYPE, String) { (NET_VALIDATE_PASSWORD_TYPE::default(), "NET_VALIDATE_PASSWORD_TYPE::default()".to_string()) }
pub fn get_strange_NL_NETWORK_CONNECTIVITY_HINT() -> (NL_NETWORK_CONNECTIVITY_HINT, String) { (NL_NETWORK_CONNECTIVITY_HINT::default(), "NL_NETWORK_CONNECTIVITY_HINT::default()".to_string()) }
pub fn get_strange_NOTIFY_ICON_MESSAGE() -> (NOTIFY_ICON_MESSAGE, String) { (NOTIFY_ICON_MESSAGE::default(), "NOTIFY_ICON_MESSAGE::default()".to_string()) }
pub fn get_strange_NOTIFY_IME_ACTION() -> (NOTIFY_IME_ACTION, String) { (NOTIFY_IME_ACTION::default(), "NOTIFY_IME_ACTION::default()".to_string()) }
pub fn get_strange_NOTIFY_IME_INDEX() -> (NOTIFY_IME_INDEX, String) { (NOTIFY_IME_INDEX::default(), "NOTIFY_IME_INDEX::default()".to_string()) }
pub fn get_strange_NTSTATUS() -> (NTSTATUS, String) { (NTSTATUS::default(), "NTSTATUS::default()".to_string()) }
pub fn get_strange_NUMPARSE() -> (NUMPARSE, String) { (NUMPARSE::default(), "NUMPARSE::default()".to_string()) }
pub fn get_strange_NamespaceHandle() -> (NamespaceHandle, String) { (NamespaceHandle::default(), "NamespaceHandle::default()".to_string()) }
pub fn get_strange_NavigateDirection() -> (NavigateDirection, String) { (NavigateDirection::default(), "NavigateDirection::default()".to_string()) }
pub fn get_strange_NormalizeState() -> (NormalizeState, String) { (NormalizeState::default(), "NormalizeState::default()".to_string()) }
pub fn get_strange_NotificationKind() -> (NotificationKind, String) { (NotificationKind::default(), "NotificationKind::default()".to_string()) }
pub fn get_strange_NotificationProcessing() -> (NotificationProcessing, String) { (NotificationProcessing::default(), "NotificationProcessing::default()".to_string()) }
pub fn get_strange_OBJECT_SECURITY_INFORMATION() -> (OBJECT_SECURITY_INFORMATION, String) { (OBJECT_SECURITY_INFORMATION::default(), "OBJECT_SECURITY_INFORMATION::default()".to_string()) }
pub fn get_strange_OBJECT_TYPE_LIST() -> (OBJECT_TYPE_LIST, String) { (OBJECT_TYPE_LIST::default(), "OBJECT_TYPE_LIST::default()".to_string()) }
pub fn get_strange_OBJ_TYPE() -> (OBJ_TYPE, String) { (OBJ_TYPE::default(), "OBJ_TYPE::default()".to_string()) }
pub fn get_strange_OCPFIPARAMS() -> (OCPFIPARAMS, String) { (OCPFIPARAMS::default(), "OCPFIPARAMS::default()".to_string()) }
pub fn get_strange_OEM_SOURCE_MEDIA_TYPE() -> (OEM_SOURCE_MEDIA_TYPE, String) { (OEM_SOURCE_MEDIA_TYPE::default(), "OEM_SOURCE_MEDIA_TYPE::default()".to_string()) }
pub fn get_strange_OIFI() -> (OIFI, String) { (OIFI::default(), "OIFI::default()".to_string()) }
pub fn get_strange_OLESTREAM() -> (OLESTREAM, String) { (OLESTREAM::default(), "OLESTREAM::default()".to_string()) }
pub fn get_strange_OOBE_COMPLETED_CALLBACK() -> (OOBE_COMPLETED_CALLBACK, String) { (OOBE_COMPLETED_CALLBACK::default(), "OOBE_COMPLETED_CALLBACK::default()".to_string()) }
pub fn get_strange_OPENCARDNAMEA() -> (OPENCARDNAMEA, String) { (OPENCARDNAMEA::default(), "OPENCARDNAMEA::default()".to_string()) }
pub fn get_strange_OPENCARDNAMEW() -> (OPENCARDNAMEW, String) { (OPENCARDNAMEW::default(), "OPENCARDNAMEW::default()".to_string()) }
pub fn get_strange_OPENCARDNAME_EXA() -> (OPENCARDNAME_EXA, String) { (OPENCARDNAME_EXA::default(), "OPENCARDNAME_EXA::default()".to_string()) }
pub fn get_strange_OPENCARDNAME_EXW() -> (OPENCARDNAME_EXW, String) { (OPENCARDNAME_EXW::default(), "OPENCARDNAME_EXW::default()".to_string()) }
pub fn get_strange_OPENFILENAMEA() -> (OPENFILENAMEA, String) { (OPENFILENAMEA::default(), "OPENFILENAMEA::default()".to_string()) }
pub fn get_strange_OPENFILENAMEW() -> (OPENFILENAMEW, String) { (OPENFILENAMEW::default(), "OPENFILENAMEW::default()".to_string()) }
pub fn get_strange_OPEN_THEME_DATA_FLAGS() -> (OPEN_THEME_DATA_FLAGS, String) { (OPEN_THEME_DATA_FLAGS::default(), "OPEN_THEME_DATA_FLAGS::default()".to_string()) }
pub fn get_strange_OPEN_VIRTUAL_DISK_FLAG() -> (OPEN_VIRTUAL_DISK_FLAG, String) { (OPEN_VIRTUAL_DISK_FLAG::default(), "OPEN_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_OPM_VIDEO_OUTPUT_SEMANTICS() -> (OPM_VIDEO_OUTPUT_SEMANTICS, String) { (OPM_VIDEO_OUTPUT_SEMANTICS::default(), "OPM_VIDEO_OUTPUT_SEMANTICS::default()".to_string()) }
pub fn get_strange_ORIENTATION_PREFERENCE() -> (ORIENTATION_PREFERENCE, String) { (ORIENTATION_PREFERENCE::default(), "ORIENTATION_PREFERENCE::default()".to_string()) }
pub fn get_strange_OS() -> (OS, String) { (OS::default(), "OS::default()".to_string()) }
pub fn get_strange_OUTLINETEXTMETRICA() -> (OUTLINETEXTMETRICA, String) { (OUTLINETEXTMETRICA::default(), "OUTLINETEXTMETRICA::default()".to_string()) }
pub fn get_strange_OUTLINETEXTMETRICW() -> (OUTLINETEXTMETRICW, String) { (OUTLINETEXTMETRICW::default(), "OUTLINETEXTMETRICW::default()".to_string()) }
pub fn get_strange_OVERLAPPED() -> (OVERLAPPED, String) { (OVERLAPPED::default(), "OVERLAPPED::default()".to_string()) }
pub fn get_strange_OVERLAPPED_ENTRY() -> (OVERLAPPED_ENTRY, String) { (OVERLAPPED_ENTRY::default(), "OVERLAPPED_ENTRY::default()".to_string()) }
pub fn get_strange_OleMenuGroupWidths() -> (OleMenuGroupWidths, String) { (OleMenuGroupWidths::default(), "OleMenuGroupWidths::default()".to_string()) }
pub fn get_strange_PACKAGE_VERSION() -> (PACKAGE_VERSION, String) { (PACKAGE_VERSION::default(), "PACKAGE_VERSION::default()".to_string()) }
pub fn get_strange_PAGESETUPDLGA() -> (PAGESETUPDLGA, String) { (PAGESETUPDLGA::default(), "PAGESETUPDLGA::default()".to_string()) }
pub fn get_strange_PAGESETUPDLGW() -> (PAGESETUPDLGW, String) { (PAGESETUPDLGW::default(), "PAGESETUPDLGW::default()".to_string()) }
pub fn get_strange_PAINTSTRUCT() -> (PAINTSTRUCT, String) { (PAINTSTRUCT::default(), "PAINTSTRUCT::default()".to_string()) }
pub fn get_strange_PALETTEENTRY() -> (PALETTEENTRY, String) { (PALETTEENTRY::default(), "PALETTEENTRY::default()".to_string()) }
pub fn get_strange_PAPCFUNC() -> (PAPCFUNC, String) { (PAPCFUNC::default(), "PAPCFUNC::default()".to_string()) }
pub fn get_strange_PARSEACTION() -> (PARSEACTION, String) { (PARSEACTION::default(), "PARSEACTION::default()".to_string()) }
pub fn get_strange_PARSEDURLA() -> (PARSEDURLA, String) { (PARSEDURLA::default(), "PARSEDURLA::default()".to_string()) }
pub fn get_strange_PARSEDURLW() -> (PARSEDURLW, String) { (PARSEDURLW::default(), "PARSEDURLW::default()".to_string()) }
pub fn get_strange_PATHDATA() -> (PATHDATA, String) { (PATHDATA::default(), "PATHDATA::default()".to_string()) }
pub fn get_strange_PATHOBJ() -> (PATHOBJ, String) { (PATHOBJ::default(), "PATHOBJ::default()".to_string()) }
pub fn get_strange_PCM_NOTIFY_CALLBACK() -> (PCM_NOTIFY_CALLBACK, String) { (PCM_NOTIFY_CALLBACK::default(), "PCM_NOTIFY_CALLBACK::default()".to_string()) }
pub fn get_strange_PCSTR() -> (PCSTR, String) {    (PCSTR::default(), "PCSTR::default()".to_string())}
pub fn get_strange_PCWSTR() -> (PCWSTR, String) {    (PCWSTR::default(), "PCWSTR::default()".to_string())}
pub fn get_strange_PDEV_QUERY_RESULT_CALLBACK() -> (PDEV_QUERY_RESULT_CALLBACK, String) { (PDEV_QUERY_RESULT_CALLBACK::default(), "PDEV_QUERY_RESULT_CALLBACK::default()".to_string()) }
pub fn get_strange_PDH_COUNTER_INFO_A() -> (PDH_COUNTER_INFO_A, String) { (PDH_COUNTER_INFO_A::default(), "PDH_COUNTER_INFO_A::default()".to_string()) }
pub fn get_strange_PDH_COUNTER_INFO_W() -> (PDH_COUNTER_INFO_W, String) { (PDH_COUNTER_INFO_W::default(), "PDH_COUNTER_INFO_W::default()".to_string()) }
pub fn get_strange_PDH_COUNTER_PATH_ELEMENTS_A() -> (PDH_COUNTER_PATH_ELEMENTS_A, String) { (PDH_COUNTER_PATH_ELEMENTS_A::default(), "PDH_COUNTER_PATH_ELEMENTS_A::default()".to_string()) }
pub fn get_strange_PDH_COUNTER_PATH_ELEMENTS_W() -> (PDH_COUNTER_PATH_ELEMENTS_W, String) { (PDH_COUNTER_PATH_ELEMENTS_W::default(), "PDH_COUNTER_PATH_ELEMENTS_W::default()".to_string()) }
pub fn get_strange_PDH_DLL_VERSION() -> (PDH_DLL_VERSION, String) { (PDH_DLL_VERSION::default(), "PDH_DLL_VERSION::default()".to_string()) }
pub fn get_strange_PDH_FMT() -> (PDH_FMT, String) { (PDH_FMT::default(), "PDH_FMT::default()".to_string()) }
pub fn get_strange_PDH_FMT_COUNTERVALUE() -> (PDH_FMT_COUNTERVALUE, String) { (PDH_FMT_COUNTERVALUE::default(), "PDH_FMT_COUNTERVALUE::default()".to_string()) }
pub fn get_strange_PDH_FMT_COUNTERVALUE_ITEM_A() -> (PDH_FMT_COUNTERVALUE_ITEM_A, String) { (PDH_FMT_COUNTERVALUE_ITEM_A::default(), "PDH_FMT_COUNTERVALUE_ITEM_A::default()".to_string()) }
pub fn get_strange_PDH_FMT_COUNTERVALUE_ITEM_W() -> (PDH_FMT_COUNTERVALUE_ITEM_W, String) { (PDH_FMT_COUNTERVALUE_ITEM_W::default(), "PDH_FMT_COUNTERVALUE_ITEM_W::default()".to_string()) }
pub fn get_strange_PDH_LOG() -> (PDH_LOG, String) { (PDH_LOG::default(), "PDH_LOG::default()".to_string()) }
pub fn get_strange_PDH_LOG_TYPE() -> (PDH_LOG_TYPE, String) { (PDH_LOG_TYPE::default(), "PDH_LOG_TYPE::default()".to_string()) }
pub fn get_strange_PDH_PATH_FLAGS() -> (PDH_PATH_FLAGS, String) { (PDH_PATH_FLAGS::default(), "PDH_PATH_FLAGS::default()".to_string()) }
pub fn get_strange_PDH_RAW_COUNTER() -> (PDH_RAW_COUNTER, String) { (PDH_RAW_COUNTER::default(), "PDH_RAW_COUNTER::default()".to_string()) }
pub fn get_strange_PDH_RAW_COUNTER_ITEM_A() -> (PDH_RAW_COUNTER_ITEM_A, String) { (PDH_RAW_COUNTER_ITEM_A::default(), "PDH_RAW_COUNTER_ITEM_A::default()".to_string()) }
pub fn get_strange_PDH_RAW_COUNTER_ITEM_W() -> (PDH_RAW_COUNTER_ITEM_W, String) { (PDH_RAW_COUNTER_ITEM_W::default(), "PDH_RAW_COUNTER_ITEM_W::default()".to_string()) }
pub fn get_strange_PDH_RAW_LOG_RECORD() -> (PDH_RAW_LOG_RECORD, String) { (PDH_RAW_LOG_RECORD::default(), "PDH_RAW_LOG_RECORD::default()".to_string()) }
pub fn get_strange_PDH_SELECT_DATA_SOURCE_FLAGS() -> (PDH_SELECT_DATA_SOURCE_FLAGS, String) { (PDH_SELECT_DATA_SOURCE_FLAGS::default(), "PDH_SELECT_DATA_SOURCE_FLAGS::default()".to_string()) }
pub fn get_strange_PDH_STATISTICS() -> (PDH_STATISTICS, String) { (PDH_STATISTICS::default(), "PDH_STATISTICS::default()".to_string()) }
pub fn get_strange_PDH_TIME_INFO() -> (PDH_TIME_INFO, String) { (PDH_TIME_INFO::default(), "PDH_TIME_INFO::default()".to_string()) }
pub fn get_strange_PENABLECALLBACK() -> (PENABLECALLBACK, String) { (PENABLECALLBACK::default(), "PENABLECALLBACK::default()".to_string()) }
pub fn get_strange_PENUM_PAGE_FILE_CALLBACKA() -> (PENUM_PAGE_FILE_CALLBACKA, String) { (PENUM_PAGE_FILE_CALLBACKA::default(), "PENUM_PAGE_FILE_CALLBACKA::default()".to_string()) }
pub fn get_strange_PENUM_PAGE_FILE_CALLBACKW() -> (PENUM_PAGE_FILE_CALLBACKW, String) { (PENUM_PAGE_FILE_CALLBACKW::default(), "PENUM_PAGE_FILE_CALLBACKW::default()".to_string()) }
pub fn get_strange_PEN_STYLE() -> (PEN_STYLE, String) { (PEN_STYLE::default(), "PEN_STYLE::default()".to_string()) }
pub fn get_strange_PERCEIVED() -> (PERCEIVED, String) { (PERCEIVED::default(), "PERCEIVED::default()".to_string()) }
pub fn get_strange_PERFLIBREQUEST() -> (PERFLIBREQUEST, String) { (PERFLIBREQUEST::default(), "PERFLIBREQUEST::default()".to_string()) }
pub fn get_strange_PERFORMANCE_DATA() -> (PERFORMANCE_DATA, String) { (PERFORMANCE_DATA::default(), "PERFORMANCE_DATA::default()".to_string()) }
pub fn get_strange_PERFORMANCE_INFORMATION() -> (PERFORMANCE_INFORMATION, String) { (PERFORMANCE_INFORMATION::default(), "PERFORMANCE_INFORMATION::default()".to_string()) }
pub fn get_strange_PERF_COUNTERSET_INFO() -> (PERF_COUNTERSET_INFO, String) { (PERF_COUNTERSET_INFO::default(), "PERF_COUNTERSET_INFO::default()".to_string()) }
pub fn get_strange_PERF_COUNTERSET_INSTANCE() -> (PERF_COUNTERSET_INSTANCE, String) { (PERF_COUNTERSET_INSTANCE::default(), "PERF_COUNTERSET_INSTANCE::default()".to_string()) }
pub fn get_strange_PERF_COUNTER_IDENTIFIER() -> (PERF_COUNTER_IDENTIFIER, String) { (PERF_COUNTER_IDENTIFIER::default(), "PERF_COUNTER_IDENTIFIER::default()".to_string()) }
pub fn get_strange_PERF_DATA_HEADER() -> (PERF_DATA_HEADER, String) { (PERF_DATA_HEADER::default(), "PERF_DATA_HEADER::default()".to_string()) }
pub fn get_strange_PERF_DETAIL() -> (PERF_DETAIL, String) {    (PERF_DETAIL::default(), "PERF_DETAIL::default()".to_string())}
pub fn get_strange_PERF_INSTANCE_HEADER() -> (PERF_INSTANCE_HEADER, String) { (PERF_INSTANCE_HEADER::default(), "PERF_INSTANCE_HEADER::default()".to_string()) }
pub fn get_strange_PEVENT_CALLBACK() -> (PEVENT_CALLBACK, String) { (PEVENT_CALLBACK::default(), "PEVENT_CALLBACK::default()".to_string()) }
pub fn get_strange_PFADDRESSTYPE() -> (PFADDRESSTYPE, String) { (PFADDRESSTYPE::default(), "PFADDRESSTYPE::default()".to_string()) }
pub fn get_strange_PFFORWARD_ACTION() -> (PFFORWARD_ACTION, String) { (PFFORWARD_ACTION::default(), "PFFORWARD_ACTION::default()".to_string()) }
pub fn get_strange_PFLS_CALLBACK_FUNCTION() -> (PFLS_CALLBACK_FUNCTION, String) { (PFLS_CALLBACK_FUNCTION::default(), "PFLS_CALLBACK_FUNCTION::default()".to_string()) }
pub fn get_strange_PFNALLOC() -> (PFNALLOC, String) { (PFNALLOC::default(), "PFNALLOC::default()".to_string()) }
pub fn get_strange_PFNCALLBACK() -> (PFNCALLBACK, String) { (PFNCALLBACK::default(), "PFNCALLBACK::default()".to_string()) }
pub fn get_strange_PFNCLOSE() -> (PFNCLOSE, String) { (PFNCLOSE::default(), "PFNCLOSE::default()".to_string()) }
pub fn get_strange_PFNDACOMPARE() -> (PFNDACOMPARE, String) { (PFNDACOMPARE::default(), "PFNDACOMPARE::default()".to_string()) }
pub fn get_strange_PFNDAENUMCALLBACK() -> (PFNDAENUMCALLBACK, String) { (PFNDAENUMCALLBACK::default(), "PFNDAENUMCALLBACK::default()".to_string()) }
pub fn get_strange_PFNDAVAUTHCALLBACK() -> (PFNDAVAUTHCALLBACK, String) { (PFNDAVAUTHCALLBACK::default(), "PFNDAVAUTHCALLBACK::default()".to_string()) }
pub fn get_strange_PFNDPAMERGE() -> (PFNDPAMERGE, String) { (PFNDPAMERGE::default(), "PFNDPAMERGE::default()".to_string()) }
pub fn get_strange_PFNDPASTREAM() -> (PFNDPASTREAM, String) { (PFNDPASTREAM::default(), "PFNDPASTREAM::default()".to_string()) }
pub fn get_strange_PFNFCIALLOC() -> (PFNFCIALLOC, String) { (PFNFCIALLOC::default(), "PFNFCIALLOC::default()".to_string()) }
pub fn get_strange_PFNFCICLOSE() -> (PFNFCICLOSE, String) { (PFNFCICLOSE::default(), "PFNFCICLOSE::default()".to_string()) }
pub fn get_strange_PFNFCIDELETE() -> (PFNFCIDELETE, String) { (PFNFCIDELETE::default(), "PFNFCIDELETE::default()".to_string()) }
pub fn get_strange_PFNFCIFILEPLACED() -> (PFNFCIFILEPLACED, String) { (PFNFCIFILEPLACED::default(), "PFNFCIFILEPLACED::default()".to_string()) }
pub fn get_strange_PFNFCIFREE() -> (PFNFCIFREE, String) { (PFNFCIFREE::default(), "PFNFCIFREE::default()".to_string()) }
pub fn get_strange_PFNFCIGETNEXTCABINET() -> (PFNFCIGETNEXTCABINET, String) { (PFNFCIGETNEXTCABINET::default(), "PFNFCIGETNEXTCABINET::default()".to_string()) }
pub fn get_strange_PFNFCIGETOPENINFO() -> (PFNFCIGETOPENINFO, String) { (PFNFCIGETOPENINFO::default(), "PFNFCIGETOPENINFO::default()".to_string()) }
pub fn get_strange_PFNFCIGETTEMPFILE() -> (PFNFCIGETTEMPFILE, String) { (PFNFCIGETTEMPFILE::default(), "PFNFCIGETTEMPFILE::default()".to_string()) }
pub fn get_strange_PFNFCIOPEN() -> (PFNFCIOPEN, String) { (PFNFCIOPEN::default(), "PFNFCIOPEN::default()".to_string()) }
pub fn get_strange_PFNFCIREAD() -> (PFNFCIREAD, String) { (PFNFCIREAD::default(), "PFNFCIREAD::default()".to_string()) }
pub fn get_strange_PFNFCISEEK() -> (PFNFCISEEK, String) { (PFNFCISEEK::default(), "PFNFCISEEK::default()".to_string()) }
pub fn get_strange_PFNFCISTATUS() -> (PFNFCISTATUS, String) { (PFNFCISTATUS::default(), "PFNFCISTATUS::default()".to_string()) }
pub fn get_strange_PFNFCIWRITE() -> (PFNFCIWRITE, String) { (PFNFCIWRITE::default(), "PFNFCIWRITE::default()".to_string()) }
pub fn get_strange_PFNFDIDECRYPT() -> (PFNFDIDECRYPT, String) { (PFNFDIDECRYPT::default(), "PFNFDIDECRYPT::default()".to_string()) }
pub fn get_strange_PFNFDINOTIFY() -> (PFNFDINOTIFY, String) { (PFNFDINOTIFY::default(), "PFNFDINOTIFY::default()".to_string()) }
pub fn get_strange_PFNFREE() -> (PFNFREE, String) { (PFNFREE::default(), "PFNFREE::default()".to_string()) }
pub fn get_strange_PFNOPEN() -> (PFNOPEN, String) { (PFNOPEN::default(), "PFNOPEN::default()".to_string()) }
pub fn get_strange_PFNPROPSHEETUI() -> (PFNPROPSHEETUI, String) {    (PFNPROPSHEETUI::default(), "PFNPROPSHEETUI::default()".to_string())}
pub fn get_strange_PFNREAD() -> (PFNREAD, String) { (PFNREAD::default(), "PFNREAD::default()".to_string()) }
pub fn get_strange_PFNREADOBJECTSECURITY() -> (PFNREADOBJECTSECURITY, String) { (PFNREADOBJECTSECURITY::default(), "PFNREADOBJECTSECURITY::default()".to_string()) }
pub fn get_strange_PFNSEEK() -> (PFNSEEK, String) { (PFNSEEK::default(), "PFNSEEK::default()".to_string()) }
pub fn get_strange_PFNWRITE() -> (PFNWRITE, String) { (PFNWRITE::default(), "PFNWRITE::default()".to_string()) }
pub fn get_strange_PFNWRITEOBJECTSECURITY() -> (PFNWRITEOBJECTSECURITY, String) { (PFNWRITEOBJECTSECURITY::default(), "PFNWRITEOBJECTSECURITY::default()".to_string()) }
pub fn get_strange_PFN_AUTHENTICATION_CALLBACK() -> (PFN_AUTHENTICATION_CALLBACK, String) { (PFN_AUTHENTICATION_CALLBACK::default(), "PFN_AUTHENTICATION_CALLBACK::default()".to_string()) }
pub fn get_strange_PFN_AUTHENTICATION_CALLBACK_EX() -> (PFN_AUTHENTICATION_CALLBACK_EX, String) { (PFN_AUTHENTICATION_CALLBACK_EX::default(), "PFN_AUTHENTICATION_CALLBACK_EX::default()".to_string()) }
pub fn get_strange_PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS() -> (PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, String) { (PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS::default(), "PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS::default()".to_string()) }
pub fn get_strange_PFN_AUTHZ_DYNAMIC_ACCESS_CHECK() -> (PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, String) { (PFN_AUTHZ_DYNAMIC_ACCESS_CHECK::default(), "PFN_AUTHZ_DYNAMIC_ACCESS_CHECK::default()".to_string()) }
pub fn get_strange_PFN_AUTHZ_FREE_DYNAMIC_GROUPS() -> (PFN_AUTHZ_FREE_DYNAMIC_GROUPS, String) { (PFN_AUTHZ_FREE_DYNAMIC_GROUPS::default(), "PFN_AUTHZ_FREE_DYNAMIC_GROUPS::default()".to_string()) }
pub fn get_strange_PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK() -> (PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, String) { (PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK::default(), "PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK::default()".to_string()) }
pub fn get_strange_PFN_CDF_PARSE_ERROR_CALLBACK() -> (PFN_CDF_PARSE_ERROR_CALLBACK, String) { (PFN_CDF_PARSE_ERROR_CALLBACK::default(), "PFN_CDF_PARSE_ERROR_CALLBACK::default()".to_string()) }
pub fn get_strange_PF_FILTER_DESCRIPTOR() -> (PF_FILTER_DESCRIPTOR, String) { (PF_FILTER_DESCRIPTOR::default(), "PF_FILTER_DESCRIPTOR::default()".to_string()) }
pub fn get_strange_PF_INTERFACE_STATS() -> (PF_INTERFACE_STATS, String) { (PF_INTERFACE_STATS::default(), "PF_INTERFACE_STATS::default()".to_string()) }
pub fn get_strange_PF_LATEBIND_INFO() -> (PF_LATEBIND_INFO, String) { (PF_LATEBIND_INFO::default(), "PF_LATEBIND_INFO::default()".to_string()) }
pub fn get_strange_PHANDLER_ROUTINE() -> (PHANDLER_ROUTINE, String) { (PHANDLER_ROUTINE::default(), "PHANDLER_ROUTINE::default()".to_string()) }
pub fn get_strange_PHIDP_INSERT_SCANCODES() -> (PHIDP_INSERT_SCANCODES, String) { (PHIDP_INSERT_SCANCODES::default(), "PHIDP_INSERT_SCANCODES::default()".to_string()) }
pub fn get_strange_PHONEBUTTONINFO() -> (PHONEBUTTONINFO, String) { (PHONEBUTTONINFO::default(), "PHONEBUTTONINFO::default()".to_string()) }
pub fn get_strange_PHONECALLBACK() -> (PHONECALLBACK, String) { (PHONECALLBACK::default(), "PHONECALLBACK::default()".to_string()) }
pub fn get_strange_PHONECAPS() -> (PHONECAPS, String) { (PHONECAPS::default(), "PHONECAPS::default()".to_string()) }
pub fn get_strange_PHONEEXTENSIONID() -> (PHONEEXTENSIONID, String) { (PHONEEXTENSIONID::default(), "PHONEEXTENSIONID::default()".to_string()) }
pub fn get_strange_PHONEINITIALIZEEXPARAMS() -> (PHONEINITIALIZEEXPARAMS, String) { (PHONEINITIALIZEEXPARAMS::default(), "PHONEINITIALIZEEXPARAMS::default()".to_string()) }
pub fn get_strange_PHONEMESSAGE() -> (PHONEMESSAGE, String) { (PHONEMESSAGE::default(), "PHONEMESSAGE::default()".to_string()) }
pub fn get_strange_PHONESTATUS() -> (PHONESTATUS, String) { (PHONESTATUS::default(), "PHONESTATUS::default()".to_string()) }
pub fn get_strange_PHYSICAL_MONITOR() -> (PHYSICAL_MONITOR, String) { (PHYSICAL_MONITOR::default(), "PHYSICAL_MONITOR::default()".to_string()) }
pub fn get_strange_PICTDESC() -> (PICTDESC, String) { (PICTDESC::default(), "PICTDESC::default()".to_string()) }
pub fn get_strange_PINIT_ONCE_FN() -> (PINIT_ONCE_FN, String) { (PINIT_ONCE_FN::default(), "PINIT_ONCE_FN::default()".to_string()) }
pub fn get_strange_PINSPECT_HSTRING_CALLBACK() -> (PINSPECT_HSTRING_CALLBACK, String) { (PINSPECT_HSTRING_CALLBACK::default(), "PINSPECT_HSTRING_CALLBACK::default()".to_string()) }
pub fn get_strange_PINSPECT_HSTRING_CALLBACK2() -> (PINSPECT_HSTRING_CALLBACK2, String) { (PINSPECT_HSTRING_CALLBACK2::default(), "PINSPECT_HSTRING_CALLBACK2::default()".to_string()) }
pub fn get_strange_PINSPECT_MEMORY_CALLBACK() -> (PINSPECT_MEMORY_CALLBACK, String) { (PINSPECT_MEMORY_CALLBACK::default(), "PINSPECT_MEMORY_CALLBACK::default()".to_string()) }
pub fn get_strange_PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK() -> (PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, String) { (PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK::default(), "PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK::default()".to_string()) }
pub fn get_strange_PIO_APC_ROUTINE() -> (PIO_APC_ROUTINE, String) { (PIO_APC_ROUTINE::default(), "PIO_APC_ROUTINE::default()".to_string()) }
pub fn get_strange_PIPFORWARD_CHANGE_CALLBACK() -> (PIPFORWARD_CHANGE_CALLBACK, String) { (PIPFORWARD_CHANGE_CALLBACK::default(), "PIPFORWARD_CHANGE_CALLBACK::default()".to_string()) }
pub fn get_strange_PIPINTERFACE_CHANGE_CALLBACK() -> (PIPINTERFACE_CHANGE_CALLBACK, String) { (PIPINTERFACE_CHANGE_CALLBACK::default(), "PIPINTERFACE_CHANGE_CALLBACK::default()".to_string()) }
pub fn get_strange_PIXELFORMATDESCRIPTOR() -> (PIXELFORMATDESCRIPTOR, String) { (PIXELFORMATDESCRIPTOR::default(), "PIXELFORMATDESCRIPTOR::default()".to_string()) }
pub fn get_strange_PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK() -> (PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, String) { (PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK::default(), "PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK::default()".to_string()) }
pub fn get_strange_PNP_VETO_TYPE() -> (PNP_VETO_TYPE, String) { (PNP_VETO_TYPE::default(), "PNP_VETO_TYPE::default()".to_string()) }
pub fn get_strange_POINT() -> (POINT, String) {    (POINT::default(), "POINT::default()".to_string())}
pub fn get_strange_POINTER_DEVICE_CURSOR_INFO() -> (POINTER_DEVICE_CURSOR_INFO, String) { (POINTER_DEVICE_CURSOR_INFO::default(), "POINTER_DEVICE_CURSOR_INFO::default()".to_string()) }
pub fn get_strange_POINTER_DEVICE_INFO() -> (POINTER_DEVICE_INFO, String) { (POINTER_DEVICE_INFO::default(), "POINTER_DEVICE_INFO::default()".to_string()) }
pub fn get_strange_POINTER_DEVICE_PROPERTY() -> (POINTER_DEVICE_PROPERTY, String) { (POINTER_DEVICE_PROPERTY::default(), "POINTER_DEVICE_PROPERTY::default()".to_string()) }
pub fn get_strange_POINTER_FEEDBACK_MODE() -> (POINTER_FEEDBACK_MODE, String) { (POINTER_FEEDBACK_MODE::default(), "POINTER_FEEDBACK_MODE::default()".to_string()) }
pub fn get_strange_POINTER_INFO() -> (POINTER_INFO, String) { (POINTER_INFO::default(), "POINTER_INFO::default()".to_string()) }
pub fn get_strange_POINTER_INPUT_TYPE() -> (POINTER_INPUT_TYPE, String) { (POINTER_INPUT_TYPE::default(), "POINTER_INPUT_TYPE::default()".to_string()) }
pub fn get_strange_POINTER_PEN_INFO() -> (POINTER_PEN_INFO, String) { (POINTER_PEN_INFO::default(), "POINTER_PEN_INFO::default()".to_string()) }
pub fn get_strange_POINTER_TOUCH_INFO() -> (POINTER_TOUCH_INFO, String) { (POINTER_TOUCH_INFO::default(), "POINTER_TOUCH_INFO::default()".to_string()) }
pub fn get_strange_POINTFIX() -> (POINTFIX, String) { (POINTFIX::default(), "POINTFIX::default()".to_string()) }
pub fn get_strange_POINTL() -> (POINTL, String) { (POINTL::default(), "POINTL::default()".to_string()) }
pub fn get_strange_POINTQF() -> (POINTQF, String) { (POINTQF::default(), "POINTQF::default()".to_string()) }
pub fn get_strange_POINTS() -> (POINTS, String) { (POINTS::default(), "POINTS::default()".to_string()) }
pub fn get_strange_POWER_DATA_ACCESSOR() -> (POWER_DATA_ACCESSOR, String) { (POWER_DATA_ACCESSOR::default(), "POWER_DATA_ACCESSOR::default()".to_string()) }
pub fn get_strange_POWER_INFORMATION_LEVEL() -> (POWER_INFORMATION_LEVEL, String) { (POWER_INFORMATION_LEVEL::default(), "POWER_INFORMATION_LEVEL::default()".to_string()) }
pub fn get_strange_POWER_PLATFORM_ROLE_VERSION() -> (POWER_PLATFORM_ROLE_VERSION, String) { (POWER_PLATFORM_ROLE_VERSION::default(), "POWER_PLATFORM_ROLE_VERSION::default()".to_string()) }
pub fn get_strange_POWER_POLICY() -> (POWER_POLICY, String) { (POWER_POLICY::default(), "POWER_POLICY::default()".to_string()) }
pub fn get_strange_POWER_REQUEST_TYPE() -> (POWER_REQUEST_TYPE, String) { (POWER_REQUEST_TYPE::default(), "POWER_REQUEST_TYPE::default()".to_string()) }
pub fn get_strange_POWER_SETTING_REGISTER_NOTIFICATION_FLAGS() -> (POWER_SETTING_REGISTER_NOTIFICATION_FLAGS, String) { (POWER_SETTING_REGISTER_NOTIFICATION_FLAGS::default(), "POWER_SETTING_REGISTER_NOTIFICATION_FLAGS::default()".to_string()) }
pub fn get_strange_PRINTDLGA() -> (PRINTDLGA, String) { (PRINTDLGA::default(), "PRINTDLGA::default()".to_string()) }
pub fn get_strange_PRINTDLGEXA() -> (PRINTDLGEXA, String) { (PRINTDLGEXA::default(), "PRINTDLGEXA::default()".to_string()) }
pub fn get_strange_PRINTDLGEXW() -> (PRINTDLGEXW, String) { (PRINTDLGEXW::default(), "PRINTDLGEXW::default()".to_string()) }
pub fn get_strange_PRINTDLGW() -> (PRINTDLGW, String) { (PRINTDLGW::default(), "PRINTDLGW::default()".to_string()) }
pub fn get_strange_PRINT_EXECUTION_DATA() -> (PRINT_EXECUTION_DATA, String) { (PRINT_EXECUTION_DATA::default(), "PRINT_EXECUTION_DATA::default()".to_string()) }
pub fn get_strange_PRINT_WINDOW_FLAGS() -> (PRINT_WINDOW_FLAGS, String) { (PRINT_WINDOW_FLAGS::default(), "PRINT_WINDOW_FLAGS::default()".to_string()) }
pub fn get_strange_PRIORITY() -> (PRIORITY, String) { (PRIORITY::default(), "PRIORITY::default()".to_string()) }
pub fn get_strange_PRIVILEGE_SET() -> (PRIVILEGE_SET, String) { (PRIVILEGE_SET::default(), "PRIVILEGE_SET::default()".to_string()) }
pub fn get_strange_PROCESSENTRY32() -> (PROCESSENTRY32, String) { (PROCESSENTRY32::default(), "PROCESSENTRY32::default()".to_string()) }
pub fn get_strange_PROCESSENTRY32W() -> (PROCESSENTRY32W, String) { (PROCESSENTRY32W::default(), "PROCESSENTRY32W::default()".to_string()) }
pub fn get_strange_PROCESSINFOCLASS() -> (PROCESSINFOCLASS, String) { (PROCESSINFOCLASS::default(), "PROCESSINFOCLASS::default()".to_string()) }
pub fn get_strange_PROCESSOR_FEATURE_ID() -> (PROCESSOR_FEATURE_ID, String) { (PROCESSOR_FEATURE_ID::default(), "PROCESSOR_FEATURE_ID::default()".to_string()) }
pub fn get_strange_PROCESSOR_NUMBER() -> (PROCESSOR_NUMBER, String) { (PROCESSOR_NUMBER::default(), "PROCESSOR_NUMBER::default()".to_string()) }
pub fn get_strange_PROCESS_ACCESS_RIGHTS() -> (PROCESS_ACCESS_RIGHTS, String) { (PROCESS_ACCESS_RIGHTS::default(), "PROCESS_ACCESS_RIGHTS::default()".to_string()) }
pub fn get_strange_PROCESS_AFFINITY_AUTO_UPDATE_FLAGS() -> (PROCESS_AFFINITY_AUTO_UPDATE_FLAGS, String) { (PROCESS_AFFINITY_AUTO_UPDATE_FLAGS::default(), "PROCESS_AFFINITY_AUTO_UPDATE_FLAGS::default()".to_string()) }
pub fn get_strange_PROCESS_CREATION_FLAGS() -> (PROCESS_CREATION_FLAGS, String) { (PROCESS_CREATION_FLAGS::default(), "PROCESS_CREATION_FLAGS::default()".to_string()) }
pub fn get_strange_PROCESS_DEP_FLAGS() -> (PROCESS_DEP_FLAGS, String) { (PROCESS_DEP_FLAGS::default(), "PROCESS_DEP_FLAGS::default()".to_string()) }
pub fn get_strange_PROCESS_DPI_AWARENESS() -> (PROCESS_DPI_AWARENESS, String) { (PROCESS_DPI_AWARENESS::default(), "PROCESS_DPI_AWARENESS::default()".to_string()) }
pub fn get_strange_PROCESS_DYNAMIC_EH_CONTINUATION_TARGET() -> (PROCESS_DYNAMIC_EH_CONTINUATION_TARGET, String) { (PROCESS_DYNAMIC_EH_CONTINUATION_TARGET::default(), "PROCESS_DYNAMIC_EH_CONTINUATION_TARGET::default()".to_string()) }
pub fn get_strange_PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE() -> (PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE, String) { (PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE::default(), "PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE::default()".to_string()) }
pub fn get_strange_PROCESS_INFORMATION() -> (PROCESS_INFORMATION, String) { (PROCESS_INFORMATION::default(), "PROCESS_INFORMATION::default()".to_string()) }
pub fn get_strange_PROCESS_INFORMATION_CLASS() -> (PROCESS_INFORMATION_CLASS, String) { (PROCESS_INFORMATION_CLASS::default(), "PROCESS_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_PROCESS_MEMORY_COUNTERS() -> (PROCESS_MEMORY_COUNTERS, String) { (PROCESS_MEMORY_COUNTERS::default(), "PROCESS_MEMORY_COUNTERS::default()".to_string()) }
pub fn get_strange_PROCESS_MITIGATION_POLICY() -> (PROCESS_MITIGATION_POLICY, String) { (PROCESS_MITIGATION_POLICY::default(), "PROCESS_MITIGATION_POLICY::default()".to_string()) }
pub fn get_strange_PROCESS_NAME_FORMAT() -> (PROCESS_NAME_FORMAT, String) { (PROCESS_NAME_FORMAT::default(), "PROCESS_NAME_FORMAT::default()".to_string()) }
pub fn get_strange_PROFILEINFOA() -> (PROFILEINFOA, String) { (PROFILEINFOA::default(), "PROFILEINFOA::default()".to_string()) }
pub fn get_strange_PROFILEINFOW() -> (PROFILEINFOW, String) { (PROFILEINFOW::default(), "PROFILEINFOW::default()".to_string()) }
pub fn get_strange_PROG_INVOKE_SETTING() -> (PROG_INVOKE_SETTING, String) { (PROG_INVOKE_SETTING::default(), "PROG_INVOKE_SETTING::default()".to_string()) }
pub fn get_strange_PROPERTYKEY() -> (PROPERTYKEY, String) { (PROPERTYKEY::default(), "PROPERTYKEY::default()".to_string()) }
pub fn get_strange_PROPERTYORIGIN() -> (PROPERTYORIGIN, String) { (PROPERTYORIGIN::default(), "PROPERTYORIGIN::default()".to_string()) }
pub fn get_strange_PROPSHEETHEADERA_V2() -> (PROPSHEETHEADERA_V2, String) { (PROPSHEETHEADERA_V2::default(), "PROPSHEETHEADERA_V2::default()".to_string()) }
pub fn get_strange_PROPSHEETHEADERW_V2() -> (PROPSHEETHEADERW_V2, String) { (PROPSHEETHEADERW_V2::default(), "PROPSHEETHEADERW_V2::default()".to_string()) }
pub fn get_strange_PROPSHEETPAGEA() -> (PROPSHEETPAGEA, String) { (PROPSHEETPAGEA::default(), "PROPSHEETPAGEA::default()".to_string()) }
pub fn get_strange_PROPSHEETPAGEW() -> (PROPSHEETPAGEW, String) { (PROPSHEETPAGEW::default(), "PROPSHEETPAGEW::default()".to_string()) }
pub fn get_strange_PROPVARIANT() -> (PROPVARIANT, String) { (PROPVARIANT::default(), "PROPVARIANT::default()".to_string()) }
pub fn get_strange_PROVIDER_ENUMERATION_INFO() -> (PROVIDER_ENUMERATION_INFO, String) { (PROVIDER_ENUMERATION_INFO::default(), "PROVIDER_ENUMERATION_INFO::default()".to_string()) }
pub fn get_strange_PROVIDER_EVENT_INFO() -> (PROVIDER_EVENT_INFO, String) { (PROVIDER_EVENT_INFO::default(), "PROVIDER_EVENT_INFO::default()".to_string()) }
pub fn get_strange_PROVIDER_FIELD_INFOARRAY() -> (PROVIDER_FIELD_INFOARRAY, String) { (PROVIDER_FIELD_INFOARRAY::default(), "PROVIDER_FIELD_INFOARRAY::default()".to_string()) }
pub fn get_strange_PROXY_AUTO_DETECT_TYPE() -> (PROXY_AUTO_DETECT_TYPE, String) { (PROXY_AUTO_DETECT_TYPE::default(), "PROXY_AUTO_DETECT_TYPE::default()".to_string()) }
pub fn get_strange_PSAPI_WS_WATCH_INFORMATION() -> (PSAPI_WS_WATCH_INFORMATION, String) { (PSAPI_WS_WATCH_INFORMATION::default(), "PSAPI_WS_WATCH_INFORMATION::default()".to_string()) }
pub fn get_strange_PSAPI_WS_WATCH_INFORMATION_EX() -> (PSAPI_WS_WATCH_INFORMATION_EX, String) { (PSAPI_WS_WATCH_INFORMATION_EX::default(), "PSAPI_WS_WATCH_INFORMATION_EX::default()".to_string()) }
pub fn get_strange_PSID() -> (PSID, String) {    (PSID::default(), "PSID::default()".to_string())}
pub fn get_strange_PSP_DETSIG_CMPPROC() -> (PSP_DETSIG_CMPPROC, String) { (PSP_DETSIG_CMPPROC::default(), "PSP_DETSIG_CMPPROC::default()".to_string()) }
pub fn get_strange_PSP_FILE_CALLBACK_A() -> (PSP_FILE_CALLBACK_A, String) { (PSP_FILE_CALLBACK_A::default(), "PSP_FILE_CALLBACK_A::default()".to_string()) }
pub fn get_strange_PSP_FILE_CALLBACK_W() -> (PSP_FILE_CALLBACK_W, String) { (PSP_FILE_CALLBACK_W::default(), "PSP_FILE_CALLBACK_W::default()".to_string()) }
pub fn get_strange_PSS_CAPTURE_FLAGS() -> (PSS_CAPTURE_FLAGS, String) { (PSS_CAPTURE_FLAGS::default(), "PSS_CAPTURE_FLAGS::default()".to_string()) }
pub fn get_strange_PSS_DUPLICATE_FLAGS() -> (PSS_DUPLICATE_FLAGS, String) { (PSS_DUPLICATE_FLAGS::default(), "PSS_DUPLICATE_FLAGS::default()".to_string()) }
pub fn get_strange_PSS_QUERY_INFORMATION_CLASS() -> (PSS_QUERY_INFORMATION_CLASS, String) { (PSS_QUERY_INFORMATION_CLASS::default(), "PSS_QUERY_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_PSS_WALK_INFORMATION_CLASS() -> (PSS_WALK_INFORMATION_CLASS, String) { (PSS_WALK_INFORMATION_CLASS::default(), "PSS_WALK_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK() -> (PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, String) { (PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK::default(), "PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK::default()".to_string()) }
pub fn get_strange_PSTIME_FLAGS() -> (PSTIME_FLAGS, String) { (PSTIME_FLAGS::default(), "PSTIME_FLAGS::default()".to_string()) }
pub fn get_strange_PSTR() -> (PSTR, String) {    (PSTR::default(), "PSTR::default()".to_string())}
pub fn get_strange_PSUACTION() -> (PSUACTION, String) { (PSUACTION::default(), "PSUACTION::default()".to_string()) }
pub fn get_strange_PTEREDO_PORT_CHANGE_CALLBACK() -> (PTEREDO_PORT_CHANGE_CALLBACK, String) { (PTEREDO_PORT_CHANGE_CALLBACK::default(), "PTEREDO_PORT_CHANGE_CALLBACK::default()".to_string()) }
pub fn get_strange_PTIMERAPCROUTINE() -> (PTIMERAPCROUTINE, String) { (PTIMERAPCROUTINE::default(), "PTIMERAPCROUTINE::default()".to_string()) }
pub fn get_strange_PTP_POOL() -> (PTP_POOL, String) { (PTP_POOL::default(), "PTP_POOL::default()".to_string()) }
pub fn get_strange_PTP_SIMPLE_CALLBACK() -> (PTP_SIMPLE_CALLBACK, String) { (PTP_SIMPLE_CALLBACK::default(), "PTP_SIMPLE_CALLBACK::default()".to_string()) }
pub fn get_strange_PTP_TIMER_CALLBACK() -> (PTP_TIMER_CALLBACK, String) { (PTP_TIMER_CALLBACK::default(), "PTP_TIMER_CALLBACK::default()".to_string()) }
pub fn get_strange_PTP_WAIT_CALLBACK() -> (PTP_WAIT_CALLBACK, String) { (PTP_WAIT_CALLBACK::default(), "PTP_WAIT_CALLBACK::default()".to_string()) }
pub fn get_strange_PTP_WIN32_IO_CALLBACK() -> (PTP_WIN32_IO_CALLBACK, String) { (PTP_WIN32_IO_CALLBACK::default(), "PTP_WIN32_IO_CALLBACK::default()".to_string()) }
pub fn get_strange_PTP_WORK_CALLBACK() -> (PTP_WORK_CALLBACK, String) { (PTP_WORK_CALLBACK::default(), "PTP_WORK_CALLBACK::default()".to_string()) }
pub fn get_strange_PWRSCHEMESENUMPROC() -> (PWRSCHEMESENUMPROC, String) { (PWRSCHEMESENUMPROC::default(), "PWRSCHEMESENUMPROC::default()".to_string()) }
pub fn get_strange_PWSTR() -> (PWSTR, String) {    (PWSTR::default(), "PWSTR::default()".to_string())}
pub fn get_strange_PackageDependencyLifetimeKind() -> (PackageDependencyLifetimeKind, String) { (PackageDependencyLifetimeKind::default(), "PackageDependencyLifetimeKind::default()".to_string()) }
pub fn get_strange_PackageDependencyProcessorArchitectures() -> (PackageDependencyProcessorArchitectures, String) { (PackageDependencyProcessorArchitectures::default(), "PackageDependencyProcessorArchitectures::default()".to_string()) }
pub fn get_strange_PackageOrigin() -> (PackageOrigin, String) { (PackageOrigin::default(), "PackageOrigin::default()".to_string()) }
pub fn get_strange_PackagePathType() -> (PackagePathType, String) { (PackagePathType::default(), "PackagePathType::default()".to_string()) }
pub fn get_strange_PerfProviderHandle() -> (PerfProviderHandle, String) { (PerfProviderHandle::default(), "PerfProviderHandle::default()".to_string()) }
pub fn get_strange_PerfQueryHandle() -> (PerfQueryHandle, String) { (PerfQueryHandle::default(), "PerfQueryHandle::default()".to_string()) }
pub fn get_strange_PerfRegInfoType() -> (PerfRegInfoType, String) { (PerfRegInfoType::default(), "PerfRegInfoType::default()".to_string()) }
pub fn get_strange_PrintAsyncNotifyConversationStyle() -> (PrintAsyncNotifyConversationStyle, String) { (PrintAsyncNotifyConversationStyle::default(), "PrintAsyncNotifyConversationStyle::default()".to_string()) }
pub fn get_strange_PrintAsyncNotifyUserFilter() -> (PrintAsyncNotifyUserFilter, String) { (PrintAsyncNotifyUserFilter::default(), "PrintAsyncNotifyUserFilter::default()".to_string()) }
pub fn get_strange_PrintPropertyValue() -> (PrintPropertyValue, String) { (PrintPropertyValue::default(), "PrintPropertyValue::default()".to_string()) }
pub fn get_strange_QOCINFO() -> (QOCINFO, String) { (QOCINFO::default(), "QOCINFO::default()".to_string()) }
pub fn get_strange_QOS_NOTIFY_FLOW() -> (QOS_NOTIFY_FLOW, String) { (QOS_NOTIFY_FLOW::default(), "QOS_NOTIFY_FLOW::default()".to_string()) }
pub fn get_strange_QOS_QUERY_FLOW() -> (QOS_QUERY_FLOW, String) { (QOS_QUERY_FLOW::default(), "QOS_QUERY_FLOW::default()".to_string()) }
pub fn get_strange_QOS_SET_FLOW() -> (QOS_SET_FLOW, String) { (QOS_SET_FLOW::default(), "QOS_SET_FLOW::default()".to_string()) }
pub fn get_strange_QOS_TRAFFIC_TYPE() -> (QOS_TRAFFIC_TYPE, String) { (QOS_TRAFFIC_TYPE::default(), "QOS_TRAFFIC_TYPE::default()".to_string()) }
pub fn get_strange_QUERYCONTEXT() -> (QUERYCONTEXT, String) { (QUERYCONTEXT::default(), "QUERYCONTEXT::default()".to_string()) }
pub fn get_strange_QUERYOPTION() -> (QUERYOPTION, String) { (QUERYOPTION::default(), "QUERYOPTION::default()".to_string()) }
pub fn get_strange_QUERY_CHANGES_VIRTUAL_DISK_FLAG() -> (QUERY_CHANGES_VIRTUAL_DISK_FLAG, String) { (QUERY_CHANGES_VIRTUAL_DISK_FLAG::default(), "QUERY_CHANGES_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_QUERY_CHANGES_VIRTUAL_DISK_RANGE() -> (QUERY_CHANGES_VIRTUAL_DISK_RANGE, String) { (QUERY_CHANGES_VIRTUAL_DISK_RANGE::default(), "QUERY_CHANGES_VIRTUAL_DISK_RANGE::default()".to_string()) }
pub fn get_strange_QUERY_SERVICE_CONFIGA() -> (QUERY_SERVICE_CONFIGA, String) { (QUERY_SERVICE_CONFIGA::default(), "QUERY_SERVICE_CONFIGA::default()".to_string()) }
pub fn get_strange_QUERY_SERVICE_CONFIGW() -> (QUERY_SERVICE_CONFIGW, String) { (QUERY_SERVICE_CONFIGW::default(), "QUERY_SERVICE_CONFIGW::default()".to_string()) }
pub fn get_strange_QUERY_SERVICE_LOCK_STATUSA() -> (QUERY_SERVICE_LOCK_STATUSA, String) { (QUERY_SERVICE_LOCK_STATUSA::default(), "QUERY_SERVICE_LOCK_STATUSA::default()".to_string()) }
pub fn get_strange_QUERY_SERVICE_LOCK_STATUSW() -> (QUERY_SERVICE_LOCK_STATUSW, String) { (QUERY_SERVICE_LOCK_STATUSW::default(), "QUERY_SERVICE_LOCK_STATUSW::default()".to_string()) }
pub fn get_strange_QUEUE_USER_APC_FLAGS() -> (QUEUE_USER_APC_FLAGS, String) { (QUEUE_USER_APC_FLAGS::default(), "QUEUE_USER_APC_FLAGS::default()".to_string()) }
pub fn get_strange_QUOTA_LIMITS() -> (QUOTA_LIMITS, String) { (QUOTA_LIMITS::default(), "QUOTA_LIMITS::default()".to_string()) }
pub fn get_strange_R2_MODE() -> (R2_MODE, String) { (R2_MODE::default(), "R2_MODE::default()".to_string()) }
pub fn get_strange_RASTERIZER_STATUS() -> (RASTERIZER_STATUS, String) { (RASTERIZER_STATUS::default(), "RASTERIZER_STATUS::default()".to_string()) }
pub fn get_strange_RAWINPUT() -> (RAWINPUT, String) { (RAWINPUT::default(), "RAWINPUT::default()".to_string()) }
pub fn get_strange_RAWINPUTDEVICE() -> (RAWINPUTDEVICE, String) { (RAWINPUTDEVICE::default(), "RAWINPUTDEVICE::default()".to_string()) }
pub fn get_strange_RAWINPUTDEVICELIST() -> (RAWINPUTDEVICELIST, String) { (RAWINPUTDEVICELIST::default(), "RAWINPUTDEVICELIST::default()".to_string()) }
pub fn get_strange_RAW_INPUT_DATA_COMMAND_FLAGS() -> (RAW_INPUT_DATA_COMMAND_FLAGS, String) { (RAW_INPUT_DATA_COMMAND_FLAGS::default(), "RAW_INPUT_DATA_COMMAND_FLAGS::default()".to_string()) }
pub fn get_strange_RAW_INPUT_DEVICE_INFO_COMMAND() -> (RAW_INPUT_DEVICE_INFO_COMMAND, String) { (RAW_INPUT_DEVICE_INFO_COMMAND::default(), "RAW_INPUT_DEVICE_INFO_COMMAND::default()".to_string()) }
pub fn get_strange_RAW_SCSI_VIRTUAL_DISK_FLAG() -> (RAW_SCSI_VIRTUAL_DISK_FLAG, String) { (RAW_SCSI_VIRTUAL_DISK_FLAG::default(), "RAW_SCSI_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_RAW_SCSI_VIRTUAL_DISK_RESPONSE() -> (RAW_SCSI_VIRTUAL_DISK_RESPONSE, String) { (RAW_SCSI_VIRTUAL_DISK_RESPONSE::default(), "RAW_SCSI_VIRTUAL_DISK_RESPONSE::default()".to_string()) }
pub fn get_strange_READEMBEDPROC() -> (READEMBEDPROC, String) { (READEMBEDPROC::default(), "READEMBEDPROC::default()".to_string()) }
pub fn get_strange_READ_EVENT_LOG_READ_FLAGS() -> (READ_EVENT_LOG_READ_FLAGS, String) { (READ_EVENT_LOG_READ_FLAGS::default(), "READ_EVENT_LOG_READ_FLAGS::default()".to_string()) }
pub fn get_strange_REAL_TIME_DATA_SOURCE_ID_FLAGS() -> (REAL_TIME_DATA_SOURCE_ID_FLAGS, String) { (REAL_TIME_DATA_SOURCE_ID_FLAGS::default(), "REAL_TIME_DATA_SOURCE_ID_FLAGS::default()".to_string()) }
pub fn get_strange_RECO_ATTRS() -> (RECO_ATTRS, String) { (RECO_ATTRS::default(), "RECO_ATTRS::default()".to_string()) }
pub fn get_strange_RECT() -> (RECT, String) { (RECT::default(), "RECT::default()".to_string()) }
pub fn get_strange_RECTFX() -> (RECTFX, String) { (RECTFX::default(), "RECTFX::default()".to_string()) }
pub fn get_strange_RECTL() -> (RECTL, String) { (RECTL::default(), "RECTL::default()".to_string()) }
pub fn get_strange_REDRAW_WINDOW_FLAGS() -> (REDRAW_WINDOW_FLAGS, String) { (REDRAW_WINDOW_FLAGS::default(), "REDRAW_WINDOW_FLAGS::default()".to_string()) }
pub fn get_strange_REGISTERWORDENUMPROCA() -> (REGISTERWORDENUMPROCA, String) { (REGISTERWORDENUMPROCA::default(), "REGISTERWORDENUMPROCA::default()".to_string()) }
pub fn get_strange_REGISTERWORDENUMPROCW() -> (REGISTERWORDENUMPROCW, String) { (REGISTERWORDENUMPROCW::default(), "REGISTERWORDENUMPROCW::default()".to_string()) }
pub fn get_strange_REGISTER_APPLICATION_RESTART_FLAGS() -> (REGISTER_APPLICATION_RESTART_FLAGS, String) { (REGISTER_APPLICATION_RESTART_FLAGS::default(), "REGISTER_APPLICATION_RESTART_FLAGS::default()".to_string()) }
pub fn get_strange_REGISTER_TOUCH_WINDOW_FLAGS() -> (REGISTER_TOUCH_WINDOW_FLAGS, String) { (REGISTER_TOUCH_WINDOW_FLAGS::default(), "REGISTER_TOUCH_WINDOW_FLAGS::default()".to_string()) }
pub fn get_strange_REGKIND() -> (REGKIND, String) { (REGKIND::default(), "REGKIND::default()".to_string()) }
pub fn get_strange_REG_CREATE_KEY_DISPOSITION() -> (REG_CREATE_KEY_DISPOSITION, String) { (REG_CREATE_KEY_DISPOSITION::default(), "REG_CREATE_KEY_DISPOSITION::default()".to_string()) }
pub fn get_strange_REG_NOTIFY_FILTER() -> (REG_NOTIFY_FILTER, String) { (REG_NOTIFY_FILTER::default(), "REG_NOTIFY_FILTER::default()".to_string()) }
pub fn get_strange_REG_OPEN_CREATE_OPTIONS() -> (REG_OPEN_CREATE_OPTIONS, String) { (REG_OPEN_CREATE_OPTIONS::default(), "REG_OPEN_CREATE_OPTIONS::default()".to_string()) }
pub fn get_strange_REG_RESTORE_KEY_FLAGS() -> (REG_RESTORE_KEY_FLAGS, String) { (REG_RESTORE_KEY_FLAGS::default(), "REG_RESTORE_KEY_FLAGS::default()".to_string()) }
pub fn get_strange_REG_SAM_FLAGS() -> (REG_SAM_FLAGS, String) { (REG_SAM_FLAGS::default(), "REG_SAM_FLAGS::default()".to_string()) }
pub fn get_strange_REG_SAVE_FORMAT() -> (REG_SAVE_FORMAT, String) { (REG_SAVE_FORMAT::default(), "REG_SAVE_FORMAT::default()".to_string()) }
pub fn get_strange_REG_VALUE_TYPE() -> (REG_VALUE_TYPE, String) { (REG_VALUE_TYPE::default(), "REG_VALUE_TYPE::default()".to_string()) }
pub fn get_strange_REPORT_EVENT_TYPE() -> (REPORT_EVENT_TYPE, String) { (REPORT_EVENT_TYPE::default(), "REPORT_EVENT_TYPE::default()".to_string()) }
pub fn get_strange_REPORT_STORE_TYPES() -> (REPORT_STORE_TYPES, String) { (REPORT_STORE_TYPES::default(), "REPORT_STORE_TYPES::default()".to_string()) }
pub fn get_strange_RESIZE_VIRTUAL_DISK_FLAG() -> (RESIZE_VIRTUAL_DISK_FLAG, String) { (RESIZE_VIRTUAL_DISK_FLAG::default(), "RESIZE_VIRTUAL_DISK_FLAG::default()".to_string()) }
pub fn get_strange_RESTRICTIONS() -> (RESTRICTIONS, String) { (RESTRICTIONS::default(), "RESTRICTIONS::default()".to_string()) }
pub fn get_strange_RGBQUAD() -> (RGBQUAD, String) { (RGBQUAD::default(), "RGBQUAD::default()".to_string()) }
pub fn get_strange_RGNDATA() -> (RGNDATA, String) { (RGNDATA::default(), "RGNDATA::default()".to_string()) }
pub fn get_strange_RGN_COMBINE_MODE() -> (RGN_COMBINE_MODE, String) { (RGN_COMBINE_MODE::default(), "RGN_COMBINE_MODE::default()".to_string()) }
pub fn get_strange_RM_FILTER_ACTION() -> (RM_FILTER_ACTION, String) { (RM_FILTER_ACTION::default(), "RM_FILTER_ACTION::default()".to_string()) }
pub fn get_strange_RM_PROCESS_INFO() -> (RM_PROCESS_INFO, String) { (RM_PROCESS_INFO::default(), "RM_PROCESS_INFO::default()".to_string()) }
pub fn get_strange_RM_WRITE_STATUS_CALLBACK() -> (RM_WRITE_STATUS_CALLBACK, String) { (RM_WRITE_STATUS_CALLBACK::default(), "RM_WRITE_STATUS_CALLBACK::default()".to_string()) }
pub fn get_strange_ROPARAMIIDHANDLE() -> (ROPARAMIIDHANDLE, String) { (ROPARAMIIDHANDLE::default(), "ROPARAMIIDHANDLE::default()".to_string()) }
pub fn get_strange_ROP_CODE() -> (ROP_CODE, String) { (ROP_CODE::default(), "ROP_CODE::default()".to_string()) }
pub fn get_strange_RO_INIT_TYPE() -> (RO_INIT_TYPE, String) { (RO_INIT_TYPE::default(), "RO_INIT_TYPE::default()".to_string()) }
pub fn get_strange_RPC_C_AUTHN_LEVEL() -> (RPC_C_AUTHN_LEVEL, String) { (RPC_C_AUTHN_LEVEL::default(), "RPC_C_AUTHN_LEVEL::default()".to_string()) }
pub fn get_strange_RPC_C_IMP_LEVEL() -> (RPC_C_IMP_LEVEL, String) { (RPC_C_IMP_LEVEL::default(), "RPC_C_IMP_LEVEL::default()".to_string()) }
pub fn get_strange_RRF_RT() -> (RRF_RT, String) { (RRF_RT::default(), "RRF_RT::default()".to_string()) }
pub fn get_strange_RTL_BARRIER() -> (RTL_BARRIER, String) { (RTL_BARRIER::default(), "RTL_BARRIER::default()".to_string()) }
pub fn get_strange_RTL_CONDITION_VARIABLE() -> (RTL_CONDITION_VARIABLE, String) { (RTL_CONDITION_VARIABLE::default(), "RTL_CONDITION_VARIABLE::default()".to_string()) }
pub fn get_strange_RTL_CRITICAL_SECTION() -> (RTL_CRITICAL_SECTION, String) { (RTL_CRITICAL_SECTION::default(), "RTL_CRITICAL_SECTION::default()".to_string()) }
pub fn get_strange_RTL_RUN_ONCE() -> (RTL_RUN_ONCE, String) { (RTL_RUN_ONCE::default(), "RTL_RUN_ONCE::default()".to_string()) }
pub fn get_strange_RTL_SRWLOCK() -> (RTL_SRWLOCK, String) { (RTL_SRWLOCK::default(), "RTL_SRWLOCK::default()".to_string()) }
pub fn get_strange_RTL_UMS_THREAD_INFO_CLASS() -> (RTL_UMS_THREAD_INFO_CLASS, String) { (RTL_UMS_THREAD_INFO_CLASS::default(), "RTL_UMS_THREAD_INFO_CLASS::default()".to_string()) }
pub fn get_strange_SAFEARRAY() -> (SAFEARRAY, String) { (SAFEARRAY::default(), "SAFEARRAY::default()".to_string()) }
pub fn get_strange_SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS() -> (SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, String) { (SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS::default(), "SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS::default()".to_string()) }
pub fn get_strange_SAFER_LEVEL_HANDLE() -> (SAFER_LEVEL_HANDLE, String) { (SAFER_LEVEL_HANDLE::default(), "SAFER_LEVEL_HANDLE::default()".to_string()) }
pub fn get_strange_SAFER_OBJECT_INFO_CLASS() -> (SAFER_OBJECT_INFO_CLASS, String) { (SAFER_OBJECT_INFO_CLASS::default(), "SAFER_OBJECT_INFO_CLASS::default()".to_string()) }
pub fn get_strange_SAFER_POLICY_INFO_CLASS() -> (SAFER_POLICY_INFO_CLASS, String) { (SAFER_POLICY_INFO_CLASS::default(), "SAFER_POLICY_INFO_CLASS::default()".to_string()) }
pub fn get_strange_SCARD_IO_REQUEST() -> (SCARD_IO_REQUEST, String) { (SCARD_IO_REQUEST::default(), "SCARD_IO_REQUEST::default()".to_string()) }
pub fn get_strange_SCARD_READERSTATEA() -> (SCARD_READERSTATEA, String) { (SCARD_READERSTATEA::default(), "SCARD_READERSTATEA::default()".to_string()) }
pub fn get_strange_SCARD_READERSTATEW() -> (SCARD_READERSTATEW, String) { (SCARD_READERSTATEW::default(), "SCARD_READERSTATEW::default()".to_string()) }
pub fn get_strange_SCARD_SCOPE() -> (SCARD_SCOPE, String) { (SCARD_SCOPE::default(), "SCARD_SCOPE::default()".to_string()) }
pub fn get_strange_SCNRT_STATUS() -> (SCNRT_STATUS, String) { (SCNRT_STATUS::default(), "SCNRT_STATUS::default()".to_string()) }
pub fn get_strange_SCROLLBAR_CONSTANTS() -> (SCROLLBAR_CONSTANTS, String) { (SCROLLBAR_CONSTANTS::default(), "SCROLLBAR_CONSTANTS::default()".to_string()) }
pub fn get_strange_SCROLLINFO() -> (SCROLLINFO, String) { (SCROLLINFO::default(), "SCROLLINFO::default()".to_string()) }
pub fn get_strange_SC_ENUM_TYPE() -> (SC_ENUM_TYPE, String) { (SC_ENUM_TYPE::default(), "SC_ENUM_TYPE::default()".to_string()) }
pub fn get_strange_SC_HANDLE() -> (SC_HANDLE, String) {    (SC_HANDLE::default(), "SC_HANDLE::default()".to_string())}
pub fn get_strange_SC_STATUS_TYPE() -> (SC_STATUS_TYPE, String) { (SC_STATUS_TYPE::default(), "SC_STATUS_TYPE::default()".to_string()) }
pub fn get_strange_SDP_ELEMENT_DATA() -> (SDP_ELEMENT_DATA, String) { (SDP_ELEMENT_DATA::default(), "SDP_ELEMENT_DATA::default()".to_string()) }
pub fn get_strange_SECURITY_ATTRIBUTES() -> (SECURITY_ATTRIBUTES, String) { (SECURITY_ATTRIBUTES::default(), "SECURITY_ATTRIBUTES::default()".to_string()) }
pub fn get_strange_SECURITY_AUTO_INHERIT_FLAGS() -> (SECURITY_AUTO_INHERIT_FLAGS, String) { (SECURITY_AUTO_INHERIT_FLAGS::default(), "SECURITY_AUTO_INHERIT_FLAGS::default()".to_string()) }
pub fn get_strange_SECURITY_DESCRIPTOR() -> (SECURITY_DESCRIPTOR, String) { (SECURITY_DESCRIPTOR::default(), "SECURITY_DESCRIPTOR::default()".to_string()) }
pub fn get_strange_SECURITY_IMPERSONATION_LEVEL() -> (SECURITY_IMPERSONATION_LEVEL, String) { (SECURITY_IMPERSONATION_LEVEL::default(), "SECURITY_IMPERSONATION_LEVEL::default()".to_string()) }
pub fn get_strange_SECURITY_QUALITY_OF_SERVICE() -> (SECURITY_QUALITY_OF_SERVICE, String) { (SECURITY_QUALITY_OF_SERVICE::default(), "SECURITY_QUALITY_OF_SERVICE::default()".to_string()) }
pub fn get_strange_SERIALIZEDPROPERTYVALUE() -> (SERIALIZEDPROPERTYVALUE, String) { (SERIALIZEDPROPERTYVALUE::default(), "SERIALIZEDPROPERTYVALUE::default()".to_string()) }
pub fn get_strange_SERVICE_CONFIG() -> (SERVICE_CONFIG, String) { (SERVICE_CONFIG::default(), "SERVICE_CONFIG::default()".to_string()) }
pub fn get_strange_SERVICE_DIRECTORY_TYPE() -> (SERVICE_DIRECTORY_TYPE, String) { (SERVICE_DIRECTORY_TYPE::default(), "SERVICE_DIRECTORY_TYPE::default()".to_string()) }
pub fn get_strange_SERVICE_ERROR() -> (SERVICE_ERROR, String) { (SERVICE_ERROR::default(), "SERVICE_ERROR::default()".to_string()) }
pub fn get_strange_SERVICE_NOTIFY() -> (SERVICE_NOTIFY, String) { (SERVICE_NOTIFY::default(), "SERVICE_NOTIFY::default()".to_string()) }
pub fn get_strange_SERVICE_REGISTRY_STATE_TYPE() -> (SERVICE_REGISTRY_STATE_TYPE, String) { (SERVICE_REGISTRY_STATE_TYPE::default(), "SERVICE_REGISTRY_STATE_TYPE::default()".to_string()) }
pub fn get_strange_SERVICE_SHARED_DIRECTORY_TYPE() -> (SERVICE_SHARED_DIRECTORY_TYPE, String) { (SERVICE_SHARED_DIRECTORY_TYPE::default(), "SERVICE_SHARED_DIRECTORY_TYPE::default()".to_string()) }
pub fn get_strange_SERVICE_SHARED_REGISTRY_STATE_TYPE() -> (SERVICE_SHARED_REGISTRY_STATE_TYPE, String) { (SERVICE_SHARED_REGISTRY_STATE_TYPE::default(), "SERVICE_SHARED_REGISTRY_STATE_TYPE::default()".to_string()) }
pub fn get_strange_SERVICE_START_TYPE() -> (SERVICE_START_TYPE, String) { (SERVICE_START_TYPE::default(), "SERVICE_START_TYPE::default()".to_string()) }
pub fn get_strange_SERVICE_STATUS() -> (SERVICE_STATUS, String) { (SERVICE_STATUS::default(), "SERVICE_STATUS::default()".to_string()) }
pub fn get_strange_SERVICE_STATUS_HANDLE() -> (SERVICE_STATUS_HANDLE, String) { (SERVICE_STATUS_HANDLE::default(), "SERVICE_STATUS_HANDLE::default()".to_string()) }
pub fn get_strange_SETUP_DI_BUILD_DRIVER_DRIVER_TYPE() -> (SETUP_DI_BUILD_DRIVER_DRIVER_TYPE, String) { (SETUP_DI_BUILD_DRIVER_DRIVER_TYPE::default(), "SETUP_DI_BUILD_DRIVER_DRIVER_TYPE::default()".to_string()) }
pub fn get_strange_SETUP_FILE_OPERATION() -> (SETUP_FILE_OPERATION, String) {    (SETUP_FILE_OPERATION::default(), "SETUP_FILE_OPERATION::default()".to_string())}
pub fn get_strange_SET_BOUNDS_RECT_FLAGS() -> (SET_BOUNDS_RECT_FLAGS, String) { (SET_BOUNDS_RECT_FLAGS::default(), "SET_BOUNDS_RECT_FLAGS::default()".to_string()) }
pub fn get_strange_SET_COMPOSITION_STRING_TYPE() -> (SET_COMPOSITION_STRING_TYPE, String) { (SET_COMPOSITION_STRING_TYPE::default(), "SET_COMPOSITION_STRING_TYPE::default()".to_string()) }
pub fn get_strange_SE_OBJECT_TYPE() -> (SE_OBJECT_TYPE, String) { (SE_OBJECT_TYPE::default(), "SE_OBJECT_TYPE::default()".to_string()) }
pub fn get_strange_SID_NAME_USE() -> (SID_NAME_USE, String) { (SID_NAME_USE::default(), "SID_NAME_USE::default()".to_string()) }
pub fn get_strange_SIGDN() -> (SIGDN, String) { (SIGDN::default(), "SIGDN::default()".to_string()) }
pub fn get_strange_SIP_ADD_NEWPROVIDER() -> (SIP_ADD_NEWPROVIDER, String) { (SIP_ADD_NEWPROVIDER::default(), "SIP_ADD_NEWPROVIDER::default()".to_string()) }
pub fn get_strange_SIP_CAP_SET_V3() -> (SIP_CAP_SET_V3, String) { (SIP_CAP_SET_V3::default(), "SIP_CAP_SET_V3::default()".to_string()) }
pub fn get_strange_SIP_DISPATCH_INFO() -> (SIP_DISPATCH_INFO, String) { (SIP_DISPATCH_INFO::default(), "SIP_DISPATCH_INFO::default()".to_string()) }
pub fn get_strange_SIP_INDIRECT_DATA() -> (SIP_INDIRECT_DATA, String) { (SIP_INDIRECT_DATA::default(), "SIP_INDIRECT_DATA::default()".to_string()) }
pub fn get_strange_SIP_SUBJECTINFO() -> (SIP_SUBJECTINFO, String) { (SIP_SUBJECTINFO::default(), "SIP_SUBJECTINFO::default()".to_string()) }
pub fn get_strange_SIZE() -> (SIZE, String) {    (SIZE::default(), "SIZE::default()".to_string())}
pub fn get_strange_SI_PAGE_TYPE() -> (SI_PAGE_TYPE, String) { (SI_PAGE_TYPE::default(), "SI_PAGE_TYPE::default()".to_string()) }
pub fn get_strange_SLIST_ENTRY() -> (SLIST_ENTRY, String) { (SLIST_ENTRY::default(), "SLIST_ENTRY::default()".to_string()) }
pub fn get_strange_SLIST_HEADER() -> (SLIST_HEADER, String) { (SLIST_HEADER::default(), "SLIST_HEADER::default()".to_string()) }
pub fn get_strange_SMALL_RECT() -> (SMALL_RECT, String) { (SMALL_RECT::default(), "SMALL_RECT::default()".to_string()) }
pub fn get_strange_SNMPAPI_CALLBACK() -> (SNMPAPI_CALLBACK, String) { (SNMPAPI_CALLBACK::default(), "SNMPAPI_CALLBACK::default()".to_string()) }
pub fn get_strange_SNMP_API_TRANSLATE_MODE() -> (SNMP_API_TRANSLATE_MODE, String) { (SNMP_API_TRANSLATE_MODE::default(), "SNMP_API_TRANSLATE_MODE::default()".to_string()) }
pub fn get_strange_SNMP_ERROR() -> (SNMP_ERROR, String) { (SNMP_ERROR::default(), "SNMP_ERROR::default()".to_string()) }
pub fn get_strange_SNMP_ERROR_STATUS() -> (SNMP_ERROR_STATUS, String) { (SNMP_ERROR_STATUS::default(), "SNMP_ERROR_STATUS::default()".to_string()) }
pub fn get_strange_SNMP_GENERICTRAP() -> (SNMP_GENERICTRAP, String) { (SNMP_GENERICTRAP::default(), "SNMP_GENERICTRAP::default()".to_string()) }
pub fn get_strange_SNMP_LOG() -> (SNMP_LOG, String) { (SNMP_LOG::default(), "SNMP_LOG::default()".to_string()) }
pub fn get_strange_SNMP_OUTPUT_LOG_TYPE() -> (SNMP_OUTPUT_LOG_TYPE, String) { (SNMP_OUTPUT_LOG_TYPE::default(), "SNMP_OUTPUT_LOG_TYPE::default()".to_string()) }
pub fn get_strange_SNMP_PDU_TYPE() -> (SNMP_PDU_TYPE, String) { (SNMP_PDU_TYPE::default(), "SNMP_PDU_TYPE::default()".to_string()) }
pub fn get_strange_SNMP_STATUS() -> (SNMP_STATUS, String) { (SNMP_STATUS::default(), "SNMP_STATUS::default()".to_string()) }
pub fn get_strange_SOCKADDR_INET() -> (SOCKADDR_INET, String) { (SOCKADDR_INET::default(), "SOCKADDR_INET::default()".to_string()) }
pub fn get_strange_SOCKET() -> (SOCKET, String) { (SOCKET::default(), "SOCKET::default()".to_string()) }
pub fn get_strange_SOFTDISTINFO() -> (SOFTDISTINFO, String) { (SOFTDISTINFO::default(), "SOFTDISTINFO::default()".to_string()) }
pub fn get_strange_SP_BACKUP_QUEUE_PARAMS_V2_A() -> (SP_BACKUP_QUEUE_PARAMS_V2_A, String) { (SP_BACKUP_QUEUE_PARAMS_V2_A::default(), "SP_BACKUP_QUEUE_PARAMS_V2_A::default()".to_string()) }
pub fn get_strange_SP_BACKUP_QUEUE_PARAMS_V2_W() -> (SP_BACKUP_QUEUE_PARAMS_V2_W, String) { (SP_BACKUP_QUEUE_PARAMS_V2_W::default(), "SP_BACKUP_QUEUE_PARAMS_V2_W::default()".to_string()) }
pub fn get_strange_SP_CLASSIMAGELIST_DATA() -> (SP_CLASSIMAGELIST_DATA, String) { (SP_CLASSIMAGELIST_DATA::default(), "SP_CLASSIMAGELIST_DATA::default()".to_string()) }
pub fn get_strange_SP_CLASSINSTALL_HEADER() -> (SP_CLASSINSTALL_HEADER, String) { (SP_CLASSINSTALL_HEADER::default(), "SP_CLASSINSTALL_HEADER::default()".to_string()) }
pub fn get_strange_SP_COPY_STYLE() -> (SP_COPY_STYLE, String) { (SP_COPY_STYLE::default(), "SP_COPY_STYLE::default()".to_string()) }
pub fn get_strange_SP_DEVICE_INTERFACE_DATA() -> (SP_DEVICE_INTERFACE_DATA, String) { (SP_DEVICE_INTERFACE_DATA::default(), "SP_DEVICE_INTERFACE_DATA::default()".to_string()) }
pub fn get_strange_SP_DEVICE_INTERFACE_DETAIL_DATA_A() -> (SP_DEVICE_INTERFACE_DETAIL_DATA_A, String) { (SP_DEVICE_INTERFACE_DETAIL_DATA_A::default(), "SP_DEVICE_INTERFACE_DETAIL_DATA_A::default()".to_string()) }
pub fn get_strange_SP_DEVICE_INTERFACE_DETAIL_DATA_W() -> (SP_DEVICE_INTERFACE_DETAIL_DATA_W, String) { (SP_DEVICE_INTERFACE_DETAIL_DATA_W::default(), "SP_DEVICE_INTERFACE_DETAIL_DATA_W::default()".to_string()) }
pub fn get_strange_SP_DEVINFO_DATA() -> (SP_DEVINFO_DATA, String) {    (SP_DEVINFO_DATA::default(), "SP_DEVINFO_DATA::default()".to_string())}
pub fn get_strange_SP_DEVINFO_LIST_DETAIL_DATA_A() -> (SP_DEVINFO_LIST_DETAIL_DATA_A, String) { (SP_DEVINFO_LIST_DETAIL_DATA_A::default(), "SP_DEVINFO_LIST_DETAIL_DATA_A::default()".to_string()) }
pub fn get_strange_SP_DEVINFO_LIST_DETAIL_DATA_W() -> (SP_DEVINFO_LIST_DETAIL_DATA_W, String) { (SP_DEVINFO_LIST_DETAIL_DATA_W::default(), "SP_DEVINFO_LIST_DETAIL_DATA_W::default()".to_string()) }
pub fn get_strange_SP_DEVINSTALL_PARAMS_A() -> (SP_DEVINSTALL_PARAMS_A, String) { (SP_DEVINSTALL_PARAMS_A::default(), "SP_DEVINSTALL_PARAMS_A::default()".to_string()) }
pub fn get_strange_SP_DEVINSTALL_PARAMS_W() -> (SP_DEVINSTALL_PARAMS_W, String) { (SP_DEVINSTALL_PARAMS_W::default(), "SP_DEVINSTALL_PARAMS_W::default()".to_string()) }
pub fn get_strange_SP_DRVINFO_DATA_V2_A() -> (SP_DRVINFO_DATA_V2_A, String) { (SP_DRVINFO_DATA_V2_A::default(), "SP_DRVINFO_DATA_V2_A::default()".to_string()) }
pub fn get_strange_SP_DRVINFO_DATA_V2_W() -> (SP_DRVINFO_DATA_V2_W, String) { (SP_DRVINFO_DATA_V2_W::default(), "SP_DRVINFO_DATA_V2_W::default()".to_string()) }
pub fn get_strange_SP_DRVINFO_DETAIL_DATA_A() -> (SP_DRVINFO_DETAIL_DATA_A, String) { (SP_DRVINFO_DETAIL_DATA_A::default(), "SP_DRVINFO_DETAIL_DATA_A::default()".to_string()) }
pub fn get_strange_SP_DRVINFO_DETAIL_DATA_W() -> (SP_DRVINFO_DETAIL_DATA_W, String) { (SP_DRVINFO_DETAIL_DATA_W::default(), "SP_DRVINFO_DETAIL_DATA_W::default()".to_string()) }
pub fn get_strange_SP_DRVINSTALL_PARAMS() -> (SP_DRVINSTALL_PARAMS, String) { (SP_DRVINSTALL_PARAMS::default(), "SP_DRVINSTALL_PARAMS::default()".to_string()) }
pub fn get_strange_SP_INF_INFORMATION() -> (SP_INF_INFORMATION, String) { (SP_INF_INFORMATION::default(), "SP_INF_INFORMATION::default()".to_string()) }
pub fn get_strange_SP_INF_SIGNER_INFO_V2_A() -> (SP_INF_SIGNER_INFO_V2_A, String) { (SP_INF_SIGNER_INFO_V2_A::default(), "SP_INF_SIGNER_INFO_V2_A::default()".to_string()) }
pub fn get_strange_SP_INF_SIGNER_INFO_V2_W() -> (SP_INF_SIGNER_INFO_V2_W, String) { (SP_INF_SIGNER_INFO_V2_W::default(), "SP_INF_SIGNER_INFO_V2_W::default()".to_string()) }
pub fn get_strange_SP_ORIGINAL_FILE_INFO_A() -> (SP_ORIGINAL_FILE_INFO_A, String) { (SP_ORIGINAL_FILE_INFO_A::default(), "SP_ORIGINAL_FILE_INFO_A::default()".to_string()) }
pub fn get_strange_SP_ORIGINAL_FILE_INFO_W() -> (SP_ORIGINAL_FILE_INFO_W, String) { (SP_ORIGINAL_FILE_INFO_W::default(), "SP_ORIGINAL_FILE_INFO_W::default()".to_string()) }
pub fn get_strange_STARTUPINFOA() -> (STARTUPINFOA, String) { (STARTUPINFOA::default(), "STARTUPINFOA::default()".to_string()) }
pub fn get_strange_STARTUPINFOW() -> (STARTUPINFOW, String) { (STARTUPINFOW::default(), "STARTUPINFOW::default()".to_string()) }
pub fn get_strange_STATEMGRSTATUS() -> (STATEMGRSTATUS, String) { (STATEMGRSTATUS::default(), "STATEMGRSTATUS::default()".to_string()) }
pub fn get_strange_STD_HANDLE() -> (STD_HANDLE, String) { (STD_HANDLE::default(), "STD_HANDLE::default()".to_string()) }
pub fn get_strange_STGFMT() -> (STGFMT, String) { (STGFMT::default(), "STGFMT::default()".to_string()) }
pub fn get_strange_STGM() -> (STGM, String) { (STGM::default(), "STGM::default()".to_string()) }
pub fn get_strange_STGMEDIUM() -> (STGMEDIUM, String) { (STGMEDIUM::default(), "STGMEDIUM::default()".to_string()) }
pub fn get_strange_STGOPTIONS() -> (STGOPTIONS, String) { (STGOPTIONS::default(), "STGOPTIONS::default()".to_string()) }
pub fn get_strange_STORAGE_DEPENDENCY_INFO() -> (STORAGE_DEPENDENCY_INFO, String) { (STORAGE_DEPENDENCY_INFO::default(), "STORAGE_DEPENDENCY_INFO::default()".to_string()) }
pub fn get_strange_STRETCH_BLT_MODE() -> (STRETCH_BLT_MODE, String) { (STRETCH_BLT_MODE::default(), "STRETCH_BLT_MODE::default()".to_string()) }
pub fn get_strange_STROBJ() -> (STROBJ, String) { (STROBJ::default(), "STROBJ::default()".to_string()) }
pub fn get_strange_STRRET() -> (STRRET, String) { (STRRET::default(), "STRRET::default()".to_string()) }
pub fn get_strange_STYLEBUFA() -> (STYLEBUFA, String) { (STYLEBUFA::default(), "STYLEBUFA::default()".to_string()) }
pub fn get_strange_STYLEBUFW() -> (STYLEBUFW, String) { (STYLEBUFW::default(), "STYLEBUFW::default()".to_string()) }
pub fn get_strange_SUBCLASSPROC() -> (SUBCLASSPROC, String) { (SUBCLASSPROC::default(), "SUBCLASSPROC::default()".to_string()) }
pub fn get_strange_SURFOBJ() -> (SURFOBJ, String) {    (SURFOBJ::default(), "SURFOBJ::default()".to_string())}
pub fn get_strange_SW_DEVICE_CREATE_CALLBACK() -> (SW_DEVICE_CREATE_CALLBACK, String) { (SW_DEVICE_CREATE_CALLBACK::default(), "SW_DEVICE_CREATE_CALLBACK::default()".to_string()) }
pub fn get_strange_SW_DEVICE_LIFETIME() -> (SW_DEVICE_LIFETIME, String) { (SW_DEVICE_LIFETIME::default(), "SW_DEVICE_LIFETIME::default()".to_string()) }
pub fn get_strange_SYSKIND() -> (SYSKIND, String) { (SYSKIND::default(), "SYSKIND::default()".to_string()) }
pub fn get_strange_SYSTEMTIME() -> (SYSTEMTIME, String) { (SYSTEMTIME::default(), "SYSTEMTIME::default()".to_string()) }
pub fn get_strange_SYSTEM_PALETTE_USE() -> (SYSTEM_PALETTE_USE, String) { (SYSTEM_PALETTE_USE::default(), "SYSTEM_PALETTE_USE::default()".to_string()) }
pub fn get_strange_SYSTEM_POWER_CAPABILITIES() -> (SYSTEM_POWER_CAPABILITIES, String) { (SYSTEM_POWER_CAPABILITIES::default(), "SYSTEM_POWER_CAPABILITIES::default()".to_string()) }
pub fn get_strange_SYSTEM_POWER_STATUS() -> (SYSTEM_POWER_STATUS, String) { (SYSTEM_POWER_STATUS::default(), "SYSTEM_POWER_STATUS::default()".to_string()) }
pub fn get_strange_ScrollAmount() -> (ScrollAmount, String) { (ScrollAmount::default(), "ScrollAmount::default()".to_string()) }
pub fn get_strange_SecHandle() -> (SecHandle, String) { (SecHandle::default(), "SecHandle::default()".to_string()) }
pub fn get_strange_ServerInformation() -> (ServerInformation, String) { (ServerInformation::default(), "ServerInformation::default()".to_string()) }
pub fn get_strange_SetupFileLogInfo() -> (SetupFileLogInfo, String) { (SetupFileLogInfo::default(), "SetupFileLogInfo::default()".to_string()) }
pub fn get_strange_SnmpVarBind() -> (SnmpVarBind, String) { (SnmpVarBind::default(), "SnmpVarBind::default()".to_string()) }
pub fn get_strange_SnmpVarBindList() -> (SnmpVarBindList, String) { (SnmpVarBindList::default(), "SnmpVarBindList::default()".to_string()) }
pub fn get_strange_StructureChangeType() -> (StructureChangeType, String) { (StructureChangeType::default(), "StructureChangeType::default()".to_string()) }
pub fn get_strange_SupportedTextSelection() -> (SupportedTextSelection, String) { (SupportedTextSelection::default(), "SupportedTextSelection::default()".to_string()) }
pub fn get_strange_SynchronizedInputType() -> (SynchronizedInputType, String) { (SynchronizedInputType::default(), "SynchronizedInputType::default()".to_string()) }
pub fn get_strange_TAKE_SNAPSHOT_VHDSET_FLAG() -> (TAKE_SNAPSHOT_VHDSET_FLAG, String) { (TAKE_SNAPSHOT_VHDSET_FLAG::default(), "TAKE_SNAPSHOT_VHDSET_FLAG::default()".to_string()) }
pub fn get_strange_TAP_PARAMETER() -> (TAP_PARAMETER, String) { (TAP_PARAMETER::default(), "TAP_PARAMETER::default()".to_string()) }
pub fn get_strange_TASKDIALOG_COMMON_BUTTON_FLAGS() -> (TASKDIALOG_COMMON_BUTTON_FLAGS, String) { (TASKDIALOG_COMMON_BUTTON_FLAGS::default(), "TASKDIALOG_COMMON_BUTTON_FLAGS::default()".to_string()) }
pub fn get_strange_TA_PROPERTY() -> (TA_PROPERTY, String) { (TA_PROPERTY::default(), "TA_PROPERTY::default()".to_string()) }
pub fn get_strange_TA_TIMINGFUNCTION() -> (TA_TIMINGFUNCTION, String) { (TA_TIMINGFUNCTION::default(), "TA_TIMINGFUNCTION::default()".to_string()) }
pub fn get_strange_TA_TRANSFORM() -> (TA_TRANSFORM, String) { (TA_TRANSFORM::default(), "TA_TRANSFORM::default()".to_string()) }
pub fn get_strange_TBBUTTON() -> (TBBUTTON, String) { (TBBUTTON::default(), "TBBUTTON::default()".to_string()) }
pub fn get_strange_TBS_COMMAND_LOCALITY() -> (TBS_COMMAND_LOCALITY, String) { (TBS_COMMAND_LOCALITY::default(), "TBS_COMMAND_LOCALITY::default()".to_string()) }
pub fn get_strange_TBS_COMMAND_PRIORITY() -> (TBS_COMMAND_PRIORITY, String) { (TBS_COMMAND_PRIORITY::default(), "TBS_COMMAND_PRIORITY::default()".to_string()) }
pub fn get_strange_TCPIP_OWNER_MODULE_INFO_CLASS() -> (TCPIP_OWNER_MODULE_INFO_CLASS, String) { (TCPIP_OWNER_MODULE_INFO_CLASS::default(), "TCPIP_OWNER_MODULE_INFO_CLASS::default()".to_string()) }
pub fn get_strange_TCP_ESTATS_TYPE() -> (TCP_ESTATS_TYPE, String) { (TCP_ESTATS_TYPE::default(), "TCP_ESTATS_TYPE::default()".to_string()) }
pub fn get_strange_TCP_TABLE_CLASS() -> (TCP_TABLE_CLASS, String) { (TCP_TABLE_CLASS::default(), "TCP_TABLE_CLASS::default()".to_string()) }
pub fn get_strange_TC_IFC_DESCRIPTOR() -> (TC_IFC_DESCRIPTOR, String) { (TC_IFC_DESCRIPTOR::default(), "TC_IFC_DESCRIPTOR::default()".to_string()) }
pub fn get_strange_TDH_CONTEXT() -> (TDH_CONTEXT, String) { (TDH_CONTEXT::default(), "TDH_CONTEXT::default()".to_string()) }
pub fn get_strange_TDH_HANDLE() -> (TDH_HANDLE, String) { (TDH_HANDLE::default(), "TDH_HANDLE::default()".to_string()) }
pub fn get_strange_TEXTMETRICA() -> (TEXTMETRICA, String) { (TEXTMETRICA::default(), "TEXTMETRICA::default()".to_string()) }
pub fn get_strange_TEXTMETRICW() -> (TEXTMETRICW, String) { (TEXTMETRICW::default(), "TEXTMETRICW::default()".to_string()) }
pub fn get_strange_TEXT_ALIGN_OPTIONS() -> (TEXT_ALIGN_OPTIONS, String) { (TEXT_ALIGN_OPTIONS::default(), "TEXT_ALIGN_OPTIONS::default()".to_string()) }
pub fn get_strange_THEMESIZE() -> (THEMESIZE, String) { (THEMESIZE::default(), "THEMESIZE::default()".to_string()) }
pub fn get_strange_THEME_PROPERTY_SYMBOL_ID() -> (THEME_PROPERTY_SYMBOL_ID, String) { (THEME_PROPERTY_SYMBOL_ID::default(), "THEME_PROPERTY_SYMBOL_ID::default()".to_string()) }
pub fn get_strange_THREADENTRY32() -> (THREADENTRY32, String) { (THREADENTRY32::default(), "THREADENTRY32::default()".to_string()) }
pub fn get_strange_THREADINFOCLASS() -> (THREADINFOCLASS, String) { (THREADINFOCLASS::default(), "THREADINFOCLASS::default()".to_string()) }
pub fn get_strange_THREAD_ACCESS_RIGHTS() -> (THREAD_ACCESS_RIGHTS, String) { (THREAD_ACCESS_RIGHTS::default(), "THREAD_ACCESS_RIGHTS::default()".to_string()) }
pub fn get_strange_THREAD_CREATION_FLAGS() -> (THREAD_CREATION_FLAGS, String) { (THREAD_CREATION_FLAGS::default(), "THREAD_CREATION_FLAGS::default()".to_string()) }
pub fn get_strange_THREAD_ERROR_MODE() -> (THREAD_ERROR_MODE, String) { (THREAD_ERROR_MODE::default(), "THREAD_ERROR_MODE::default()".to_string()) }
pub fn get_strange_THREAD_INFORMATION_CLASS() -> (THREAD_INFORMATION_CLASS, String) { (THREAD_INFORMATION_CLASS::default(), "THREAD_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_THREAD_PRIORITY() -> (THREAD_PRIORITY, String) { (THREAD_PRIORITY::default(), "THREAD_PRIORITY::default()".to_string()) }
pub fn get_strange_TIMECAPS() -> (TIMECAPS, String) { (TIMECAPS::default(), "TIMECAPS::default()".to_string()) }
pub fn get_strange_TIME_ZONE_INFORMATION() -> (TIME_ZONE_INFORMATION, String) { (TIME_ZONE_INFORMATION::default(), "TIME_ZONE_INFORMATION::default()".to_string()) }
pub fn get_strange_TOKEN_ACCESS_MASK() -> (TOKEN_ACCESS_MASK, String) { (TOKEN_ACCESS_MASK::default(), "TOKEN_ACCESS_MASK::default()".to_string()) }
pub fn get_strange_TOKEN_GROUPS() -> (TOKEN_GROUPS, String) { (TOKEN_GROUPS::default(), "TOKEN_GROUPS::default()".to_string()) }
pub fn get_strange_TOKEN_INFORMATION_CLASS() -> (TOKEN_INFORMATION_CLASS, String) { (TOKEN_INFORMATION_CLASS::default(), "TOKEN_INFORMATION_CLASS::default()".to_string()) }
pub fn get_strange_TOKEN_PRIVILEGES() -> (TOKEN_PRIVILEGES, String) { (TOKEN_PRIVILEGES::default(), "TOKEN_PRIVILEGES::default()".to_string()) }
pub fn get_strange_TOKEN_TYPE() -> (TOKEN_TYPE, String) { (TOKEN_TYPE::default(), "TOKEN_TYPE::default()".to_string()) }
pub fn get_strange_TOUCHINPUT() -> (TOUCHINPUT, String) { (TOUCHINPUT::default(), "TOUCHINPUT::default()".to_string()) }
pub fn get_strange_TOUCH_FEEDBACK_MODE() -> (TOUCH_FEEDBACK_MODE, String) { (TOUCH_FEEDBACK_MODE::default(), "TOUCH_FEEDBACK_MODE::default()".to_string()) }
pub fn get_strange_TOUCH_HIT_TESTING_PROXIMITY_EVALUATION() -> (TOUCH_HIT_TESTING_PROXIMITY_EVALUATION, String) { (TOUCH_HIT_TESTING_PROXIMITY_EVALUATION::default(), "TOUCH_HIT_TESTING_PROXIMITY_EVALUATION::default()".to_string()) }
pub fn get_strange_TP_POOL_STACK_INFORMATION() -> (TP_POOL_STACK_INFORMATION, String) { (TP_POOL_STACK_INFORMATION::default(), "TP_POOL_STACK_INFORMATION::default()".to_string()) }
pub fn get_strange_TRACE_EVENT_INFO() -> (TRACE_EVENT_INFO, String) { (TRACE_EVENT_INFO::default(), "TRACE_EVENT_INFO::default()".to_string()) }
pub fn get_strange_TRACE_MESSAGE_FLAGS() -> (TRACE_MESSAGE_FLAGS, String) { (TRACE_MESSAGE_FLAGS::default(), "TRACE_MESSAGE_FLAGS::default()".to_string()) }
pub fn get_strange_TRACE_QUERY_INFO_CLASS() -> (TRACE_QUERY_INFO_CLASS, String) { (TRACE_QUERY_INFO_CLASS::default(), "TRACE_QUERY_INFO_CLASS::default()".to_string()) }
pub fn get_strange_TRACKMOUSEEVENT() -> (TRACKMOUSEEVENT, String) { (TRACKMOUSEEVENT::default(), "TRACKMOUSEEVENT::default()".to_string()) }
pub fn get_strange_TRANSLATION_PARAMETER() -> (TRANSLATION_PARAMETER, String) { (TRANSLATION_PARAMETER::default(), "TRANSLATION_PARAMETER::default()".to_string()) }
pub fn get_strange_TREE_SEC_INFO() -> (TREE_SEC_INFO, String) { (TREE_SEC_INFO::default(), "TREE_SEC_INFO::default()".to_string()) }
pub fn get_strange_TRIVERTEX() -> (TRIVERTEX, String) { (TRIVERTEX::default(), "TRIVERTEX::default()".to_string()) }
pub fn get_strange_TRUSTEE_A() -> (TRUSTEE_A, String) { (TRUSTEE_A::default(), "TRUSTEE_A::default()".to_string()) }
pub fn get_strange_TRUSTEE_W() -> (TRUSTEE_W, String) { (TRUSTEE_W::default(), "TRUSTEE_W::default()".to_string()) }
pub fn get_strange_TTEMBED_FLAGS() -> (TTEMBED_FLAGS, String) { (TTEMBED_FLAGS::default(), "TTEMBED_FLAGS::default()".to_string()) }
pub fn get_strange_TTLOAD_EMBEDDED_FONT_STATUS() -> (TTLOAD_EMBEDDED_FONT_STATUS, String) { (TTLOAD_EMBEDDED_FONT_STATUS::default(), "TTLOAD_EMBEDDED_FONT_STATUS::default()".to_string()) }
pub fn get_strange_TextEditChangeType() -> (TextEditChangeType, String) { (TextEditChangeType::default(), "TextEditChangeType::default()".to_string()) }
pub fn get_strange_TextPatternRangeEndpoint() -> (TextPatternRangeEndpoint, String) { (TextPatternRangeEndpoint::default(), "TextPatternRangeEndpoint::default()".to_string()) }
pub fn get_strange_TextUnit() -> (TextUnit, String) { (TextUnit::default(), "TextUnit::default()".to_string()) }
pub fn get_strange_TreeScope() -> (TreeScope, String) { (TreeScope::default(), "TreeScope::default()".to_string()) }
pub fn get_strange_UDATE() -> (UDATE, String) { (UDATE::default(), "UDATE::default()".to_string()) }
pub fn get_strange_UDP_TABLE_CLASS() -> (UDP_TABLE_CLASS, String) { (UDP_TABLE_CLASS::default(), "UDP_TABLE_CLASS::default()".to_string()) }
pub fn get_strange_UMS_SYSTEM_THREAD_INFORMATION() -> (UMS_SYSTEM_THREAD_INFORMATION, String) { (UMS_SYSTEM_THREAD_INFORMATION::default(), "UMS_SYSTEM_THREAD_INFORMATION::default()".to_string()) }
pub fn get_strange_UNICODE_STRING() -> (UNICODE_STRING, String) { (UNICODE_STRING::default(), "UNICODE_STRING::default()".to_string()) }
pub fn get_strange_URI_CREATE_FLAGS() -> (URI_CREATE_FLAGS, String) { (URI_CREATE_FLAGS::default(), "URI_CREATE_FLAGS::default()".to_string()) }
pub fn get_strange_URLCACHE_ENTRY_INFO() -> (URLCACHE_ENTRY_INFO, String) { (URLCACHE_ENTRY_INFO::default(), "URLCACHE_ENTRY_INFO::default()".to_string()) }
pub fn get_strange_URLIS() -> (URLIS, String) { (URLIS::default(), "URLIS::default()".to_string()) }
pub fn get_strange_URL_CACHE_LIMIT_TYPE() -> (URL_CACHE_LIMIT_TYPE, String) { (URL_CACHE_LIMIT_TYPE::default(), "URL_CACHE_LIMIT_TYPE::default()".to_string()) }
pub fn get_strange_URL_COMPONENTS() -> (URL_COMPONENTS, String) { (URL_COMPONENTS::default(), "URL_COMPONENTS::default()".to_string()) }
pub fn get_strange_URL_COMPONENTSA() -> (URL_COMPONENTSA, String) { (URL_COMPONENTSA::default(), "URL_COMPONENTSA::default()".to_string()) }
pub fn get_strange_URL_COMPONENTSW() -> (URL_COMPONENTSW, String) { (URL_COMPONENTSW::default(), "URL_COMPONENTSW::default()".to_string()) }
pub fn get_strange_USAGE_AND_PAGE() -> (USAGE_AND_PAGE, String) { (USAGE_AND_PAGE::default(), "USAGE_AND_PAGE::default()".to_string()) }
pub fn get_strange_USBD_ISO_PACKET_DESCRIPTOR() -> (USBD_ISO_PACKET_DESCRIPTOR, String) { (USBD_ISO_PACKET_DESCRIPTOR::default(), "USBD_ISO_PACKET_DESCRIPTOR::default()".to_string()) }
pub fn get_strange_USB_INTERFACE_DESCRIPTOR() -> (USB_INTERFACE_DESCRIPTOR, String) { (USB_INTERFACE_DESCRIPTOR::default(), "USB_INTERFACE_DESCRIPTOR::default()".to_string()) }
pub fn get_strange_USER_OBJECT_INFORMATION_INDEX() -> (USER_OBJECT_INFORMATION_INDEX, String) { (USER_OBJECT_INFORMATION_INDEX::default(), "USER_OBJECT_INFORMATION_INDEX::default()".to_string()) }
pub fn get_strange_UiaCacheRequest() -> (UiaCacheRequest, String) { (UiaCacheRequest::default(), "UiaCacheRequest::default()".to_string()) }
pub fn get_strange_UiaChangeInfo() -> (UiaChangeInfo, String) { (UiaChangeInfo::default(), "UiaChangeInfo::default()".to_string()) }
pub fn get_strange_UiaCondition() -> (UiaCondition, String) { (UiaCondition::default(), "UiaCondition::default()".to_string()) }
pub fn get_strange_UiaEventCallback() -> (UiaEventCallback, String) { (UiaEventCallback::default(), "UiaEventCallback::default()".to_string()) }
pub fn get_strange_UiaFindParams() -> (UiaFindParams, String) { (UiaFindParams::default(), "UiaFindParams::default()".to_string()) }
pub fn get_strange_UiaPoint() -> (UiaPoint, String) { (UiaPoint::default(), "UiaPoint::default()".to_string()) }
pub fn get_strange_UiaProviderCallback() -> (UiaProviderCallback, String) { (UiaProviderCallback::default(), "UiaProviderCallback::default()".to_string()) }
pub fn get_strange_VALENTA() -> (VALENTA, String) { (VALENTA::default(), "VALENTA::default()".to_string()) }
pub fn get_strange_VALENTW() -> (VALENTW, String) { (VALENTW::default(), "VALENTW::default()".to_string()) }
pub fn get_strange_VARIANT() -> (VARIANT, String) { (VARIANT::default(), "VARIANT::default()".to_string()) }
pub fn get_strange_VARSTRING() -> (VARSTRING, String) {    (VARSTRING::default(), "VARSTRING::default()".to_string())}
pub fn get_strange_VERIFIER_ENUM_RESOURCE_FLAGS() -> (VERIFIER_ENUM_RESOURCE_FLAGS, String) { (VERIFIER_ENUM_RESOURCE_FLAGS::default(), "VERIFIER_ENUM_RESOURCE_FLAGS::default()".to_string()) }
pub fn get_strange_VIRTUAL_DISK_ACCESS_MASK() -> (VIRTUAL_DISK_ACCESS_MASK, String) { (VIRTUAL_DISK_ACCESS_MASK::default(), "VIRTUAL_DISK_ACCESS_MASK::default()".to_string()) }
pub fn get_strange_VIRTUAL_DISK_PROGRESS() -> (VIRTUAL_DISK_PROGRESS, String) { (VIRTUAL_DISK_PROGRESS::default(), "VIRTUAL_DISK_PROGRESS::default()".to_string()) }
pub fn get_strange_WAITORTIMERCALLBACK() -> (WAITORTIMERCALLBACK, String) { (WAITORTIMERCALLBACK::default(), "WAITORTIMERCALLBACK::default()".to_string()) }
pub fn get_strange_WAVEFILTER() -> (WAVEFILTER, String) { (WAVEFILTER::default(), "WAVEFILTER::default()".to_string()) }
pub fn get_strange_WAVEFORMATEX() -> (WAVEFORMATEX, String) { (WAVEFORMATEX::default(), "WAVEFORMATEX::default()".to_string()) }
pub fn get_strange_WAVEHDR() -> (WAVEHDR, String) { (WAVEHDR::default(), "WAVEHDR::default()".to_string()) }
pub fn get_strange_WAVEINCAPSA() -> (WAVEINCAPSA, String) { (WAVEINCAPSA::default(), "WAVEINCAPSA::default()".to_string()) }
pub fn get_strange_WAVEINCAPSW() -> (WAVEINCAPSW, String) { (WAVEINCAPSW::default(), "WAVEINCAPSW::default()".to_string()) }
pub fn get_strange_WAVEOUTCAPSA() -> (WAVEOUTCAPSA, String) { (WAVEOUTCAPSA::default(), "WAVEOUTCAPSA::default()".to_string()) }
pub fn get_strange_WAVEOUTCAPSW() -> (WAVEOUTCAPSW, String) { (WAVEOUTCAPSW::default(), "WAVEOUTCAPSW::default()".to_string()) }
pub fn get_strange_WEB_SOCKET_ACTION() -> (WEB_SOCKET_ACTION, String) { (WEB_SOCKET_ACTION::default(), "WEB_SOCKET_ACTION::default()".to_string()) }
pub fn get_strange_WEB_SOCKET_ACTION_QUEUE() -> (WEB_SOCKET_ACTION_QUEUE, String) { (WEB_SOCKET_ACTION_QUEUE::default(), "WEB_SOCKET_ACTION_QUEUE::default()".to_string()) }
pub fn get_strange_WEB_SOCKET_BUFFER() -> (WEB_SOCKET_BUFFER, String) { (WEB_SOCKET_BUFFER::default(), "WEB_SOCKET_BUFFER::default()".to_string()) }
pub fn get_strange_WEB_SOCKET_BUFFER_TYPE() -> (WEB_SOCKET_BUFFER_TYPE, String) { (WEB_SOCKET_BUFFER_TYPE::default(), "WEB_SOCKET_BUFFER_TYPE::default()".to_string()) }
pub fn get_strange_WEB_SOCKET_HANDLE() -> (WEB_SOCKET_HANDLE, String) { (WEB_SOCKET_HANDLE::default(), "WEB_SOCKET_HANDLE::default()".to_string()) }
pub fn get_strange_WEB_SOCKET_PROPERTY_TYPE() -> (WEB_SOCKET_PROPERTY_TYPE, String) { (WEB_SOCKET_PROPERTY_TYPE::default(), "WEB_SOCKET_PROPERTY_TYPE::default()".to_string()) }
pub fn get_strange_WELL_KNOWN_SID_TYPE() -> (WELL_KNOWN_SID_TYPE, String) { (WELL_KNOWN_SID_TYPE::default(), "WELL_KNOWN_SID_TYPE::default()".to_string()) }
pub fn get_strange_WER_CONSENT() -> (WER_CONSENT, String) { (WER_CONSENT::default(), "WER_CONSENT::default()".to_string()) }
pub fn get_strange_WER_DUMP_TYPE() -> (WER_DUMP_TYPE, String) { (WER_DUMP_TYPE::default(), "WER_DUMP_TYPE::default()".to_string()) }
pub fn get_strange_WER_FAULT_REPORTING() -> (WER_FAULT_REPORTING, String) { (WER_FAULT_REPORTING::default(), "WER_FAULT_REPORTING::default()".to_string()) }
pub fn get_strange_WER_FILE() -> (WER_FILE, String) { (WER_FILE::default(), "WER_FILE::default()".to_string()) }
pub fn get_strange_WER_FILE_TYPE() -> (WER_FILE_TYPE, String) { (WER_FILE_TYPE::default(), "WER_FILE_TYPE::default()".to_string()) }
pub fn get_strange_WER_REGISTER_FILE_TYPE() -> (WER_REGISTER_FILE_TYPE, String) { (WER_REGISTER_FILE_TYPE::default(), "WER_REGISTER_FILE_TYPE::default()".to_string()) }
pub fn get_strange_WER_REPORT_METADATA_V1() -> (WER_REPORT_METADATA_V1, String) { (WER_REPORT_METADATA_V1::default(), "WER_REPORT_METADATA_V1::default()".to_string()) }
pub fn get_strange_WER_REPORT_METADATA_V2() -> (WER_REPORT_METADATA_V2, String) { (WER_REPORT_METADATA_V2::default(), "WER_REPORT_METADATA_V2::default()".to_string()) }
pub fn get_strange_WER_REPORT_METADATA_V3() -> (WER_REPORT_METADATA_V3, String) { (WER_REPORT_METADATA_V3::default(), "WER_REPORT_METADATA_V3::default()".to_string()) }
pub fn get_strange_WER_REPORT_TYPE() -> (WER_REPORT_TYPE, String) { (WER_REPORT_TYPE::default(), "WER_REPORT_TYPE::default()".to_string()) }
pub fn get_strange_WER_REPORT_UI() -> (WER_REPORT_UI, String) { (WER_REPORT_UI::default(), "WER_REPORT_UI::default()".to_string()) }
pub fn get_strange_WER_SUBMIT_FLAGS() -> (WER_SUBMIT_FLAGS, String) { (WER_SUBMIT_FLAGS::default(), "WER_SUBMIT_FLAGS::default()".to_string()) }
pub fn get_strange_WER_SUBMIT_RESULT() -> (WER_SUBMIT_RESULT, String) { (WER_SUBMIT_RESULT::default(), "WER_SUBMIT_RESULT::default()".to_string()) }
pub fn get_strange_WFD_OPEN_SESSION_COMPLETE_CALLBACK() -> (WFD_OPEN_SESSION_COMPLETE_CALLBACK, String) { (WFD_OPEN_SESSION_COMPLETE_CALLBACK::default(), "WFD_OPEN_SESSION_COMPLETE_CALLBACK::default()".to_string()) }
pub fn get_strange_WICSectionAccessLevel() -> (WICSectionAccessLevel, String) { (WICSectionAccessLevel::default(), "WICSectionAccessLevel::default()".to_string()) }
pub fn get_strange_WIN32_ERROR() -> (WIN32_ERROR, String) { (WIN32_ERROR::default(), "WIN32_ERROR::default()".to_string()) }
pub fn get_strange_WIN32_FIND_DATAA() -> (WIN32_FIND_DATAA, String) { (WIN32_FIND_DATAA::default(), "WIN32_FIND_DATAA::default()".to_string()) }
pub fn get_strange_WIN32_FIND_DATAW() -> (WIN32_FIND_DATAW, String) { (WIN32_FIND_DATAW::default(), "WIN32_FIND_DATAW::default()".to_string()) }
pub fn get_strange_WINDOWTHEMEATTRIBUTETYPE() -> (WINDOWTHEMEATTRIBUTETYPE, String) { (WINDOWTHEMEATTRIBUTETYPE::default(), "WINDOWTHEMEATTRIBUTETYPE::default()".to_string()) }
pub fn get_strange_WINDOW_EX_STYLE() -> (WINDOW_EX_STYLE, String) { (WINDOW_EX_STYLE::default(), "WINDOW_EX_STYLE::default()".to_string()) }
pub fn get_strange_WINDOW_LONG_PTR_INDEX() -> (WINDOW_LONG_PTR_INDEX, String) {    (WINDOW_LONG_PTR_INDEX::default(), "WINDOW_LONG_PTR_INDEX::default()".to_string())}
pub fn get_strange_WINDOW_STYLE() -> (WINDOW_STYLE, String) { (WINDOW_STYLE::default(), "WINDOW_STYLE::default()".to_string()) }
pub fn get_strange_WINEVENTPROC() -> (WINEVENTPROC, String) { (WINEVENTPROC::default(), "WINEVENTPROC::default()".to_string()) }
pub fn get_strange_WINHTTP_ACCESS_TYPE() -> (WINHTTP_ACCESS_TYPE, String) { (WINHTTP_ACCESS_TYPE::default(), "WINHTTP_ACCESS_TYPE::default()".to_string()) }
pub fn get_strange_WINHTTP_AUTOPROXY_OPTIONS() -> (WINHTTP_AUTOPROXY_OPTIONS, String) { (WINHTTP_AUTOPROXY_OPTIONS::default(), "WINHTTP_AUTOPROXY_OPTIONS::default()".to_string()) }
pub fn get_strange_WINHTTP_CURRENT_USER_IE_PROXY_CONFIG() -> (WINHTTP_CURRENT_USER_IE_PROXY_CONFIG, String) { (WINHTTP_CURRENT_USER_IE_PROXY_CONFIG::default(), "WINHTTP_CURRENT_USER_IE_PROXY_CONFIG::default()".to_string()) }
pub fn get_strange_WINHTTP_OPEN_REQUEST_FLAGS() -> (WINHTTP_OPEN_REQUEST_FLAGS, String) { (WINHTTP_OPEN_REQUEST_FLAGS::default(), "WINHTTP_OPEN_REQUEST_FLAGS::default()".to_string()) }
pub fn get_strange_WINHTTP_PROXY_INFO() -> (WINHTTP_PROXY_INFO, String) { (WINHTTP_PROXY_INFO::default(), "WINHTTP_PROXY_INFO::default()".to_string()) }
pub fn get_strange_WINHTTP_PROXY_RESULT() -> (WINHTTP_PROXY_RESULT, String) { (WINHTTP_PROXY_RESULT::default(), "WINHTTP_PROXY_RESULT::default()".to_string()) }
pub fn get_strange_WINHTTP_PROXY_RESULT_EX() -> (WINHTTP_PROXY_RESULT_EX, String) { (WINHTTP_PROXY_RESULT_EX::default(), "WINHTTP_PROXY_RESULT_EX::default()".to_string()) }
pub fn get_strange_WINHTTP_PROXY_SETTINGS() -> (WINHTTP_PROXY_SETTINGS, String) { (WINHTTP_PROXY_SETTINGS::default(), "WINHTTP_PROXY_SETTINGS::default()".to_string()) }
pub fn get_strange_WINHTTP_QUERY_CONNECTION_GROUP_RESULT() -> (WINHTTP_QUERY_CONNECTION_GROUP_RESULT, String) { (WINHTTP_QUERY_CONNECTION_GROUP_RESULT::default(), "WINHTTP_QUERY_CONNECTION_GROUP_RESULT::default()".to_string()) }
pub fn get_strange_WINHTTP_STATUS_CALLBACK() -> (WINHTTP_STATUS_CALLBACK, String) { (WINHTTP_STATUS_CALLBACK::default(), "WINHTTP_STATUS_CALLBACK::default()".to_string()) }
pub fn get_strange_WINHTTP_WEB_SOCKET_BUFFER_TYPE() -> (WINHTTP_WEB_SOCKET_BUFFER_TYPE, String) { (WINHTTP_WEB_SOCKET_BUFFER_TYPE::default(), "WINHTTP_WEB_SOCKET_BUFFER_TYPE::default()".to_string()) }
pub fn get_strange_WININET_PROXY_INFO_LIST() -> (WININET_PROXY_INFO_LIST, String) { (WININET_PROXY_INFO_LIST::default(), "WININET_PROXY_INFO_LIST::default()".to_string()) }
pub fn get_strange_WINSTAENUMPROCA() -> (WINSTAENUMPROCA, String) { (WINSTAENUMPROCA::default(), "WINSTAENUMPROCA::default()".to_string()) }
pub fn get_strange_WINSTAENUMPROCW() -> (WINSTAENUMPROCW, String) { (WINSTAENUMPROCW::default(), "WINSTAENUMPROCW::default()".to_string()) }
pub fn get_strange_WINTRUST_DATA() -> (WINTRUST_DATA, String) { (WINTRUST_DATA::default(), "WINTRUST_DATA::default()".to_string()) }
pub fn get_strange_WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION() -> (WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, String) { (WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION::default(), "WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION::default()".to_string()) }
pub fn get_strange_WINTRUST_POLICY_FLAGS() -> (WINTRUST_POLICY_FLAGS, String) { (WINTRUST_POLICY_FLAGS::default(), "WINTRUST_POLICY_FLAGS::default()".to_string()) }
pub fn get_strange_WINUSB_PIPE_INFORMATION() -> (WINUSB_PIPE_INFORMATION, String) { (WINUSB_PIPE_INFORMATION::default(), "WINUSB_PIPE_INFORMATION::default()".to_string()) }
pub fn get_strange_WINUSB_PIPE_INFORMATION_EX() -> (WINUSB_PIPE_INFORMATION_EX, String) { (WINUSB_PIPE_INFORMATION_EX::default(), "WINUSB_PIPE_INFORMATION_EX::default()".to_string()) }
pub fn get_strange_WINUSB_SETUP_PACKET() -> (WINUSB_SETUP_PACKET, String) { (WINUSB_SETUP_PACKET::default(), "WINUSB_SETUP_PACKET::default()".to_string()) }
pub fn get_strange_WIN_CERTIFICATE() -> (WIN_CERTIFICATE, String) { (WIN_CERTIFICATE::default(), "WIN_CERTIFICATE::default()".to_string()) }
pub fn get_strange_WIN_HTTP_CREATE_URL_FLAGS() -> (WIN_HTTP_CREATE_URL_FLAGS, String) { (WIN_HTTP_CREATE_URL_FLAGS::default(), "WIN_HTTP_CREATE_URL_FLAGS::default()".to_string()) }
pub fn get_strange_WLAN_AUTOCONF_OPCODE() -> (WLAN_AUTOCONF_OPCODE, String) { (WLAN_AUTOCONF_OPCODE::default(), "WLAN_AUTOCONF_OPCODE::default()".to_string()) }
pub fn get_strange_WLAN_FILTER_LIST_TYPE() -> (WLAN_FILTER_LIST_TYPE, String) { (WLAN_FILTER_LIST_TYPE::default(), "WLAN_FILTER_LIST_TYPE::default()".to_string()) }
pub fn get_strange_WLAN_HOSTED_NETWORK_OPCODE() -> (WLAN_HOSTED_NETWORK_OPCODE, String) { (WLAN_HOSTED_NETWORK_OPCODE::default(), "WLAN_HOSTED_NETWORK_OPCODE::default()".to_string()) }
pub fn get_strange_WLAN_HOSTED_NETWORK_REASON() -> (WLAN_HOSTED_NETWORK_REASON, String) { (WLAN_HOSTED_NETWORK_REASON::default(), "WLAN_HOSTED_NETWORK_REASON::default()".to_string()) }
pub fn get_strange_WLAN_IHV_CONTROL_TYPE() -> (WLAN_IHV_CONTROL_TYPE, String) { (WLAN_IHV_CONTROL_TYPE::default(), "WLAN_IHV_CONTROL_TYPE::default()".to_string()) }
pub fn get_strange_WLAN_INTF_OPCODE() -> (WLAN_INTF_OPCODE, String) { (WLAN_INTF_OPCODE::default(), "WLAN_INTF_OPCODE::default()".to_string()) }
pub fn get_strange_WLAN_NOTIFICATION_CALLBACK() -> (WLAN_NOTIFICATION_CALLBACK, String) { (WLAN_NOTIFICATION_CALLBACK::default(), "WLAN_NOTIFICATION_CALLBACK::default()".to_string()) }
pub fn get_strange_WLAN_OPCODE_VALUE_TYPE() -> (WLAN_OPCODE_VALUE_TYPE, String) { (WLAN_OPCODE_VALUE_TYPE::default(), "WLAN_OPCODE_VALUE_TYPE::default()".to_string()) }
pub fn get_strange_WLAN_SECURABLE_OBJECT() -> (WLAN_SECURABLE_OBJECT, String) { (WLAN_SECURABLE_OBJECT::default(), "WLAN_SECURABLE_OBJECT::default()".to_string()) }
pub fn get_strange_WLAN_SET_EAPHOST_FLAGS() -> (WLAN_SET_EAPHOST_FLAGS, String) { (WLAN_SET_EAPHOST_FLAGS::default(), "WLAN_SET_EAPHOST_FLAGS::default()".to_string()) }
pub fn get_strange_WL_DISPLAY_PAGES() -> (WL_DISPLAY_PAGES, String) { (WL_DISPLAY_PAGES::default(), "WL_DISPLAY_PAGES::default()".to_string()) }
pub fn get_strange_WMIDPREQUEST() -> (WMIDPREQUEST, String) { (WMIDPREQUEST::default(), "WMIDPREQUEST::default()".to_string()) }
pub fn get_strange_WNDENUMPROC() -> (WNDENUMPROC, String) { (WNDENUMPROC::default(), "WNDENUMPROC::default()".to_string()) }
pub fn get_strange_WORKER_THREAD_FLAGS() -> (WORKER_THREAD_FLAGS, String) { (WORKER_THREAD_FLAGS::default(), "WORKER_THREAD_FLAGS::default()".to_string()) }
pub fn get_strange_WPAD_CACHE_DELETE() -> (WPAD_CACHE_DELETE, String) { (WPAD_CACHE_DELETE::default(), "WPAD_CACHE_DELETE::default()".to_string()) }
pub fn get_strange_WPARAM() -> (WPARAM, String) {    (WPARAM::default(), "WPARAM::default()".to_string())}
pub fn get_strange_WRITEEMBEDPROC() -> (WRITEEMBEDPROC, String) { (WRITEEMBEDPROC::default(), "WRITEEMBEDPROC::default()".to_string()) }
pub fn get_strange_WSB_PROP() -> (WSB_PROP, String) { (WSB_PROP::default(), "WSB_PROP::default()".to_string()) }
pub fn get_strange_WSDXML_ELEMENT() -> (WSDXML_ELEMENT, String) { (WSDXML_ELEMENT::default(), "WSDXML_ELEMENT::default()".to_string()) }
pub fn get_strange_WSDXML_NAME() -> (WSDXML_NAME, String) { (WSDXML_NAME::default(), "WSDXML_NAME::default()".to_string()) }
pub fn get_strange_WSMAN_DATA() -> (WSMAN_DATA, String) { (WSMAN_DATA::default(), "WSMAN_DATA::default()".to_string()) }
pub fn get_strange_WSManSessionOption() -> (WSManSessionOption, String) { (WSManSessionOption::default(), "WSManSessionOption::default()".to_string()) }
pub fn get_strange_WTSLISTENERCONFIGA() -> (WTSLISTENERCONFIGA, String) { (WTSLISTENERCONFIGA::default(), "WTSLISTENERCONFIGA::default()".to_string()) }
pub fn get_strange_WTSLISTENERCONFIGW() -> (WTSLISTENERCONFIGW, String) { (WTSLISTENERCONFIGW::default(), "WTSLISTENERCONFIGW::default()".to_string()) }
pub fn get_strange_WTS_CONFIG_CLASS() -> (WTS_CONFIG_CLASS, String) { (WTS_CONFIG_CLASS::default(), "WTS_CONFIG_CLASS::default()".to_string()) }
pub fn get_strange_WTS_INFO_CLASS() -> (WTS_INFO_CLASS, String) { (WTS_INFO_CLASS::default(), "WTS_INFO_CLASS::default()".to_string()) }
pub fn get_strange_WTS_TYPE_CLASS() -> (WTS_TYPE_CLASS, String) { (WTS_TYPE_CLASS::default(), "WTS_TYPE_CLASS::default()".to_string()) }
pub fn get_strange_WTS_VIRTUAL_CLASS() -> (WTS_VIRTUAL_CLASS, String) { (WTS_VIRTUAL_CLASS::default(), "WTS_VIRTUAL_CLASS::default()".to_string()) }
pub fn get_strange_WindowVisualState() -> (WindowVisualState, String) { (WindowVisualState::default(), "WindowVisualState::default()".to_string()) }
pub fn get_strange_XFORM() -> (XFORM, String) { (XFORM::default(), "XFORM::default()".to_string()) }
pub fn get_strange_XFORML() -> (XFORML, String) { (XFORML::default(), "XFORML::default()".to_string()) }
pub fn get_strange_XFORMOBJ() -> (XFORMOBJ, String) { (XFORMOBJ::default(), "XFORMOBJ::default()".to_string()) }
pub fn get_strange_XLATEOBJ() -> (XLATEOBJ, String) { (XLATEOBJ::default(), "XLATEOBJ::default()".to_string()) }
pub fn get_strange_YIELDPROC() -> (YIELDPROC, String) { (YIELDPROC::default(), "YIELDPROC::default()".to_string()) }
pub fn get_strange_berelement() -> (berelement, String) { (berelement::default(), "berelement::default()".to_string()) }
pub fn get_strange_eAvrfResourceTypes() -> (eAvrfResourceTypes, String) { (eAvrfResourceTypes::default(), "eAvrfResourceTypes::default()".to_string()) }
pub fn get_strange_ldap() -> (ldap, String) {    (ldap::default(), "ldap::default()".to_string())}
pub fn get_strange_ldap_version_info() -> (ldap_version_info, String) { (ldap_version_info::default(), "ldap_version_info::default()".to_string()) }
pub fn get_strange_ldapcontrolA() -> (ldapcontrolA, String) { (ldapcontrolA::default(), "ldapcontrolA::default()".to_string()) }
pub fn get_strange_ldapcontrolW() -> (ldapcontrolW, String) { (ldapcontrolW::default(), "ldapcontrolW::default()".to_string()) }
pub fn get_strange_ldapvlvinfo() -> (ldapvlvinfo, String) { (ldapvlvinfo::default(), "ldapvlvinfo::default()".to_string()) }
pub fn get_strange_smiOCTETS() -> (smiOCTETS, String) { (smiOCTETS::default(), "smiOCTETS::default()".to_string()) }
pub fn get_strange_smiOID() -> (smiOID, String) { (smiOID::default(), "smiOID::default()".to_string()) }
pub fn get_strange_smiVALUE() -> (smiVALUE, String) { (smiVALUE::default(), "smiVALUE::default()".to_string()) }
pub fn get_strange_smiVENDORINFO() -> (smiVENDORINFO, String) { (smiVENDORINFO::default(), "smiVENDORINFO::default()".to_string()) }
pub fn get_strange_tACMFORMATDETAILSW() -> (tACMFORMATDETAILSW, String) { (tACMFORMATDETAILSW::default(), "tACMFORMATDETAILSW::default()".to_string()) }

// ([^\n]+)
// pub fn get_strange_$1\(\) -> \($1, String\) { \($1::default\(\), "$1::default\(\)".to_string\(\)\) }
// pub fn get_strange_ACL() -> (ACL, String) { (ACL::default(), "ACL::default()".to_string()) }
