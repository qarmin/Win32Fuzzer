use self::TypeOfProblem::*;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

static WINDOWS_RS_FOLDER: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn get_windows_rs_folder() -> String {
    WINDOWS_RS_FOLDER.lock().unwrap().clone()
}

pub fn set_windows_rs_folder(folder: String) {
    *WINDOWS_RS_FOLDER.lock().unwrap() = folder;
}

// thread_local!(pub static WINDOWS_RS_FOLDER: String = "/home/rafal/test/windows-rs/".to_string());
pub const DISABLED_CLASSES: &[&str] = &[
    "AddressBook", // Shows message box when os doesn't have any mail app inside
    "Imapi",       // Shows message box when os doesn't have any mail app inside
    "Shutdown",    // Shutdown computer
    //
    "Gaming",                              // api-ms-win-gaming-expandedresources-l1-1-0.dll
    "P2P",                                 // p2p.dll
    "BiometricFramework",                  // winbio.dll
    "DeploymentServices",                  // wdstptc.dll
    "FileSystem",                          // clfsw32.dll, txfw32.dll, wofutil.dll
    "CloudFilters",                        // cldapi.dll
    "Identity",                            // tokenbinding.dll
    "WindowsConnectionManager",            // ondemandconnroutehelper.dll
    "WindowsProgramming",                  // wldp.dll, api-ms-win-core-realtime-l1-1-2.dll, api-ms-win-core-apiquery-l2-1-0.dll
    "ActiveDirectory",                     // dsparse.dll, dsprop.dll
    "AllJoyn",                             // msajapi.dll
    "ApplicationInstallationAndServicing", // mspatchc.dll
    "Communication",                       // api-ms-win-core-comm-l1-1-1.dll, api-ms-win-core-comm-l1-1-2.dll
    "EnterpriseData",                      // srpapi.dll, efswrt.dll
    "EventCollector",                      // wecapi.dll
    "ExtensibleAuthenticationProtocol",    // eappprxy.dll, eappcfg.dll
    "Fax",                                 // fxsutility.dll, winfax.dll
    "Globalization",                       // bcp47mrm.dll, icu.dll
    "GroupPolicy",                         // gpedit.dll
    "HostComputeNetwork",                  // computenetwork.dll
    "HostComputeSystem",                   // computecore.dll, computestorage.dll
    "Hypervisor",                          // vmsavedstatedumpprovider.dll
    "Clustering",                          // ntlanman.dll
    "ColorSystem",                         // icm32.dll
    "Cryptography",                        // infocardapi.dll
    "DeveloperLicensing",                  // wsclient.dll
    "Dhcp",                                // dhcpsapi.dll
    "Js",                                  // chakra.dll
    "Magnification",                       // magnification.dll
    "Memory",                              // api-ms-win-core-memory-l1-1-8.dll
    "MobileDeviceManagementRegistration",  // mdmlocalmanagement.dll, mdmregistration.dll
    "MsHtml",                              // imgutil.dll, msrating.dll
    "NetShell",                            // netsh.dll
    "OfflineFiles",                        // cscapi.dll
    "ProjectedFileSystem",                 // projectedfslib.dll
    "Rpc",                                 // rpcns4.dll
    "Rras",                                // rtm.dll
    "SecurityCenter",                      // wscapi.dll
    "Sensors",                             // sensorsutilsv2.dll
    "SqlLite",                             // winsqlite3.dll
    "SubsystemForLinux",                   // api-ms-win-wsl-api-l1-1-0.dll
    "SystemInformation",                   // api-ms-win-core-sysinfo-l1-2-4.dll, api-ms-win-core-sysinfo-l1-2-3.dll
    "UserAccessLogging",                   // ualapi.dll
    "WNet",                                // ntlanman.dll
    "WinSock",                             // windows.networking
    "WindowsFirewall",                     // api-ms-win-net-isolation-l1-1-0.dll
    "WindowsNetworkVirtualization",        // wnvapi.dll
    "WindowsWebServices",                  // webauthn.dll
    "XboxController",                      // xinputuap.dll
    "IscsiDisc",                           // iscsidsc.dll
    "DiagnosticDataQuery",                 // diagnosticdataquery.dll
    "Diagnostics",                         // windows.ui.xaml
    "Certificates",                        // certadm.dll
    "WindowsAndMessaging",                 // mrmsupport.dll
    "FileHistory",                         // fhsvcctl.dll
    /////////////////////////
    "Printer",
    "Dwm",
    "Gaming",
    "Dxgi",
    "BiometricFramework",
    "Com",
    "CloudFilters",
    "Debug",
    "DeploymentServices",
    "FileSystem",
    "Identity",
    "MediaFoundation",
    "Ole",
    "P2P",
    "PrintTicket",
    "Printing",
    "Printing2",
    "Shell",
    "Threading",
    "Urlmon",
    "WinRT",
    "WindowsConnectionManager",
    "WindowsProgramming",
    "Accessibility",
    "ActiveDirectory",
    "AddressBook",
    "AllJoyn",
    "Antimalware",
    "AppLocker",
    "ApplicationInstallationAndServicing",
    "ApplicationVerifier",
    "Appx",
    "Audio",
    "Authorization",
    "Bluetooth",
    "Cabinets",
    "CallObj",
    "Catalog",
    "Ceip",
    "Clustering",
    "ColorSystem",
    "Communication",
    "ComponentServices",
    "CompositionSwapchain",
    "Compression",
    // "Console",
    "Controls",
    "CorrelationVector",
    "Credentials",
    "Cryptography",
    "DXCore",
    "DataExchange",
    "DeveloperLicensing",
    "DeviceAccess",
    "DeviceAndDriverInstallation",
    "DeviceQuery",
    "Dhcp",
    "Dialogs",
    "Direct2D",
    "Direct3D10",
    "Direct3D11",
    "Direct3D112",
    "Direct3D11on12",
    "Direct3D12",
    "Direct3D9",
    "Direct3D9on12",
    "DirectComposition",
    "DirectDraw",
    "DirectML",
    "DirectShow",
    "DirectSound",
    "DirectWrite",
    "DirectoryServices",
    "Display",
    "DistributedFileSystem",
    "DistributedTransactionCoordinator",
    "Dns",
    "DxMediaObjects",
    "Dxc",
    "EnterpriseData",
    "Environment",
    "ErrorReporting",
    "Etw",
    "EventCollector",
    "EventLog",
    "EventNotificationService",
    "ExtensibleAuthenticationProtocol",
    "Fax",
    "Foundation",
    "Fxc",
    "Gdi",
    "Globalization",
    "GroupPolicy",
    "HardwareCounterProfiling",
    "HiDpi",
    "HostComputeNetwork",
    "HostComputeSystem",
    "HttpServer",
    "HumanInterfaceDevice",
    "Hypervisor",
    "IO",
    "Iis",
    "Imaging",
    "Imapi",
    "Ime",
    "IndexServer",
    "Input",
    "InstallableFileSystems",
    "InteractionContext",
    "IpHelper",
    "IscsiDisc",
    "Isolation",
    "Jet",
    "JobObjects",
    "Js",
    "Kernel",
    "KernelStreaming",
    "KeyboardAndMouse",
    "Ldap",
    "LibraryLoader",
    "LicenseProtection",
    "Magnification",
    "Mailslots",
    "Mapi",
    "Marshal",
    "Media",
    "Memory",
    "MobileDeviceManagementRegistration",
    "MsHtml",
    "Multicast",
    "Multimedia",
    "NetBios",
    "NetManagement",
    "NetShell",
    "NetworkDiagnosticsFramework",
    "NonVolatile",
    "OfflineFiles",
    "OpenGL",
    "OperationRecorder",
    "PasswordManagement",
    "Pdf",
    "Performance",
    "Pipes",
    "Pnp",
    "Pointer",
    "PortableDevices",
    "Power",
    "ProcessSnapshotting",
    "ProcessStatus",
    "ProjectedFileSystem",
    "PropertiesSystem",
    "QoS",
    "Recovery",
    "Registry",
    "RemoteDesktop",
    "RemoteManagement",
    "RestartManager",
    "Restore",
    "RightsManagement",
    "Rpc",
    "Rras",
    "Search",
    "Security",
    "SecurityCenter",
    "Sensors",
    "SerialCommunication",
    "Services",
    "SetupAndMigration",
    "Shutdown",
    "Sip",
    "Snmp",
    "SqlLite",
    "StationsAndDesktops",
    "StructuredStorage",
    "SubsystemForLinux",
    "SystemInformation",
    "SystemServices",
    "TabletPC",
    "Tapi",
    "TextServices",
    "Time",
    "ToolHelp",
    "Touch",
    "TpmBaseServices",
    "UI",
    "UI2",
    "Usb",
    "UserAccessLogging",
    "Vhd",
    "Vss",
    "WNet",
    "WebDav",
    "WebServicesOnDevices",
    "WebSocket",
    "WiFi",
    "WinHttp",
    "WinInet",
    "WinML",
    "WinSock",
    "WinTrust",
    "WindowsFilteringPlatform",
    "WindowsFirewall",
    "WindowsMediaFormat",
    "WindowsNetworkVirtualization",
    "WindowsWebServices",
    "Wmi",
    "XAudio2",
    "XboxController",
    "XmlLite",
    "Xps",
];

#[derive(Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub enum TypeOfProblem {
    NotImplementedLinux,
    NotImplementedWindows,
    InvalidNumberOfArguments,
    ShowsDialogWindows,
    ShowsDialogLinux,
    CrashesWindows,
    CrashesLinux,
    CrashAutomatic, // Crashes are normal for this code
    Freeze,
    Other,
}

pub fn load_settings() -> Vec<(&'static str, String, Vec<(&'static str, TypeOfProblem)>)> {
    vec![
        (
            "Dwm",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Graphics/Dwm/mod.rs"),
            vec![
                ("DwmModifyPreviousDxFrameDuration", NotImplementedLinux),
                ("DwmTetherContact", NotImplementedLinux),
                ("DwmRegisterThumbnail", InvalidNumberOfArguments),
                ("DwmSetDxFrameDuration", NotImplementedLinux),
                ("DwmIsCompositionEnabled", InvalidNumberOfArguments),
                ("DwmGetGraphicsStreamClient", InvalidNumberOfArguments),
                ("DwmQueryThumbnailSourceSize", InvalidNumberOfArguments),
            ],
        ),
        (
            "Gaming",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Gaming/mod.rs"),
            vec![("GetExpandedResourceExclusiveCpuCount", InvalidNumberOfArguments)],
        ),
        (
            "Dxgi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Graphics/Dxgi/mod.rs"),
            vec![("DXGIDeclareAdapterRemovalSupport", NotImplementedLinux)],
        ),
        (
            "BiometricFramework",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/BiometricFramework/mod.rs"
            ),
            vec![("WinBioEnrollCapture", CrashesWindows), ("WinBioLocateSensor", CrashesWindows)],
        ),
        (
            "Com",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Com/mod.rs"),
            vec![
                ("CoCancelCall", NotImplementedLinux),
                ("CoDisconnectContext", NotImplementedLinux),
                ("CoInvalidateRemoteMachineBindings", NotImplementedLinux),
                ("CoTestCancel", NotImplementedLinux),
                ("CoGetCallerTID", CrashesWindows),
                ("CoGetContextToken", CrashesWindows),
                ("CLSIDFromProgID", InvalidNumberOfArguments),
                ("CLSIDFromProgIDEx", InvalidNumberOfArguments),
                ("CLSIDFromString", InvalidNumberOfArguments),
                ("CoCreateGuid", InvalidNumberOfArguments),
                ("CoFileTimeNow", InvalidNumberOfArguments),
                ("CoGetCurrentLogicalThreadId", InvalidNumberOfArguments),
                ("GetClassFile", InvalidNumberOfArguments),
                ("IIDFromString", InvalidNumberOfArguments),
            ],
        ),
        (
            "CloudFilters",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/CloudFilters/mod.rs"
            ),
            vec![("CfGetTransferKey", CrashesWindows)],
        ),
        (
            "Debug",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Debug/mod.rs"
            ),
            vec![
                ("FatalExit", CrashAutomatic),
                ("FindDebugInfoFile", CrashesLinux),
                ("SearchTreeForFileW", CrashesLinux),
                ("FindFileInSearchPath", NotImplementedLinux),
                ("GetSymLoadError", NotImplementedLinux),
                ("RangeMapCreate", NotImplementedLinux),
                ("ReBaseImage64", NotImplementedLinux),
                ("SetSymLoadError", NotImplementedLinux),
                ("SymAddrIncludeInlineTrace", NotImplementedLinux),
                ("SymCompareInlineTrace", NotImplementedLinux),
                ("RemoveInvalidModuleList", NotImplementedLinux),
                ("SymDeleteSymbol", NotImplementedLinux),
                ("SymDeleteSymbolW", NotImplementedLinux),
                ("SymGetFileLineOffsets64", NotImplementedLinux),
                ("SymGetSourceFile", NotImplementedLinux),
                ("SymGetSourceFileChecksum", NotImplementedLinux),
                ("SymGetSourceFileChecksumW", NotImplementedLinux),
                ("SymGetSourceFileW", NotImplementedLinux),
                ("SymQueryInlineTrace", NotImplementedLinux),
                ("SymSrvDeltaName", NotImplementedLinux),
                ("SymSrvDeltaNameW", NotImplementedLinux),
                ("SymSrvGetFileIndexString", NotImplementedLinux),
                ("SymSrvGetFileIndexStringW", NotImplementedLinux),
                ("SymSrvGetFileIndexes", NotImplementedLinux),
                ("SymSrvGetFileIndexesW", NotImplementedLinux),
                ("SymSrvGetSupplement", NotImplementedLinux),
                ("SymSrvGetSupplementW", NotImplementedLinux),
                ("SymSrvIsStore", NotImplementedLinux),
                ("SymSrvIsStoreW", NotImplementedLinux),
                ("SymSrvStoreSupplement", NotImplementedLinux),
                ("SymSrvStoreSupplementW", NotImplementedLinux),
                ("SymMatchFileName", CrashesLinux),
                ("MakeSureDirectoryPathExists", CrashesLinux),
                ("SymFunctionTableAccess", NotImplementedWindows),
                ("SymGetModuleBase", NotImplementedWindows),
                ("SymLoadModule", NotImplementedWindows),
                ("SymUnloadModule", NotImplementedWindows),
                ("Beep", Freeze),
                ("DebugBreak", CrashAutomatic),
                ("SymGetSearchPath", CrashesWindows),
                ("TerminateProcessOnMemoryExhaustion", CrashesWindows),
                ("FatalAppExitW", CrashesWindows),
                ("SymMatchFileNameW", CrashesWindows),
                ("FatalAppExitA", CrashesWindows),
            ],
        ),
        (
            "DeploymentServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/DeploymentServices/mod.rs"
            ),
            vec![
                ("WdsCliGetEnumerationFlags", CrashesWindows),
                ("WdsCliFindFirstImage", CrashesWindows),
                ("WdsCliGetImageHandleFromFindHandle", CrashesWindows),
                ("WdsCliGetImageHandleFromTransferHandle", CrashesWindows),
                ("WdsCliGetImageIndex", CrashesWindows),
                ("WdsCliGetImageSize", CrashesWindows),
                ("WdsCliGetTransferSize", CrashesWindows),
                ("WdsCliGetImageVersion", InvalidNumberOfArguments),
                ("WdsCliGetImagePath", InvalidNumberOfArguments),
                ("WdsCliGetImageNamespace", InvalidNumberOfArguments),
                ("WdsCliGetImageName", InvalidNumberOfArguments),
                ("WdsCliGetImageLanguage", InvalidNumberOfArguments),
                ("WdsCliGetImageHalName", InvalidNumberOfArguments),
                ("WdsCliGetImageGroup", InvalidNumberOfArguments),
                ("WdsCliGetImageDescription", InvalidNumberOfArguments),
                ("WdsCliGetDriverQueryXml", InvalidNumberOfArguments),
            ],
        ),
        (
            "FileSystem",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/FileSystem/mod.rs"
            ),
            vec![("WofGetDriverVersion", CrashesWindows), ("WofWimAddEntry", CrashesWindows)],
        ),
        (
            "Identity",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Authentication/Identity/mod.rs"
            ),
            vec![("SLGetWindowsInformationDWORD", CrashesWindows)],
        ),
        (
            "MediaFoundation",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/MediaFoundation/mod.rs"
            ),
            vec![
                ("MFAllocateSerialWorkQueue", InvalidNumberOfArguments),
                ("MFAllocateWorkQueue", InvalidNumberOfArguments),
                ("MFFrameRateToAverageTimePerFrame", InvalidNumberOfArguments),
                ("MFGetPlaneSize", InvalidNumberOfArguments),
                ("MFGetStrideForBitmapInfoHeader", InvalidNumberOfArguments),
                ("MFGetTimerPeriodicity", InvalidNumberOfArguments),
                ("MFGetWorkQueueMMCSSPriority", InvalidNumberOfArguments),
                ("MFGetWorkQueueMMCSSTaskId", InvalidNumberOfArguments),
            ],
        ),
        (
            "Ole",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Ole/mod.rs"),
            vec![
                ("OaEnablePerUserTLibRegistration", NotImplementedLinux),
                ("VarI4FromI8", InvalidNumberOfArguments),
                ("VarI4FromStr", InvalidNumberOfArguments),
                ("VarI4FromUI4", InvalidNumberOfArguments),
                ("VarI4FromUI8", InvalidNumberOfArguments),
                ("VarI8FromStr", InvalidNumberOfArguments),
                ("VarI8FromUI4", InvalidNumberOfArguments),
                ("VarI8FromUI4", InvalidNumberOfArguments),
                ("VarI8FromUI8", InvalidNumberOfArguments),
                ("VarUI4FromI4", InvalidNumberOfArguments),
                ("VarUI4FromI8", InvalidNumberOfArguments),
                ("VarUI4FromStr", InvalidNumberOfArguments),
                ("VarUI4FromUI8", InvalidNumberOfArguments),
                ("VarUI8FromI8", InvalidNumberOfArguments),
                ("VarUI8FromStr", InvalidNumberOfArguments),
                ("VarUI8FromUI4", InvalidNumberOfArguments),
                ("VarDateFromI4", InvalidNumberOfArguments),
                ("VarDateFromI8", InvalidNumberOfArguments),
                ("VarDateFromR4", InvalidNumberOfArguments),
                ("VarDateFromR8", InvalidNumberOfArguments),
                ("VarDateFromStr", InvalidNumberOfArguments),
                ("VarDateFromUI2", InvalidNumberOfArguments),
                ("VarDateFromUI4", InvalidNumberOfArguments),
                ("VarDateFromUI8", InvalidNumberOfArguments),
                ("VarI4FromDate", InvalidNumberOfArguments),
                ("VarI4FromR4", InvalidNumberOfArguments),
                ("VarI4FromR8", InvalidNumberOfArguments),
                ("VarI4FromUI2", InvalidNumberOfArguments),
                ("VarI8FromDate", InvalidNumberOfArguments),
                ("VarI8FromR4", InvalidNumberOfArguments),
                ("VarI8FromR8", InvalidNumberOfArguments),
                ("VarI8FromUI2", InvalidNumberOfArguments),
                ("VarR4FromDate", InvalidNumberOfArguments),
                ("VarR4FromI4", InvalidNumberOfArguments),
                ("VarR4FromI8", InvalidNumberOfArguments),
                ("VarR4FromR8", InvalidNumberOfArguments),
                ("VarR4FromStr", InvalidNumberOfArguments),
                ("VarR4FromUI2", InvalidNumberOfArguments),
                ("VarR4FromUI4", InvalidNumberOfArguments),
                ("VarR4FromUI8", InvalidNumberOfArguments),
                ("VarR8FromDate", InvalidNumberOfArguments),
                ("VarR8FromI4", InvalidNumberOfArguments),
                ("VarR8FromI8", InvalidNumberOfArguments),
                ("VarR8FromR4", InvalidNumberOfArguments),
                ("VarR8FromStr", InvalidNumberOfArguments),
                ("VarR8FromUI2", InvalidNumberOfArguments),
                ("VarR8FromUI4", InvalidNumberOfArguments),
                ("VarR8FromUI8", InvalidNumberOfArguments),
                ("VarR8Pow", InvalidNumberOfArguments),
                ("VarR8Round", InvalidNumberOfArguments),
                ("VarUI2FromDate", InvalidNumberOfArguments),
                ("VarUI2FromI4", InvalidNumberOfArguments),
                ("VarUI2FromI8", InvalidNumberOfArguments),
                ("VarUI2FromR4", InvalidNumberOfArguments),
                ("VarUI2FromStr", InvalidNumberOfArguments),
                ("VarUI2FromUI4", InvalidNumberOfArguments),
                ("VarUI2FromUI8", InvalidNumberOfArguments),
                ("VarUI4FromDate", InvalidNumberOfArguments),
                ("VarUI4FromR4", InvalidNumberOfArguments),
                ("VarUI4FromR8", InvalidNumberOfArguments),
                ("VarUI4FromUI2", InvalidNumberOfArguments),
                ("VarUI8FromDate", InvalidNumberOfArguments),
                ("VarUI8FromR4", InvalidNumberOfArguments),
                ("VarUI8FromR8", InvalidNumberOfArguments),
                ("VarUI8FromUI2", InvalidNumberOfArguments),
                ("VarBoolFromDate", InvalidNumberOfArguments),
                ("VarBoolFromI2", InvalidNumberOfArguments),
                ("VarBoolFromI4", InvalidNumberOfArguments),
                ("VarBoolFromI8", InvalidNumberOfArguments),
                ("VarBoolFromR4", InvalidNumberOfArguments),
                ("VarBoolFromR8", InvalidNumberOfArguments),
                ("VarBoolFromStr", InvalidNumberOfArguments),
                ("VarBoolFromUI1", InvalidNumberOfArguments),
                ("VarBoolFromUI2", InvalidNumberOfArguments),
                ("VarBoolFromUI4", InvalidNumberOfArguments),
                ("VarBoolFromUI8", InvalidNumberOfArguments),
                ("VarDateFromBool", InvalidNumberOfArguments),
                ("VarDateFromI2", InvalidNumberOfArguments),
                ("VarDateFromUI1", InvalidNumberOfArguments),
                ("VarI2FromBool", InvalidNumberOfArguments),
                ("VarI2FromDate", InvalidNumberOfArguments),
                ("VarI2FromI4", InvalidNumberOfArguments),
                ("VarI2FromI8", InvalidNumberOfArguments),
                ("VarI2FromR4", InvalidNumberOfArguments),
                ("VarI2FromR8", InvalidNumberOfArguments),
                ("VarI2FromStr", InvalidNumberOfArguments),
                ("VarI2FromUI1", InvalidNumberOfArguments),
                ("VarI2FromUI2", InvalidNumberOfArguments),
                ("VarI2FromUI4", InvalidNumberOfArguments),
                ("VarI2FromUI8", InvalidNumberOfArguments),
                ("VarI4FromBool", InvalidNumberOfArguments),
                ("VarI4FromI2", InvalidNumberOfArguments),
                ("VarI4FromUI1", InvalidNumberOfArguments),
                ("VarI8FromBool", InvalidNumberOfArguments),
                ("VarI8FromI2", InvalidNumberOfArguments),
                ("VarI8FromUI1", InvalidNumberOfArguments),
                ("VarR4FromBool", InvalidNumberOfArguments),
                ("VarR4FromI2", InvalidNumberOfArguments),
                ("VarR4FromUI1", InvalidNumberOfArguments),
                ("VarR8FromBool", InvalidNumberOfArguments),
                ("VarR8FromI2", InvalidNumberOfArguments),
                ("VarR8FromUI1", InvalidNumberOfArguments),
                ("VarUI1FromBool", InvalidNumberOfArguments),
                ("VarUI1FromDate", InvalidNumberOfArguments),
                ("VarUI1FromI2", InvalidNumberOfArguments),
                ("VarUI1FromI4", InvalidNumberOfArguments),
                ("VarUI1FromI8", InvalidNumberOfArguments),
                ("VarUI1FromR4", InvalidNumberOfArguments),
                ("VarUI1FromR8", InvalidNumberOfArguments),
                ("VarUI1FromStr", InvalidNumberOfArguments),
                ("VarUI1FromUI2", InvalidNumberOfArguments),
                ("VarUI1FromUI4", InvalidNumberOfArguments),
                ("VarUI1FromUI8", InvalidNumberOfArguments),
                ("VarUI2FromBool", InvalidNumberOfArguments),
                ("VarUI2FromI2", InvalidNumberOfArguments),
                ("VarUI2FromUI1", InvalidNumberOfArguments),
                ("VarUI4FromBool", InvalidNumberOfArguments),
                ("VarUI4FromI2", InvalidNumberOfArguments),
                ("VarUI4FromUI1", InvalidNumberOfArguments),
                ("VarUI8FromBool", InvalidNumberOfArguments),
                ("VarUI8FromI2", InvalidNumberOfArguments),
                ("VarUI8FromUI1", InvalidNumberOfArguments),
                ("VarBoolFromI1", InvalidNumberOfArguments),
                ("VarBstrFromBool", InvalidNumberOfArguments),
                ("VarBstrFromDate", InvalidNumberOfArguments),
                ("VarBstrFromI1", InvalidNumberOfArguments),
                ("VarBstrFromI2", InvalidNumberOfArguments),
                ("VarBstrFromI4", InvalidNumberOfArguments),
                ("VarBstrFromI8", InvalidNumberOfArguments),
                ("VarBstrFromR4", InvalidNumberOfArguments),
                ("VarBstrFromR8", InvalidNumberOfArguments),
                ("VarBstrFromUI1", InvalidNumberOfArguments),
                ("VarBstrFromUI2", InvalidNumberOfArguments),
                ("VarBstrFromUI4", InvalidNumberOfArguments),
                ("VarBstrFromUI8", InvalidNumberOfArguments),
                ("VarDateFromI1", InvalidNumberOfArguments),
                ("VarI2FromI1", InvalidNumberOfArguments),
                ("VarI4FromI1", InvalidNumberOfArguments),
                ("VarI8FromI1", InvalidNumberOfArguments),
                ("VarMonthName", InvalidNumberOfArguments),
                ("VarR4FromI1", InvalidNumberOfArguments),
                ("VarUI1FromI1", InvalidNumberOfArguments),
                ("VarUI2FromI1", InvalidNumberOfArguments),
                ("VarUI4FromI1", InvalidNumberOfArguments),
                ("VarUI8FromI1", InvalidNumberOfArguments),
                ("VarWeekdayName", InvalidNumberOfArguments),
                ("VarBoolFromCy", InvalidNumberOfArguments),
                ("VarBstrFromCy", InvalidNumberOfArguments),
                ("VarCyAbs", InvalidNumberOfArguments),
                ("VarCyAdd", InvalidNumberOfArguments),
                ("VarCyFix", InvalidNumberOfArguments),
                ("VarCyFromBool", InvalidNumberOfArguments),
                ("VarCyFromDate", InvalidNumberOfArguments),
                ("VarCyFromI1", InvalidNumberOfArguments),
                ("VarCyFromI2", InvalidNumberOfArguments),
                ("VarCyFromI4", InvalidNumberOfArguments),
                ("VarCyFromI8", InvalidNumberOfArguments),
                ("VarCyFromR4", InvalidNumberOfArguments),
                ("VarCyFromR8", InvalidNumberOfArguments),
                ("VarCyFromStr", InvalidNumberOfArguments),
                ("VarCyFromUI1", InvalidNumberOfArguments),
                ("VarCyFromUI2", InvalidNumberOfArguments),
                ("VarCyFromUI4", InvalidNumberOfArguments),
                ("VarCyFromUI8", InvalidNumberOfArguments),
                ("VarCyInt", InvalidNumberOfArguments),
                ("VarCyMul", InvalidNumberOfArguments),
                ("VarCyMulI4", InvalidNumberOfArguments),
                ("VarCyMulI8", InvalidNumberOfArguments),
                ("VarCyNeg", InvalidNumberOfArguments),
                ("VarCyRound", InvalidNumberOfArguments),
                ("VarCySub", InvalidNumberOfArguments),
                ("VarDateFromCy", InvalidNumberOfArguments),
                ("VarDecFromBool", InvalidNumberOfArguments),
                ("VarDecFromCy", InvalidNumberOfArguments),
                ("VarDecFromDate", InvalidNumberOfArguments),
                ("VarDecFromI1", InvalidNumberOfArguments),
                ("VarDecFromI2", InvalidNumberOfArguments),
                ("VarDecFromI4", InvalidNumberOfArguments),
                ("VarDecFromI8", InvalidNumberOfArguments),
                ("VarDecFromR4", InvalidNumberOfArguments),
                ("VarDecFromR8", InvalidNumberOfArguments),
                ("VarDecFromStr", InvalidNumberOfArguments),
                ("VarDecFromUI1", InvalidNumberOfArguments),
                ("VarDecFromUI2", InvalidNumberOfArguments),
                ("VarDecFromUI4", InvalidNumberOfArguments),
                ("VarDecFromUI8", InvalidNumberOfArguments),
                ("VarI4FromCy", InvalidNumberOfArguments),
                ("VarI8FromCy", InvalidNumberOfArguments),
                ("VarUI1FromCy", InvalidNumberOfArguments),
                ("VarUI2FromCy", InvalidNumberOfArguments),
                ("VarUI4FromCy", InvalidNumberOfArguments),
                ("VarUI8FromCy", InvalidNumberOfArguments),
                ("VarI1FromCy", CrashesWindows),
                ("VarI1FromBool", CrashesWindows),
                ("VarI1FromUI1", CrashesWindows),
                ("VarI1FromI2", CrashesWindows),
                ("VarI1FromR8", CrashesWindows),
                ("VarI1FromR4", CrashesWindows),
                ("VarI1FromUI2", CrashesWindows),
                ("VarI1FromI4", CrashesWindows),
                ("VarI1FromDate", CrashesWindows),
                ("VarI1FromUI8", CrashesWindows),
                ("VarI1FromUI4", CrashesWindows),
                ("OleDestroyMenuDescriptor", CrashesWindows),
                ("VarI1FromI8", CrashesWindows),
            ],
        ),
        (
            "P2P",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/P2P/mod.rs"
            ),
            vec![
                ("PeerIdentityGetCryptKey", InvalidNumberOfArguments),
                ("PeerCollabGetSigninOptions", InvalidNumberOfArguments),
                ("PeerNameToPeerHostName", InvalidNumberOfArguments),
                ("PeerIdentityImport", InvalidNumberOfArguments),
                ("PeerIdentityGetXML", InvalidNumberOfArguments),
                ("PeerIdentityGetFriendlyName", InvalidNumberOfArguments),
                ("PeerIdentityGetDefault", InvalidNumberOfArguments),
                ("PeerIdentityExport", InvalidNumberOfArguments),
                ("PeerIdentityCreate", InvalidNumberOfArguments),
                ("PeerHostNameToPeerName", InvalidNumberOfArguments),
                ("PeerCreatePeerName", InvalidNumberOfArguments),
                ("PeerCollabGetEndpointName", InvalidNumberOfArguments),
                ("PeerCollabExportContact", InvalidNumberOfArguments),
            ],
        ),
        (
            "PrintTicket",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Printing/PrintTicket/mod.rs"
            ),
            vec![("PTQuerySchemaVersionSupport", InvalidNumberOfArguments)],
        ),
        (
            "Printing",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"
            ),
            vec![
                ("CloseSpoolFileHandle", NotImplementedLinux),
                ("CommitSpoolData", NotImplementedLinux),
                ("DeleteJobNamedProperty", NotImplementedLinux),
                ("DeletePrinterDataA", NotImplementedLinux),
                ("DeletePrinterDataW", NotImplementedLinux),
                ("DeletePrinterDriverPackageA", NotImplementedLinux),
                ("DeletePrinterDriverPackageW", NotImplementedLinux),
                ("DeletePrinterIC", NotImplementedLinux),
                ("DeletePrinterKeyA", NotImplementedLinux),
                ("DeletePrinterKeyW", NotImplementedLinux),
                ("GdiDeleteSpoolFileHandle", NotImplementedLinux),
                ("GdiEndDocEMF", NotImplementedLinux),
                ("GdiEndPageEMF", NotImplementedLinux),
                ("GdiGetDC", NotImplementedLinux),
                ("GdiGetPageCount", NotImplementedLinux),
                ("GdiGetPageHandle", NotImplementedLinux),
                ("GdiStartPageEMF", NotImplementedLinux),
                ("GetPrintOutputInfo", NotImplementedLinux),
                ("GetPrinterDriver2A", NotImplementedLinux),
                ("GetPrinterDriver2W", NotImplementedLinux),
                ("GetPrinterDriverPackagePathA", NotImplementedLinux),
                ("GetPrinterDriverPackagePathW", NotImplementedLinux),
                ("GetSpoolFileHandle", NotImplementedLinux),
                ("InstallPrinterDriverFromPackageA", NotImplementedLinux),
                ("InstallPrinterDriverFromPackageW", NotImplementedLinux),
                ("PrinterMessageBoxA", NotImplementedLinux),
                ("PrinterMessageBoxW", NotImplementedLinux),
                ("ProvidorFindClosePrinterChangeNotification", NotImplementedLinux),
                ("RemovePrintDeviceObject", NotImplementedLinux),
                ("RouterAllocBidiResponseContainer", NotImplementedLinux),
                ("RouterAllocPrinterNotifyInfo", NotImplementedLinux),
                ("SplIsSessionZero", NotImplementedLinux),
                ("SpoolerFindClosePrinterChangeNotification", NotImplementedLinux),
                ("UnRegisterForPrintAsyncNotifications", NotImplementedLinux),
                ("UpdatePrintDeviceObject", NotImplementedLinux),
                ("WaitForPrinterChange", NotImplementedLinux),
                ("AddPrintDeviceObject", InvalidNumberOfArguments),
                ("CorePrinterDriverInstalledA", InvalidNumberOfArguments),
                ("CorePrinterDriverInstalledW", InvalidNumberOfArguments),
                ("RouterAllocBidiMem", CrashesWindows),
                ("SHRegSetPathA", CrashesWindows),
                ("ConnectToPrinterDlg", ShowsDialogWindows),
                ("GdiPlayPageEMF", NotImplementedLinux),
                ("EnumPrintersW", CrashesLinux),
                ("GetPrinterDriverA", CrashesLinux),
                ("GetPrinterDriverW", CrashesLinux),
                ("EnumPrintersW", CrashesLinux),     // Stack Overflow
                ("GetPrinterDriverA", CrashesLinux), // Stack Overflow
                ("GetPrinterDriverW", CrashesLinux), // Stack Overflow
            ],
        ),
        (
            "Printing2",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/Xps/Printing/mod.rs"
            ),
            vec![],
        ),
        (
            "Shell",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/Shell/mod.rs"),
            vec![
                ("RegisterScaleChangeEvent", InvalidNumberOfArguments),
                ("SHGetDriveMedia", InvalidNumberOfArguments),
                ("SHIsFileAvailableOffline", InvalidNumberOfArguments),
                ("SHStrDupW", InvalidNumberOfArguments),
                ("SHStrDupA", InvalidNumberOfArguments),
                ("PathCchSkipRoot", InvalidNumberOfArguments),
                ("PathCchFindExtension", InvalidNumberOfArguments),
                ("PathAllocCombine", InvalidNumberOfArguments),
                ("PathAllocCanonicalize", InvalidNumberOfArguments),
                ("HlinkTranslateURL", InvalidNumberOfArguments),
                ("HlinkGetValueFromParams", InvalidNumberOfArguments),
                ("HlinkGetSpecialReference", InvalidNumberOfArguments),
                ("GetCurrentProcessExplicitAppUserModelID", InvalidNumberOfArguments),
                ("SHCLSIDFromString", InvalidNumberOfArguments),
                ("PathCchCanonicalizeEx", CrashesWindows),
                ("SHSimpleIDListFromPath", CrashesWindows),
                ("SHAnsiToAnsi", CrashesWindows),
                ("RestartDialogEx", Other), // Restart Windows(without dialog)
                ("StrCmpICW", CrashesWindows),
                ("wnsprintfW", CrashesWindows),
                ("SHGetNewLinkInfoW", CrashesWindows),
                ("StrCmpNICA", CrashesWindows),
                ("PathCchAddBackslash", CrashesWindows),
                ("SHRegGetBoolUSValueW", CrashesWindows),
                ("SHRegCreateUSKeyW", CrashesWindows),
                ("SHUnicodeToUnicode", CrashesWindows),
                ("ExtractAssociatedIconW", CrashesWindows),
                ("SHFind_InitMenuPopup", CrashesWindows),
                ("PathCchCanonicalize", CrashesWindows),
                ("UrlFixupW", CrashesWindows),
                ("StrCmpNCW", CrashesWindows),
                ("SetCurrentProcessExplicitAppUserModelID", CrashesWindows),
                ("StrCpyNW", CrashesWindows),
                ("SHRegCloseUSKey", CrashesWindows),
                ("DoEnvironmentSubstA", CrashesWindows),
                ("CommandLineToArgvW", CrashesWindows),
                ("SHEvaluateSystemCommandTemplate", CrashesWindows),
                ("PathBuildRootW", CrashesWindows),
                ("PathCchRemoveBackslash", CrashesWindows),
                ("SHRegSetPathW", CrashesWindows),
                ("FindExecutableW", CrashesWindows),
                ("SHMessageBoxCheckW", ShowsDialogWindows),
                ("SHMessageBoxCheckA", ShowsDialogWindows),
                ("PathQualify", CrashesWindows),
                ("StrCmpNICW", CrashesWindows),
                ("StrCmpCW", CrashesWindows),
                ("PathCleanupSpec", CrashesWindows),
                ("SHMessageBoxCheckW", ShowsDialogWindows),
                ("StrCmpCA", CrashesWindows),
                ("PathIsUNCEx", CrashesWindows),
                ("SHEmptyRecycleBinA", ShowsDialogWindows),
                ("SHEmptyRecycleBinW", CrashesWindows),
                ("WinHelpW", ShowsDialogWindows),
                ("WinHelpA", ShowsDialogWindows),
                ("SHRegOpenUSKeyW", CrashesWindows),
                ("ExtractAssociatedIconExW", CrashesWindows),
                ("SHGetNewLinkInfoA", CrashesWindows),
                ("PathCchAddBackslashEx", CrashesWindows),
                ("DoEnvironmentSubstW", CrashesWindows),
                ("SHStripMneumonicW", CrashesWindows),
                ("StrCmpNCA", CrashesWindows),
                ("Shell_GetCachedImageIndexW", CrashesWindows),
                ("wnsprintfA", CrashesWindows),
                ("SHStripMneumonicA", CrashesWindows),
                ("RestartDialog", ShowsDialogWindows),
                ("StrCmpICA", CrashesWindows),
                ("FindExecutableA", CrashesWindows),
                ("StrCatChainW", CrashesWindows),
                ("PathCchRemoveBackslashEx", CrashesWindows),
                ("Shell_GetCachedImageIndex", CrashesWindows),
                ("SHRegSetPathA", CrashesWindows),
                ("CreateProfile", NotImplementedLinux),
                ("DeleteProfileA", NotImplementedLinux),
                ("DeleteProfileW", NotImplementedLinux),
                ("GetFileNameFromBrowse", ShowsDialogLinux),
                ("HlinkCreateShortcutFromString", NotImplementedLinux),
                ("HlinkResolveShortcutToString", NotImplementedLinux),
                ("HlinkSetSpecialReference", NotImplementedLinux),
                ("PathIsExe", CrashesLinux),
                ("PathIsSlowA", NotImplementedLinux),
                ("PathIsSlowW", NotImplementedLinux),
                ("PathMatchSpecA", CrashesLinux),
                ("PathMatchSpecExA", NotImplementedLinux),
                ("PathMatchSpecExW", NotImplementedLinux),
                ("PathMatchSpecExW", NotImplementedLinux),
                ("PathMatchSpecW", CrashesLinux),
                ("PathUnExpandEnvStringsW", CrashesLinux),
                ("PathYetAnotherMakeUniqueName", CrashesLinux),
                ("SHGetUnreadMailCountW", NotImplementedLinux),
                ("SHInvokePrinterCommandA", NotImplementedLinux),
                ("SHInvokePrinterCommandW", NotImplementedLinux),
                ("SHRegGetBoolUSValueA", CrashesLinux),
                ("SHRegOpenUSKeyA", CrashesLinux),
                ("SHSetFolderPathA", NotImplementedLinux),
                ("SHSetFolderPathW", NotImplementedLinux),
                ("SHStartNetConnectionDialogW", NotImplementedLinux),
                ("SHTestTokenMembership", NotImplementedLinux),
                ("ShellMessageBoxA", NotImplementedLinux),
                ("ShellMessageBoxW", NotImplementedLinux),
                ("StrChrNIW", NotImplementedLinux),
                ("UnregisterScaleChangeEvent", NotImplementedLinux),
                ("UrlCompareA", CrashesLinux),
                ("UrlGetLocationA", CrashesLinux),
                ("UrlGetLocationW", CrashesLinux),
                ("UrlHashW", CrashesLinux),
                ("Shell_GetCachedImageIndexA", NotImplementedLinux),
            ],
        ),
        (
            "Threading",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Threading/mod.rs"
            ),
            vec![
                ("ExitProcess", CrashAutomatic),
                ("ExitThread", CrashAutomatic),
                ("GetThreadDescription", InvalidNumberOfArguments),
                ("Sleep", Freeze),
                ("SleepEx", Freeze),
                ("AvSetMmMaxThreadCharacteristicsW", CrashesWindows),
                ("AvSetMmThreadCharacteristicsA", CrashesWindows),
                ("AvSetMmThreadCharacteristicsW", CrashesWindows),
                ("AvSetMmMaxThreadCharacteristicsA", CrashesWindows),
                ("CloseThreadpoolCleanupGroup", CrashesWindows),
                ("AddIntegrityLabelToBoundaryDescriptor", NotImplementedLinux),
                ("AddSIDToBoundaryDescriptor", NotImplementedLinux),
                ("AvRtDeleteThreadOrderingGroup", NotImplementedLinux),
                ("AvRtLeaveThreadOrderingGroup", NotImplementedLinux),
                ("AvRtWaitOnThreadOrderingGroup", NotImplementedLinux),
                ("ChangeTimerQueueTimer", CrashesLinux),
                ("CreateBoundaryDescriptorA", NotImplementedLinux),
                ("GetThreadSelectedCpuSets", NotImplementedLinux),
                ("IsImmersiveProcess", NotImplementedLinux),
                ("IsProcessCritical", NotImplementedLinux),
                ("SetProcessRestrictionExemption", NotImplementedLinux),
                ("WinExec", CrashesLinux),
                ("Wow64SuspendThread", NotImplementedLinux),
                ("GetNumaNodeNumberFromHandle", NotImplementedLinux),
                ("GetProcessDefaultCpuSets", NotImplementedLinux),
                ("Wow64SetThreadDefaultGuestMachine", NotImplementedLinux),
            ],
        ),
        (
            "Urlmon",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Com/Urlmon/mod.rs"
            ),
            vec![
                ("IEInstallScope", InvalidNumberOfArguments),
                ("FindMediaType", InvalidNumberOfArguments),
                ("GetClassURL", InvalidNumberOfArguments),
                ("CoInternetCompareUrl", CrashesWindows),
                ("CoInternetCombineUrl", CrashesLinux),
                ("CoInternetGetProtocolFlags", NotImplementedLinux),
                ("IEGetUserPrivateNamespaceName", NotImplementedLinux),
                ("SetSoftwareUpdateAdvertisementState", NotImplementedLinux),
            ],
        ),
        (
            "WinRT",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/WinRT/mod.rs"),
            vec![
                ("RoGetApartmentIdentifier", InvalidNumberOfArguments),
                ("RoGetErrorReportingFlags", InvalidNumberOfArguments),
                ("WindowsCompareStringOrdinal", InvalidNumberOfArguments),
                ("WindowsConcatString", InvalidNumberOfArguments),
                ("WindowsCreateString", InvalidNumberOfArguments),
                ("WindowsDuplicateString", InvalidNumberOfArguments),
                ("WindowsReplaceString", InvalidNumberOfArguments),
                ("WindowsStringHasEmbeddedNull", InvalidNumberOfArguments),
                ("WindowsSubstring", InvalidNumberOfArguments),
                ("WindowsSubstringWithSpecifiedLength", InvalidNumberOfArguments),
                ("WindowsTrimStringEnd", InvalidNumberOfArguments),
                ("WindowsTrimStringStart", InvalidNumberOfArguments),
                ("IsErrorPropagationEnabled", NotImplementedLinux),
                ("RoClearError", NotImplementedLinux),
                ("RoRevokeActivationFactories", NotImplementedLinux),
                ("RoSetErrorReportingFlags", NotImplementedLinux),
            ],
        ),
        (
            "WindowsAndMessaging",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/WindowsAndMessaging/mod.rs"
            ),
            vec![("MrmGetPriFileContentChecksum", InvalidNumberOfArguments)],
        ),
        (
            "WindowsConnectionManager",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsConnectionManager/mod.rs"
            ),
            vec![("OnDemandGetRoutingHint", InvalidNumberOfArguments)],
        ),
        (
            "WindowsProgramming",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/WindowsProgramming/mod.rs"
            ),
            vec![
                ("QueryAuxiliaryCounterFrequency", InvalidNumberOfArguments),
                ("WldpIsDynamicCodePolicyEnabled", InvalidNumberOfArguments),
            ],
        ),
        (
            "Accessibility",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Accessibility/mod.rs"
            ),
            vec![
                ("AccNotifyTouchInteraction", NotImplementedLinux),
                ("ExpandCollapsePattern_Collapse", NotImplementedLinux),
                ("ExpandCollapsePattern_Expand", NotImplementedLinux),
                ("InvokePattern_Invoke", NotImplementedLinux),
                ("LegacyIAccessiblePattern_DoDefaultAction", NotImplementedLinux),
                ("LegacyIAccessiblePattern_Select", NotImplementedLinux),
                ("LegacyIAccessiblePattern_SetValue", NotImplementedLinux),
                ("MultipleViewPattern_GetViewName", NotImplementedLinux),
                ("MultipleViewPattern_SetCurrentView", NotImplementedLinux),
                ("RangeValuePattern_SetValue", NotImplementedLinux),
                ("RegisterPointerInputTarget", NotImplementedLinux),
                ("RegisterPointerInputTargetEx", NotImplementedLinux),
                ("ScrollItemPattern_ScrollIntoView", NotImplementedLinux),
                ("ScrollPattern_SetScrollPercent", NotImplementedLinux),
                ("SelectionItemPattern_AddToSelection", NotImplementedLinux),
                ("SelectionItemPattern_RemoveFromSelection", NotImplementedLinux),
                ("SelectionItemPattern_Select", NotImplementedLinux),
                ("SynchronizedInputPattern_Cancel", NotImplementedLinux),
                ("TogglePattern_Toggle", NotImplementedLinux),
                ("TransformPattern_Move", NotImplementedLinux),
                ("TransformPattern_Resize", NotImplementedLinux),
                ("TransformPattern_Rotate", NotImplementedLinux),
                ("UiaDisconnectAllProviders", NotImplementedLinux),
                ("UiaGetErrorDescription", NotImplementedLinux),
                ("UiaHPatternObjectFromVariant", NotImplementedLinux),
                ("UiaHasServerSideProvider", NotImplementedLinux),
                ("UiaPatternRelease", NotImplementedLinux),
                ("UnregisterPointerInputTarget", NotImplementedLinux),
                ("UnregisterPointerInputTargetEx", NotImplementedLinux),
                ("ValuePattern_SetValue", NotImplementedLinux),
                ("VirtualizedItemPattern_Realize", NotImplementedLinux),
                ("WindowPattern_Close", NotImplementedLinux),
                ("WindowPattern_WaitForInputIdle", NotImplementedLinux),
            ],
        ),
        (
            "ActiveDirectory",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/ActiveDirectory/mod.rs"
            ),
            vec![],
        ),
        (
            "AddressBook",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/AddressBook/mod.rs"
            ),
            vec![],
        ),
        (
            "AllJoyn",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/AllJoyn/mod.rs"
            ),
            vec![],
        ),
        (
            "Antimalware",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Antimalware/mod.rs"
            ),
            vec![("InstallELAMCertificateInfo", NotImplementedLinux)],
        ),
        (
            "AppLocker",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/AppLocker/mod.rs"
            ),
            vec![("SaferiIsExecutableFileType", NotImplementedLinux)],
        ),
        (
            "ApplicationInstallationAndServicing",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/ApplicationInstallationAndServicing/mod.rs"
            ),
            vec![],
        ),
        (
            "ApplicationVerifier",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/ApplicationVerifier/mod.rs"
            ),
            vec![],
        ),
        (
            "Appx",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/Packaging/Appx/mod.rs"
            ),
            vec![
                ("GetResolvedPackageFullNameForPackageDependency", InvalidNumberOfArguments),
                ("CheckIsMSIXPackage", InvalidNumberOfArguments),
                ("DeactivatePackageVirtualizationContext", NotImplementedWindows),
                ("DeletePackageDependency", NotImplementedWindows),
                ("GetCurrentPackageVirtualizationContext", NotImplementedWindows),
                ("FindPackagesByPackageFamily", NotImplementedLinux),
                ("FormatApplicationUserModelId", NotImplementedLinux),
                ("GetApplicationUserModelId", NotImplementedLinux),
                ("GetApplicationUserModelIdFromToken", NotImplementedLinux),
                ("GetCurrentApplicationUserModelId", NotImplementedLinux),
                ("GetCurrentPackageInfo", NotImplementedLinux),
                ("GetPackageFamilyNameFromToken", NotImplementedLinux),
                ("GetPackageFullNameFromToken", NotImplementedLinux),
                ("GetPackageId", NotImplementedLinux),
                ("GetPackagePathByFullName", NotImplementedLinux),
                ("GetPackagesByPackageFamily", NotImplementedLinux),
                ("GetStagedPackagePathByFullName", NotImplementedLinux),
                ("PackageFamilyNameFromFullName", NotImplementedLinux),
                ("PackageNameAndPublisherIdFromFamilyName", NotImplementedLinux),
                ("ParseApplicationUserModelId", NotImplementedLinux),
                ("VerifyApplicationUserModelId", NotImplementedLinux),
                ("VerifyPackageFamilyName", NotImplementedLinux),
                ("VerifyPackageFullName", NotImplementedLinux),
                ("VerifyPackageRelativeApplicationId", NotImplementedLinux),
            ],
        ),
        (
            "Audio",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Media/Audio/mod.rs"),
            vec![],
        ),
        (
            "Authorization",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Authorization/mod.rs"
            ),
            vec![
                ("AuthzFreeCentralAccessPolicyCache", NotImplementedLinux),
                ("AuthzRegisterSecurityEventSource", NotImplementedLinux),
                ("AuthzUninstallSecurityEventSource", NotImplementedLinux),
                ("AuthzUnregisterSecurityEventSource", NotImplementedLinux),
            ],
        ),
        (
            "Bluetooth",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/Bluetooth/mod.rs"
            ),
            vec![
                ("BluetoothFindRadioClose", CrashesWindows),
                ("BluetoothFindDeviceClose", CrashesWindows),
                ("BluetoothFindNextRadio", CrashesWindows),
                ("BluetoothUnregisterAuthentication", CrashesWindows),
                ("BluetoothEnableDiscovery", NotImplementedLinux),
                ("BluetoothEnableIncomingConnections", NotImplementedLinux),
                ("BluetoothIsConnectable", NotImplementedLinux),
                ("BluetoothIsDiscoverable", NotImplementedLinux),
                ("BluetoothIsVersionAvailable", NotImplementedLinux),
            ],
        ),
        (
            "Cabinets",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/Cabinets/mod.rs"
            ),
            vec![],
        ),
        (
            "CallObj",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Com/CallObj/mod.rs"
            ),
            vec![],
        ),
        (
            "Catalog",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Cryptography/Catalog/mod.rs"
            ),
            vec![
                ("CryptCATAdminReleaseContext", CrashesWindows),
                ("CryptCATAdminReleaseCatalogContext", CrashesWindows),
                ("CryptCATAdminAddCatalog", CrashesWindows),
                ("CryptCATAdminCalcHashFromFileHandle2", CrashesWindows),
                ("CryptCATAdminPauseServiceForBackup", NotImplementedLinux),
                ("CryptCATAdminRemoveCatalog", CrashesLinux),
                ("CryptCATAllocSortedMemberInfo", NotImplementedLinux),
                ("CryptCATStoreFromHandle", NotImplementedLinux),
            ],
        ),
        (
            "Ceip",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Ceip/mod.rs"
            ),
            vec![("CryptCATStoreFromHandle", NotImplementedLinux), ("CeipIsOptedIn", NotImplementedLinux)],
        ),
        (
            "Certificates",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Cryptography/Certificates/mod.rs"
            ),
            vec![],
        ),
        (
            "Clustering",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/Clustering/mod.rs"
            ),
            vec![],
        ),
        (
            "ColorSystem",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/ColorSystem/mod.rs"),
            vec![],
        ),
        (
            "Communication",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/Communication/mod.rs"
            ),
            vec![],
        ),
        (
            "ComponentServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/ComponentServices/mod.rs"
            ),
            vec![("GetManagedExtensions", NotImplementedLinux), ("RecycleSurrogate", NotImplementedLinux)],
        ),
        (
            "CompositionSwapchain",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/CompositionSwapchain/mod.rs"
            ),
            vec![],
        ),
        (
            "Compression",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/Compression/mod.rs"
            ),
            vec![("CloseDecompressor", CrashesWindows), ("ResetDecompressor", CrashesWindows)],
        ),
        (
            "Console",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Console/mod.rs"),
            vec![
                ("FreeConsole", CrashesWindows),
                ("GetConsoleAliasExesA", NotImplementedLinux),
                ("GetConsoleAliasExesW", NotImplementedLinux),
                ("GetConsoleAliasesA", NotImplementedLinux),
                ("GetConsoleAliasesW", NotImplementedLinux),
                ("GetConsoleOriginalTitleA", NotImplementedLinux),
                ("GetConsoleOriginalTitleW", NotImplementedLinux),
                ("GetConsoleTitleA", CrashesLinux),
                ("SetConsoleNumberOfCommandsA", NotImplementedLinux),
                ("SetConsoleNumberOfCommandsW", NotImplementedLinux),
                ("SetConsoleTitleW", CrashesLinux),
            ],
        ),
        (
            "Controls",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/Controls/mod.rs"),
            vec![
                ("GetThemeTransitionDuration", InvalidNumberOfArguments),
                ("GetThemeSysInt", InvalidNumberOfArguments),
                ("GetThemeInt", InvalidNumberOfArguments),
                ("GetThemeEnumValue", InvalidNumberOfArguments),
                ("GetThemeColor", InvalidNumberOfArguments),
                ("GetBufferedPaintTargetRect", InvalidNumberOfArguments),
                ("GetThemeRect", InvalidNumberOfArguments),
                ("GetThemePosition", InvalidNumberOfArguments),
                ("DlgDirSelectComboBoxExA", CrashesWindows),
                ("DlgDirSelectExA", CrashesWindows),
                ("BeginPanningFeedback", NotImplementedLinux),
                ("EndBufferedPaint", CrashesLinux),
                ("EndPanningFeedback", NotImplementedLinux),
                ("GetBufferedPaintDC", CrashesLinux),
                ("GetBufferedPaintTargetDC", CrashesLinux),
                ("GetThemeFilename", CrashesLinux),
                ("GetThemeString", CrashesLinux),
                ("IsThemeBackgroundPartiallyTransparent", CrashesLinux),
                ("IsThemePartDefined", CrashesLinux),
                ("Str_SetPtrW", NotImplementedLinux),
                ("UpdatePanningFeedback", NotImplementedLinux),
            ],
        ),
        (
            "CorrelationVector",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/CorrelationVector/mod.rs"
            ),
            vec![],
        ),
        (
            "Credentials",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Credentials/mod.rs"
            ),
            vec![
                ("CredUIParseUserNameA", CrashesWindows),
                ("CredGetSessionTypes", CrashesWindows),
                ("CredRenameA", NotImplementedLinux),
                ("CredRenameW", NotImplementedLinux),
                ("CredUIConfirmCredentialsA", NotImplementedLinux),
                ("CredUnprotectA", NotImplementedLinux),
                ("CredUnprotectW", NotImplementedLinux),
                ("SCardAudit", NotImplementedLinux),
                ("SCardBeginTransaction", NotImplementedLinux),
                ("SCardConnectA", NotImplementedLinux),
                ("SCardConnectW", NotImplementedLinux),
                ("SCardDisconnect", NotImplementedLinux),
                ("SCardDlgExtendedError", NotImplementedLinux),
                ("SCardEndTransaction", NotImplementedLinux),
                ("SCardForgetCardTypeA", NotImplementedLinux),
                ("SCardForgetCardTypeW", NotImplementedLinux),
                ("SCardForgetReaderA", NotImplementedLinux),
                ("SCardForgetReaderGroupA", NotImplementedLinux),
                ("SCardForgetReaderGroupW", NotImplementedLinux),
                ("SCardForgetReaderW", NotImplementedLinux),
                ("SCardGetAttrib", NotImplementedLinux),
                ("SCardGetCardTypeProviderNameA", NotImplementedLinux),
                ("SCardGetCardTypeProviderNameW", NotImplementedLinux),
                ("SCardGetDeviceTypeIdA", NotImplementedLinux),
                ("SCardGetDeviceTypeIdW", NotImplementedLinux),
                ("SCardGetProviderIdA", NotImplementedLinux),
                ("SCardGetProviderIdW", NotImplementedLinux),
                ("SCardGetReaderDeviceInstanceIdA", NotImplementedLinux),
                ("SCardGetReaderDeviceInstanceIdW", NotImplementedLinux),
                ("SCardGetReaderIconA", NotImplementedLinux),
                ("SCardGetReaderIconW", NotImplementedLinux),
                ("SCardGetTransmitCount", NotImplementedLinux),
                ("SCardIntroduceReaderA", NotImplementedLinux),
                ("SCardIntroduceReaderGroupA", NotImplementedLinux),
                ("SCardIntroduceReaderGroupW", NotImplementedLinux),
                ("SCardIntroduceReaderW", NotImplementedLinux),
                ("SCardListInterfacesA", NotImplementedLinux),
                ("SCardListInterfacesW", NotImplementedLinux),
                ("SCardListReaderGroupsA", NotImplementedLinux),
                ("SCardListReaderGroupsW", NotImplementedLinux),
                ("SCardListReadersWithDeviceInstanceIdA", NotImplementedLinux),
                ("SCardReconnect", NotImplementedLinux),
                ("SCardRemoveReaderFromGroupA", NotImplementedLinux),
                ("SCardRemoveReaderFromGroupW", NotImplementedLinux),
                ("SCardSetCardTypeProviderNameA", NotImplementedLinux),
                ("SCardSetCardTypeProviderNameW", NotImplementedLinux),
                ("SCardState", NotImplementedLinux),
                ("SCardListReadersWithDeviceInstanceIdW", NotImplementedLinux),
            ],
        ),
        (
            "Cryptography",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Cryptography/mod.rs"
            ),
            vec![],
        ),
        (
            "DXCore",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/DXCore/mod.rs"
            ),
            vec![],
        ),
        (
            "DataExchange",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/DataExchange/mod.rs"
            ),
            vec![
                ("GetAtomNameA", CrashesWindows),
                ("GlobalGetAtomNameW", CrashesWindows),
                ("GlobalGetAtomNameA", CrashesWindows),
                ("GetAtomNameW", CrashesWindows),
                ("GlobalAddAtomExA", NotImplementedLinux),
                ("GlobalAddAtomExW", NotImplementedLinux),
            ],
        ),
        (
            "DeveloperLicensing",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/DeveloperLicensing/mod.rs"
            ),
            vec![],
        ),
        (
            "DeviceAccess",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/DeviceAccess/mod.rs"
            ),
            vec![],
        ),
        (
            "DeviceAndDriverInstallation",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/DeviceAndDriverInstallation/mod.rs"
            ),
            vec![
                ("SetupBackupErrorW", ShowsDialogWindows),
                ("SetupDeleteErrorA", ShowsDialogWindows),
                ("SetupDeleteErrorW", ShowsDialogWindows),
                ("InstallHinfSectionW", ShowsDialogWindows),
                ("SetupBackupErrorA", ShowsDialogWindows),
                ("SetupRenameErrorW", ShowsDialogWindows),
                ("SetupRenameErrorA", ShowsDialogWindows),
                ("SetupDiBuildClassInfoList", CrashesWindows),
                ("CM_Add_IDA", NotImplementedLinux),
                ("CM_Add_IDW", NotImplementedLinux),
                ("CM_Add_ID_ExA", NotImplementedLinux),
                ("CM_Add_ID_ExW", NotImplementedLinux),
                ("CM_Add_Range", NotImplementedLinux),
                ("CM_Create_Range_List", NotImplementedLinux),
                ("CM_Delete_DevNode_Key", NotImplementedLinux),
                ("CM_Delete_DevNode_Key_Ex", NotImplementedLinux),
                ("CM_Delete_Device_Interface_KeyA", NotImplementedLinux),
                ("CM_Delete_Device_Interface_KeyW", NotImplementedLinux),
                ("CM_Delete_Device_Interface_Key_ExA", NotImplementedLinux),
                ("CM_Delete_Device_Interface_Key_ExW", NotImplementedLinux),
                ("CM_Delete_Range", NotImplementedLinux),
                ("CM_Disable_DevNode", NotImplementedLinux),
                ("CM_Disable_DevNode_Ex", NotImplementedLinux),
                ("CM_Dup_Range_List", NotImplementedLinux),
                ("CM_Enable_DevNode", NotImplementedLinux),
                ("CM_Enable_DevNode_Ex", NotImplementedLinux),
                ("CM_Enumerate_Classes_Ex", NotImplementedLinux),
                ("CM_Enumerate_EnumeratorsA", NotImplementedLinux),
                ("CM_Enumerate_EnumeratorsW", NotImplementedLinux),
                ("CM_Enumerate_Enumerators_ExA", NotImplementedLinux),
                ("CM_Enumerate_Enumerators_ExW", NotImplementedLinux),
                ("CM_First_Range", NotImplementedLinux),
                ("CM_Free_Log_Conf", NotImplementedLinux),
                ("CM_Free_Log_Conf_Ex", NotImplementedLinux),
                ("CM_Free_Log_Conf_Handle", NotImplementedLinux),
                ("CM_Free_Range_List", NotImplementedLinux),
                ("CM_Free_Res_Des", NotImplementedLinux),
                ("CM_Free_Res_Des_Ex", NotImplementedLinux),
                ("CM_Free_Res_Des_Handle", NotImplementedLinux),
                ("CM_Free_Resource_Conflict_Handle", NotImplementedLinux),
                ("CM_Get_Depth", NotImplementedLinux),
                ("CM_Get_Depth_Ex", NotImplementedLinux),
                ("CM_Get_Device_ID_ListA", NotImplementedLinux),
                ("CM_Get_Device_ID_ListW", NotImplementedLinux),
                ("CM_Get_Device_ID_List_ExA", NotImplementedLinux),
                ("CM_Get_Device_ID_List_ExW", NotImplementedLinux),
                ("CM_Get_Device_ID_List_Size_ExA", NotImplementedLinux),
                ("CM_Get_Device_ID_List_Size_ExW", NotImplementedLinux),
                ("CM_Get_Device_ID_Size_Ex", NotImplementedLinux),
                ("CM_Get_First_Log_Conf", NotImplementedLinux),
                ("CM_Get_First_Log_Conf_Ex", NotImplementedLinux),
                ("CM_Get_Global_State", NotImplementedLinux),
                ("CM_Get_Global_State_Ex", NotImplementedLinux),
                ("CM_Get_Log_Conf_Priority", NotImplementedLinux),
                ("CM_Get_Log_Conf_Priority_Ex", NotImplementedLinux),
                ("CM_Get_Next_Log_Conf", NotImplementedLinux),
                ("CM_Get_Next_Log_Conf_Ex", NotImplementedLinux),
                ("CM_Get_Next_Res_Des", NotImplementedLinux),
                ("CM_Get_Next_Res_Des_Ex", NotImplementedLinux),
                ("CM_Get_Parent_Ex", NotImplementedLinux),
                ("CM_Get_Res_Des_Data_Size", NotImplementedLinux),
                ("CM_Get_Res_Des_Data_Size_Ex", NotImplementedLinux),
                ("CM_Get_Resource_Conflict_Count", NotImplementedLinux),
                ("CM_Get_Version_Ex", NotImplementedLinux),
                ("CM_Intersect_Range_List", NotImplementedLinux),
                ("CM_Invert_Range_List", NotImplementedLinux),
                ("CM_Is_Dock_Station_Present", NotImplementedLinux),
                ("CM_Is_Dock_Station_Present_Ex", NotImplementedLinux),
                ("CM_Is_Version_Available", NotImplementedLinux),
                ("CM_Is_Version_Available_Ex", NotImplementedLinux),
                ("CM_Merge_Range_List", NotImplementedLinux),
                ("CM_Move_DevNode", NotImplementedLinux),
                ("CM_Move_DevNode_Ex", NotImplementedLinux),
                ("CM_Next_Range", NotImplementedLinux),
                ("CM_Open_DevNode_Key_Ex", NotImplementedLinux),
                ("CM_Open_Device_Interface_KeyW", NotImplementedLinux),
                ("CM_Open_Device_Interface_Key_ExA", NotImplementedLinux),
                ("CM_Open_Device_Interface_Key_ExW", NotImplementedLinux),
                ("CM_Query_Arbitrator_Free_Size", NotImplementedLinux),
                ("CM_Query_Arbitrator_Free_Size_Ex", NotImplementedLinux),
                ("CM_Query_Remove_SubTree", NotImplementedLinux),
                ("CM_Query_Remove_SubTree_Ex", NotImplementedLinux),
                ("CM_Register_Device_Driver", NotImplementedLinux),
                ("CM_Register_Device_Driver_Ex", NotImplementedLinux),
                ("CM_Remove_SubTree", NotImplementedLinux),
                ("CM_Remove_SubTree_Ex", NotImplementedLinux),
                ("CM_Request_Eject_PC_Ex", NotImplementedLinux),
                ("CM_Request_Eject_PC", NotImplementedLinux),
                ("CM_Run_Detection", NotImplementedLinux),
                ("CM_Run_Detection_Ex", NotImplementedLinux),
                ("CM_Set_DevNode_Problem", NotImplementedLinux),
                ("CM_Set_DevNode_Problem_Ex", NotImplementedLinux),
                ("CM_Set_HW_Prof", NotImplementedLinux),
                ("CM_Set_HW_Prof_Ex", NotImplementedLinux),
                ("CM_Setup_DevNode", NotImplementedLinux),
                ("CM_Setup_DevNode_Ex", NotImplementedLinux),
                ("CM_Test_Range_Available", NotImplementedLinux),
                ("CM_Uninstall_DevNode", NotImplementedLinux),
                ("CM_Uninstall_DevNode_Ex", NotImplementedLinux),
                ("CM_Unregister_Device_InterfaceA", NotImplementedLinux),
                ("CM_Unregister_Device_InterfaceW", NotImplementedLinux),
                ("CM_Unregister_Device_Interface_ExA", NotImplementedLinux),
                ("CM_Unregister_Device_Interface_ExW", NotImplementedLinux),
                ("DiUninstallDriverA", NotImplementedLinux),
                ("DiShowUpdateDriver", NotImplementedLinux),
                ("DiUninstallDriverW", NotImplementedLinux),
                ("CM_Find_Range", CrashesLinux),
                ("SetupCancelTemporarySourceList", NotImplementedLinux),
                ("SetupDiClassGuidsFromNameW", NotImplementedLinux),
                ("SetupDiGetHwProfileFriendlyNameA", NotImplementedLinux),
                ("SetupDiGetHwProfileFriendlyNameW", NotImplementedLinux),
                ("SetupDiGetHwProfileList", NotImplementedLinux),
                ("SetupGetInfPublishedNameA", NotImplementedLinux),
                ("SetupGetInfPublishedNameW", NotImplementedLinux),
                ("SetupGetThreadLogToken", NotImplementedLinux),
                ("SetupOpenInfFileW", NotImplementedLinux),
                ("SetupOpenInfFileW", CrashesLinux),
                ("SetupRemoveFromSourceListA", NotImplementedLinux),
                ("SetupRemoveFromSourceListW", NotImplementedLinux),
                ("SetupSetPlatformPathOverrideA", NotImplementedLinux),
                ("SetupSetPlatformPathOverrideW", NotImplementedLinux),
                ("SetupSetThreadLogToken", NotImplementedLinux),
                ("SetupWriteTextLog", NotImplementedLinux),
                ("SetupWriteTextLogError", NotImplementedLinux),
                ("UpdateDriverForPlugAndPlayDevicesW", NotImplementedLinux),
                ("CM_Open_Device_Interface_KeyA", NotImplementedLinux),
            ],
        ),
        (
            "DeviceQuery",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/DeviceQuery/mod.rs"
            ),
            vec![],
        ),
        (
            "Dhcp",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/Dhcp/mod.rs"
            ),
            vec![],
        ),
        (
            "DiagnosticDataQuery",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/DiagnosticDataQuery/mod.rs"
            ),
            vec![
                ("DdqGetDiagnosticRecordPayload", InvalidNumberOfArguments),
                ("DdqGetDiagnosticReportStoreReportCount", InvalidNumberOfArguments),
            ],
        ),
        (
            "Diagnostics",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Xaml/Diagnostics/mod.rs"
            ),
            vec![],
        ),
        (
            "Dialogs",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Controls/Dialogs/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct2D",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct2D/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D10",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D10/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D11",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/WinRT/Direct3D11/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D112",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D11/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D11on12",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D11on12/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D12",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D12/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D9",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D9/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D9on12",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D9on12/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectComposition",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/DirectComposition/mod.rs"
            ),
            vec![("DCompositionBoostCompositorClock", NotImplementedWindows)],
        ),
        (
            "DirectDraw",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/DirectDraw/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectML",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/AI/MachineLearning/DirectML/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectShow",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/DirectShow/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectSound",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/Audio/DirectSound/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectWrite",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/DirectWrite/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectoryServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/DirectoryServices/mod.rs"
            ),
            vec![],
        ),
        (
            "Display",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/Display/mod.rs"
            ),
            vec![
                ("EngUnicodeToMultiByteN", CrashesWindows),
                ("EngMultiByteToUnicodeN", CrashesWindows),
                ("EngCheckAbort", NotImplementedLinux),
                ("EngComputeGlyphSet", NotImplementedLinux),
                ("EngCreateClip", NotImplementedLinux),
                ("EngCreatePalette", NotImplementedLinux),
                ("EngCreateSemaphore", NotImplementedLinux),
                ("EngFindResource", NotImplementedLinux),
                ("EngFreeModule", NotImplementedLinux),
                ("EngGetCurrentCodePage", NotImplementedLinux),
                ("EngMultiByteToWideChar", NotImplementedLinux),
                ("EngLoadModule", NotImplementedLinux),
                ("EngUnlockSurface", NotImplementedLinux),
                ("EngWideCharToMultiByte", NotImplementedLinux),
            ],
        ),
        (
            "DistributedFileSystem",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/DistributedFileSystem/mod.rs"
            ),
            vec![
                ("NetDfsRemove", NotImplementedLinux),
                ("NetDfsMove", NotImplementedLinux),
                ("NetDfsAdd", NotImplementedLinux),
                ("NetDfsAddFtRoot", NotImplementedLinux),
                ("NetDfsAddStdRoot", NotImplementedLinux),
                ("NetDfsAddRootTarget", NotImplementedLinux),
                ("NetDfsRemoveFtRoot", NotImplementedLinux),
                ("NetDfsRemoveFtRootForced", NotImplementedLinux),
                ("NetDfsRemoveRootTarget", NotImplementedLinux),
                ("NetDfsRemoveStdRoot", NotImplementedLinux),
            ],
        ),
        (
            "DistributedTransactionCoordinator",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/DistributedTransactionCoordinator/mod.rs"
            ),
            vec![],
        ),
        (
            "Dns",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/Dns/mod.rs"
            ),
            vec![("DnsFreeProxyName", NotImplementedLinux)],
        ),
        (
            "DxMediaObjects",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/DxMediaObjects/mod.rs"
            ),
            vec![],
        ),
        (
            "Dxc",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D/Dxc/mod.rs"
            ),
            vec![],
        ),
        (
            "EnterpriseData",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/EnterpriseData/mod.rs"
            ),
            vec![],
        ),
        (
            "Environment",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Environment/mod.rs"
            ),
            vec![
                ("ExpandEnvironmentStringsW", CrashesWindows),
                ("SetEnvironmentStringsW", CrashesWindows),
                ("NeedCurrentDirectoryForExePathW", CrashesWindows),
                ("GetCurrentDirectoryA", CrashesWindows),
                ("NeedCurrentDirectoryForExePathA", CrashesWindows),
                ("SetEnvironmentVariableW", CrashesWindows),
                ("GetCurrentDirectoryW", CrashesWindows),
                ("ExpandEnvironmentStringsA", CrashesWindows),
                ("ExpandEnvironmentStringsForUserW", CrashesWindows),
                ("IsEnclaveTypeSupported", NotImplementedLinux),
            ],
        ),
        (
            "ErrorReporting",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/ErrorReporting/mod.rs"
            ),
            vec![
                ("WerRegisterAppLocalDump", CrashesWindows),
                ("WerUnregisterCustomMetadata", NotImplementedLinux),
                ("WerUnregisterAppLocalDump", NotImplementedLinux),
                ("WerUnregisterAdditionalProcess", NotImplementedLinux),
                ("WerStorePurge", NotImplementedLinux),
                ("WerFreeString", NotImplementedLinux),
                ("WerRegisterAdditionalProcess", NotImplementedLinux),
                ("WerRegisterCustomMetadata", NotImplementedLinux),
                ("WerReportHang", NotImplementedLinux),
                ("AddERExcludedApplicationW", CrashesLinux),
            ],
        ),
        (
            "Etw",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Etw/mod.rs"
            ),
            vec![("CveEventWrite", NotImplementedLinux), ("TdhUnloadManifest", NotImplementedLinux)],
        ),
        (
            "EventCollector",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/EventCollector/mod.rs"
            ),
            vec![],
        ),
        (
            "EventLog",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/EventLog/mod.rs"
            ),
            vec![
                ("EvtCreateBookmark", NotImplementedLinux),
                ("EvtArchiveExportedLog", NotImplementedLinux),
                ("EvtCancel", NotImplementedLinux),
                ("EvtClearLog", NotImplementedLinux),
                ("EvtGetExtendedStatus", NotImplementedLinux),
                ("EvtGetObjectArraySize", NotImplementedLinux),
                ("EvtNextEventMetadata", NotImplementedLinux),
                ("EvtNextPublisherId", NotImplementedLinux),
                ("EvtOpenEventMetadataEnum", NotImplementedLinux),
                ("EvtOpenPublisherEnum", NotImplementedLinux),
                ("EvtOpenPublisherMetadata", NotImplementedLinux),
                ("EvtSeek", NotImplementedLinux),
                ("EvtUpdateBookmark", NotImplementedLinux),
            ],
        ),
        (
            "EventNotificationService",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/EventNotificationService/mod.rs"
            ),
            vec![],
        ),
        (
            "ExtensibleAuthenticationProtocol",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/ExtensibleAuthenticationProtocol/mod.rs"
            ),
            vec![],
        ),
        (
            "Fax",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Devices/Fax/mod.rs"),
            vec![],
        ),
        (
            "FileHistory",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/FileHistory/mod.rs"
            ),
            vec![("FhServiceOpenPipe", InvalidNumberOfArguments)],
        ),
        (
            "Foundation",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Foundation/mod.rs"),
            vec![],
        ),
        (
            "Fxc",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D/Fxc/mod.rs"
            ),
            vec![],
        ),
        (
            "Gdi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Graphics/Gdi/mod.rs"),
            vec![
                ("GetTextExtentExPointA", CrashesLinux),
                ("TTEnableEmbeddingForFacename", CrashesLinux),
                ("TransparentBlt", NotImplementedWindows),
                ("MapWindowPoints", CrashesWindows),
                ("GetEnhMetaFileW", CrashesWindows),
                ("GetTextExtentPointW", CrashesWindows),
                ("AddFontResourceW", CrashesWindows),
                ("GetTextExtentPoint32W", CrashesWindows),
                ("GetEnhMetaFileA", CrashesWindows),
                ("GetMetaFileW", CrashesWindows),
                ("GetMetaFileA", CrashesWindows),
            ],
        ),
        (
            "Globalization",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Globalization/mod.rs"),
            vec![("GetDistanceOfClosestLanguageInList", InvalidNumberOfArguments)],
        ),
        (
            "GroupPolicy",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/GroupPolicy/mod.rs"
            ),
            vec![],
        ),
        (
            "HardwareCounterProfiling",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Performance/HardwareCounterProfiling/mod.rs"
            ),
            vec![
                ("DisableThreadProfiling", NotImplementedLinux),
                ("EnableThreadProfiling", NotImplementedLinux),
                ("QueryThreadProfiling", NotImplementedLinux),
            ],
        ),
        (
            "HiDpi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/HiDpi/mod.rs"),
            vec![
                ("GetDialogControlDpiChangeBehavior", NotImplementedLinux),
                ("GetDialogDpiChangeBehavior", NotImplementedLinux),
                ("GetDpiAwarenessContextForProcess", NotImplementedLinux),
                ("GetSystemDpiForProcess", NotImplementedLinux),
                ("GetThreadDpiHostingBehavior", NotImplementedLinux),
                ("GetWindowDpiHostingBehavior", NotImplementedLinux),
            ],
        ),
        (
            "HostComputeNetwork",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/HostComputeNetwork/mod.rs"
            ),
            vec![],
        ),
        (
            "HostComputeSystem",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/HostComputeSystem/mod.rs"
            ),
            vec![
                ("HcsModifyServiceSettings", InvalidNumberOfArguments),
                ("HcsGetServiceProperties", InvalidNumberOfArguments),
                ("HcsGetProcessorCompatibilityFromSavedState", InvalidNumberOfArguments),
                ("HcsGetLayerVhdMountPath", InvalidNumberOfArguments),
            ],
        ),
        (
            "HttpServer",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/HttpServer/mod.rs"
            ),
            vec![
                ("HttpAddUrlToUrlGroup", NotImplementedLinux),
                ("HttpFindUrlGroupId", NotImplementedLinux),
                ("HttpShutdownRequestQueue", NotImplementedLinux),
                ("HttpRemoveUrlFromUrlGroup", CrashesLinux),
            ],
        ),
        (
            "HumanInterfaceDevice",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/HumanInterfaceDevice/mod.rs"
            ),
            vec![("HidD_FreePreparsedData", CrashesWindows)],
        ),
        (
            "Hypervisor",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Hypervisor/mod.rs"
            ),
            vec![],
        ),
        (
            "IO",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/IO/mod.rs"),
            vec![],
        ),
        (
            "Iis",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Iis/mod.rs"),
            vec![],
        ),
        (
            "Imaging",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/Imaging/mod.rs"
            ),
            vec![("WICMapShortNameToGuid", InvalidNumberOfArguments)],
        ),
        (
            "Imapi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Storage/Imapi/mod.rs"),
            vec![],
        ),
        (
            "Ime",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/Input/Ime/mod.rs"),
            vec![
                ("ImmInstallIMEA", CrashesWindows),
                ("ImmInstallIMEW", NotImplementedLinux),
                ("ImmSetHotKey", NotImplementedLinux),
            ],
        ),
        (
            "IndexServer",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/IndexServer/mod.rs"
            ),
            vec![],
        ),
        (
            "Input",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/Input/mod.rs"),
            vec![],
        ),
        (
            "InstallableFileSystems",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/InstallableFileSystems/mod.rs"
            ),
            vec![
                ("FilterAttach", NotImplementedWindows),
                ("FilterAttachAtAltitude", NotImplementedWindows),
                ("FilterDetach", NotImplementedWindows),
                ("FilterFindClose", NotImplementedWindows),
                ("FilterGetDosName", NotImplementedWindows),
                ("FilterInstanceFindClose", NotImplementedWindows),
                ("FilterLoad", NotImplementedWindows),
                ("FilterUnload", NotImplementedWindows),
                ("FilterVolumeFindClose", NotImplementedWindows),
                ("FilterVolumeInstanceFindClose", NotImplementedWindows),
            ],
        ),
        (
            "InteractionContext",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/InteractionContext/mod.rs"
            ),
            vec![
                ("CreateInteractionContext", InvalidNumberOfArguments),
                ("AddPointerInteractionContext", NotImplementedLinux),
                ("ProcessBufferedPacketsInteractionContext", NotImplementedLinux),
                ("RemovePointerInteractionContext", NotImplementedLinux),
                ("ResetInteractionContext", NotImplementedLinux),
                ("SetPivotInteractionContext", NotImplementedLinux),
                ("StopInteractionContext", NotImplementedLinux),
            ],
        ),
        (
            "IpHelper",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/IpHelper/mod.rs"
            ),
            vec![
                ("GetIpErrorString", CrashesWindows),
                ("ConvertCompartmentIdToGuid", NotImplementedLinux),
                ("ConvertIpv4MaskToLength", NotImplementedLinux),
                ("CreatePersistentTcpPortReservation", NotImplementedLinux),
                ("CreatePersistentUdpPortReservation", NotImplementedLinux),
                ("DeletePersistentTcpPortReservation", NotImplementedLinux),
                ("DeletePersistentUdpPortReservation", NotImplementedLinux),
                ("FlushIpNetTable2", NotImplementedLinux),
                ("FlushIpPathTable", NotImplementedLinux),
                ("GetAdapterOrderMap", NotImplementedLinux),
                ("GetCurrentThreadCompartmentId", NotImplementedLinux),
                ("GetCurrentThreadCompartmentScope", NotImplementedLinux),
                ("GetDefaultCompartmentId", NotImplementedLinux),
                ("GetJobCompartmentId", NotImplementedLinux),
                ("GetSessionCompartmentId", NotImplementedLinux),
                ("GetTeredoPort", NotImplementedLinux),
                ("LookupPersistentTcpPortReservation", NotImplementedLinux),
                ("LookupPersistentUdpPortReservation", NotImplementedLinux),
                ("PfDeleteLog", NotImplementedLinux),
                ("PfMakeLog", NotImplementedLinux),
                ("PfSetLogBuffer", NotImplementedLinux),
                ("SetCurrentThreadCompartmentId", NotImplementedLinux),
                ("SetCurrentThreadCompartmentScope", NotImplementedLinux),
                ("SetJobCompartmentId", NotImplementedLinux),
                ("SetSessionCompartmentId", NotImplementedLinux),
                ("GetAdapterIndex", CrashesLinux),
            ],
        ),
        (
            "IscsiDisc",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/IscsiDisc/mod.rs"
            ),
            vec![],
        ),
        (
            "Isolation",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Isolation/mod.rs"
            ),
            vec![
                ("IsProcessInIsolatedWindowsEnvironment", InvalidNumberOfArguments),
                ("IsProcessInIsolatedContainer", InvalidNumberOfArguments),
                ("GetAppContainerFolderPath", InvalidNumberOfArguments),
                ("DeriveAppContainerSidFromAppContainerName", InvalidNumberOfArguments),
                (
                    "DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName",
                    InvalidNumberOfArguments,
                ),
                ("GetAppContainerRegistryLocation", InvalidNumberOfArguments),
                ("DeleteAppContainerProfile", NotImplementedLinux),
                ("GetAppContainerNamedObjectPath", NotImplementedLinux),
            ],
        ),
        (
            "Jet",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Storage/Jet/mod.rs"),
            vec![
                ("JetBeginExternalBackup", NotImplementedLinux),
                ("JetBeginExternalBackupInstance", NotImplementedLinux),
                ("JetBeginTransaction", NotImplementedLinux),
                ("JetBeginTransaction2", NotImplementedLinux),
                ("JetBeginTransaction3", NotImplementedLinux),
                ("JetCloseDatabase", NotImplementedLinux),
                ("JetCloseTable", NotImplementedLinux),
                ("JetCommitTransaction", NotImplementedLinux),
                ("JetComputeStats", NotImplementedLinux),
                ("JetConfigureProcessForCrashDump", NotImplementedLinux),
                ("JetDelete", NotImplementedLinux),
                ("JetDupCursor", NotImplementedLinux),
                ("JetDupSession", NotImplementedLinux),
                ("JetEndExternalBackup", NotImplementedLinux),
                ("JetEndExternalBackupInstance", NotImplementedLinux),
                ("JetEndExternalBackupInstance2", NotImplementedLinux),
                ("JetEndSession", NotImplementedLinux),
                ("JetFreeBuffer", NotImplementedLinux),
                ("JetGetAttachInfoA", NotImplementedLinux),
                ("JetGetAttachInfoInstanceA", NotImplementedLinux),
                ("JetGetAttachInfoInstanceW", NotImplementedLinux),
                ("JetGetAttachInfoW", NotImplementedLinux),
                ("JetGetCurrentIndexA", NotImplementedLinux),
                ("JetGetCurrentIndexW", NotImplementedLinux),
                ("JetGetLock", NotImplementedLinux),
                ("JetGetLogInfoA", NotImplementedLinux),
                ("JetGetLogInfoInstanceA", NotImplementedLinux),
                ("JetGetLogInfoInstanceW", NotImplementedLinux),
                ("JetGetLogInfoW", NotImplementedLinux),
                ("JetGetTruncateLogInfoInstanceA", NotImplementedLinux),
                ("JetGetTruncateLogInfoInstanceW", NotImplementedLinux),
                ("JetGetVersion", NotImplementedLinux),
                ("JetIdle", NotImplementedLinux),
                ("JetInit", NotImplementedLinux),
                ("JetIndexRecordCount", NotImplementedLinux),
                ("JetMove", NotImplementedLinux),
                ("JetInit2", NotImplementedLinux),
                ("JetPrepareUpdate", NotImplementedLinux),
                ("JetResetSessionContext", NotImplementedLinux),
                ("JetResetTableSequential", NotImplementedLinux),
                ("JetResizeDatabase", NotImplementedLinux),
                ("JetRollback", NotImplementedLinux),
                ("JetSeek", NotImplementedLinux),
                ("JetSetIndexRange", NotImplementedLinux),
                ("JetSetTableSequential", NotImplementedLinux),
                ("JetStopBackup", NotImplementedLinux),
                ("JetStopBackupInstance", NotImplementedLinux),
                ("JetStopService", NotImplementedLinux),
                ("JetStopServiceInstance", NotImplementedLinux),
                ("JetStopServiceInstance2", NotImplementedLinux),
                ("JetTerm", NotImplementedLinux),
                ("JetTerm2", NotImplementedLinux),
                ("JetTruncateLog", NotImplementedLinux),
                ("JetTruncateLogInstance", NotImplementedLinux),
            ],
        ),
        (
            "JobObjects",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/JobObjects/mod.rs"
            ),
            vec![],
        ),
        (
            "Js",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Js/mod.rs"),
            vec![],
        ),
        (
            "Kernel",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Kernel/mod.rs"),
            vec![],
        ),
        (
            "KernelStreaming",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/KernelStreaming/mod.rs"
            ),
            vec![],
        ),
        (
            "KeyboardAndMouse",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Input/KeyboardAndMouse/mod.rs"
            ),
            vec![("GetKeyNameTextA", CrashesWindows), ("GetKeyboardLayoutNameA", CrashesWindows)],
        ),
        (
            "Ldap",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/Ldap/mod.rs"
            ),
            vec![
                ("ldap_initW", CrashesWindows),
                ("ldap_err2stringW", CrashesWindows),
                ("ldap_init", CrashesWindows),
                ("cldap_openW", CrashesWindows),
                ("ldap_sslinitA", CrashesWindows),
                ("cldap_open", CrashesWindows),
                ("ldap_err2stringA", CrashesWindows),
                ("ldap_sslinit", CrashesWindows),
                ("ldap_openW", CrashesWindows),
                ("cldap_openA", CrashesWindows),
                ("ldap_sslinitW", CrashesWindows),
                ("ldap_openA", CrashesWindows),
                ("ldap_initA", CrashesWindows),
                ("ldap_err2string", CrashesWindows),
                ("ldap_open", CrashesWindows),
                ("ldap_compare_s", CrashesLinux),
                ("ldap_compare_sA", CrashesLinux),
                ("ldap_compare_sW", CrashesLinux),
                ("ldap_delete", CrashesLinux),
                ("ldap_deleteA", CrashesLinux),
                ("ldap_deleteW", CrashesLinux),
                ("ldap_delete_s", CrashesLinux),
                ("ldap_delete_sA", CrashesLinux),
                ("ldap_delete_sW", CrashesLinux),
                ("ldap_simple_bind", CrashesLinux),
                ("ldap_simple_bindA", CrashesLinux),
                ("ldap_simple_bindW", CrashesLinux),
                ("ldap_simple_bind_s", CrashesLinux),
                ("ldap_simple_bind_sA", CrashesLinux),
                ("ldap_simple_bind_sW", CrashesLinux),
                ("ldap_unbind", CrashesLinux),
                ("ldap_unbind_s", CrashesLinux),
                ("ldap_set_dbg_flags", NotImplementedLinux),
                ("McastApiStartup", NotImplementedLinux),
                ("McastApiCleanup", NotImplementedLinux),
            ],
        ),
        (
            "LibraryLoader",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/LibraryLoader/mod.rs"
            ),
            vec![
                ("FreeLibraryAndExitThread", CrashAutomatic),
                ("GetDllDirectoryA", CrashesWindows),
                ("GetDllDirectoryW", CrashesWindows),
                ("GetModuleFileNameA", CrashesWindows),
                ("AddDllDirectory", CrashesWindows),
                ("GetModuleFileNameW", CrashesWindows),
                ("BeginUpdateResourceW", CrashesLinux),
            ],
        ),
        (
            "LicenseProtection",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/LicenseProtection/mod.rs"
            ),
            vec![],
        ),
        (
            "Magnification",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Magnification/mod.rs"
            ),
            vec![],
        ),
        (
            "Mailslots",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Mailslots/mod.rs"
            ),
            vec![],
        ),
        (
            "Mapi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Mapi/mod.rs"),
            vec![],
        ),
        (
            "Marshal",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Com/Marshal/mod.rs"
            ),
            vec![],
        ),
        (
            "Media",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Media/mod.rs"),
            vec![],
        ),
        (
            "Memory",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Memory/mod.rs"),
            vec![],
        ),
        (
            "MobileDeviceManagementRegistration",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Management/MobileDeviceManagementRegistration/mod.rs"
            ),
            vec![
                ("RegisterDeviceWithLocalManagement", InvalidNumberOfArguments),
                ("IsMdmUxWithoutAadAllowed", InvalidNumberOfArguments),
                ("IsManagementRegistrationAllowed", InvalidNumberOfArguments),
                ("ApplyLocalManagementSyncML", InvalidNumberOfArguments),
            ],
        ),
        (
            "MsHtml",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Web/MsHtml/mod.rs"),
            vec![
                ("RatingObtainQueryW", InvalidNumberOfArguments),
                ("RatingObtainQuery", InvalidNumberOfArguments),
            ],
        ),
        (
            "Multicast",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/Multicast/mod.rs"
            ),
            vec![("McastApiStartup", NotImplementedLinux), ("McastApiCleanup", NotImplementedLinux)],
        ),
        (
            "Multimedia",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/Multimedia/mod.rs"
            ),
            vec![
                ("DrawDibStart", CrashesWindows),
                ("ICInstall", CrashesWindows),
                ("mmioStringToFOURCCW", CrashesWindows),
                ("mmTaskBlock", Freeze),
                ("AVIBuildFilterW", CrashesWindows),
                ("mciGetDeviceIDA", CrashesWindows),
                ("DrawDibGetPalette", CrashesWindows),
                ("AVIBuildFilterA", CrashesWindows),
                ("DrawDibStop", CrashesWindows),
                ("DrawDibClose", CrashesWindows),
                ("DrawDibEnd", CrashesWindows),
                ("mciGetDeviceIDFromElementIDA", CrashesWindows),
                ("mciSendStringA", CrashesWindows),
                ("mciSendStringW", CrashesWindows),
                ("MCIWndCreateW", ShowsDialogWindows),
                ("MCIWndCreateA", ShowsDialogWindows),
                ("mmioStringToFOURCCA", CrashesLinux),
                ("sndOpenSound", NotImplementedLinux),
                ("OpenDriver", CrashesLinux),
                ("sndOpenSound", NotImplementedLinux),
            ],
        ),
        (
            "NetBios",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetBios/mod.rs"
            ),
            vec![],
        ),
        (
            "NetManagement",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetManagement/mod.rs"
            ),
            vec![
                ("RouterAssert", CrashesWindows),
                ("NetScheduleJobAdd", CrashesWindows),
                ("NetAddServiceAccount", NotImplementedLinux),
                ("NetAddAlternateComputerName", NotImplementedLinux),
                ("NetAccessGetUserPerms", NotImplementedLinux),
                ("NetAccessDel", NotImplementedLinux),
                ("GetNetScheduleAccountInformation", NotImplementedLinux),
                ("GetNetScheduleAccountInformation", NotImplementedLinux),
                ("NetAccessDel", NotImplementedLinux),
                ("NetAccessGetUserPerms", NotImplementedLinux),
                ("NetAddAlternateComputerName", NotImplementedLinux),
                ("NetAddServiceAccount", NotImplementedLinux),
                ("NetAuditClear", NotImplementedLinux),
                ("NetAuditWrite", NotImplementedLinux),
                ("NetConfigSet", NotImplementedLinux),
                ("NetGetDisplayInformationIndex", NotImplementedLinux),
                ("NetGroupDel", NotImplementedLinux),
                ("NetGroupDelUser", NotImplementedLinux),
                ("NetIsServiceAccount", NotImplementedLinux),
                ("NetMessageNameAdd", NotImplementedLinux),
                ("NetMessageNameDel", NotImplementedLinux),
                ("NetRemoveAlternateComputerName", NotImplementedLinux),
                ("NetRemoveServiceAccount", NotImplementedLinux),
                ("NetRenameMachineInDomain", NotImplementedLinux),
                ("NetReplExportDirDel", NotImplementedLinux),
                ("NetReplExportDirLock", NotImplementedLinux),
                ("NetReplExportDirUnlock", NotImplementedLinux),
                ("NetReplImportDirDel", NotImplementedLinux),
                ("NetReplImportDirLock", NotImplementedLinux),
                ("NetReplImportDirUnlock", NotImplementedLinux),
                ("NetScheduleJobDel", NotImplementedLinux),
                ("NetServerComputerNameAdd", NotImplementedLinux),
                ("NetServerComputerNameDel", NotImplementedLinux),
                ("NetSetPrimaryComputerName", NotImplementedLinux),
                ("NetUnjoinDomain", NotImplementedLinux),
                ("RouterGetErrorStringA", NotImplementedLinux),
                ("RouterGetErrorStringW", NotImplementedLinux),
                ("RouterLogDeregisterA", NotImplementedLinux),
                ("RouterLogDeregisterW", NotImplementedLinux),
                ("RouterLogEventExA", NotImplementedLinux),
                ("RouterLogEventExW", NotImplementedLinux),
                ("RouterLogEventValistExA", NotImplementedLinux),
                ("RouterLogEventValistExW", NotImplementedLinux),
                ("RouterLogRegisterA", NotImplementedLinux),
                ("RouterLogRegisterW", NotImplementedLinux),
                ("SetNetScheduleAccountInformation", NotImplementedLinux),
                ("TraceDeregisterA", NotImplementedLinux),
                ("TraceDeregisterExA", NotImplementedLinux),
                ("TraceDeregisterExW", NotImplementedLinux),
                ("TraceDeregisterW", NotImplementedLinux),
                ("TraceDumpExA", NotImplementedLinux),
                ("TraceDumpExW", NotImplementedLinux),
                ("TraceGetConsoleA", NotImplementedLinux),
                ("TraceGetConsoleW", NotImplementedLinux),
                ("TracePrintfA", NotImplementedLinux),
                ("TracePrintfExA", NotImplementedLinux),
                ("TracePrintfExW", NotImplementedLinux),
                ("TracePrintfW", NotImplementedLinux),
                ("TracePutsExA", NotImplementedLinux),
                ("TracePutsExW", NotImplementedLinux),
                ("TraceVprintfExA", NotImplementedLinux),
                ("TraceVprintfExW", NotImplementedLinux),
            ],
        ),
        (
            "NetShell",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetShell/mod.rs"
            ),
            vec![],
        ),
        (
            "NetworkDiagnosticsFramework",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetworkDiagnosticsFramework/mod.rs"
            ),
            vec![],
        ),
        (
            "NonVolatile",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Memory/NonVolatile/mod.rs"
            ),
            vec![],
        ),
        (
            "OfflineFiles",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/OfflineFiles/mod.rs"
            ),
            vec![],
        ),
        (
            "OpenGL",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Graphics/OpenGL/mod.rs"
            ),
            vec![],
        ),
        (
            "OperationRecorder",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/OperationRecorder/mod.rs"
            ),
            vec![],
        ),
        (
            "PasswordManagement",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/PasswordManagement/mod.rs"
            ),
            vec![],
        ),
        (
            "Pdf",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/WinRT/Pdf/mod.rs"
            ),
            vec![],
        ),
        (
            "Performance",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Performance/mod.rs"
            ),
            vec![
                ("PerfEnumerateCounterSet", CrashesWindows),
                ("UpdatePerfNameFilesW", NotImplementedLinux),
                ("UpdatePerfNameFilesA", NotImplementedLinux),
                ("SetServiceAsTrustedW", NotImplementedLinux),
                ("SetServiceAsTrustedA", NotImplementedLinux),
                ("RestorePerfRegistryFromFileW", NotImplementedLinux),
                ("PerfCloseQueryHandle", NotImplementedLinux),
                ("PdhVerifySQLDBW", NotImplementedLinux),
                ("PdhVerifySQLDBA", NotImplementedLinux),
                ("PdhUpdateLogW", NotImplementedLinux),
                ("PdhUpdateLogFileCatalog", NotImplementedLinux),
                ("PdhUpdateLogA", NotImplementedLinux),
                ("PdhSetLogSetRunID", NotImplementedLinux),
                ("PdhParseInstanceNameW", NotImplementedLinux),
                ("PdhParseInstanceNameA", NotImplementedLinux),
                ("PdhOpenQueryH", NotImplementedLinux),
                ("PdhIsRealTimeQuery", NotImplementedLinux),
                ("PdhGetLogSetGUID", NotImplementedLinux),
                ("PdhGetLogFileSize", NotImplementedLinux),
                ("PdhGetDefaultPerfObjectW", NotImplementedLinux),
                ("PdhGetDefaultPerfObjectHW", NotImplementedLinux),
                ("PdhGetDefaultPerfObjectHA", NotImplementedLinux),
                ("PdhGetDefaultPerfObjectA", NotImplementedLinux),
                ("PdhGetDefaultPerfCounterW", NotImplementedLinux),
                ("PdhGetDefaultPerfCounterHW", NotImplementedLinux),
                ("PdhGetDefaultPerfCounterHA", NotImplementedLinux),
                ("PdhGetDefaultPerfCounterA", NotImplementedLinux),
                ("PdhExpandWildCardPathHW", NotImplementedLinux),
                ("PdhExpandWildCardPathHA", NotImplementedLinux),
                ("PdhEnumObjectsW", NotImplementedLinux),
                ("PdhEnumObjectsHW", NotImplementedLinux),
                ("PdhEnumObjectsHA", NotImplementedLinux),
                ("PdhEnumObjectsA", NotImplementedLinux),
                ("PdhEnumObjectItemsHW", NotImplementedLinux),
                ("PdhEnumObjectItemsHA", NotImplementedLinux),
                ("PdhEnumMachinesW", NotImplementedLinux),
                ("PdhEnumMachinesHW", NotImplementedLinux),
                ("PdhEnumMachinesHA", NotImplementedLinux),
                ("PdhEnumMachinesA", NotImplementedLinux),
                ("PdhEnumLogSetNamesW", NotImplementedLinux),
                ("PdhEnumLogSetNamesA", NotImplementedLinux),
                ("PdhCreateSQLTablesW", NotImplementedLinux),
                ("PdhCreateSQLTablesA", NotImplementedLinux),
                ("PdhConnectMachineW", NotImplementedLinux),
                ("PdhConnectMachineA", NotImplementedLinux),
                ("PdhCloseLog", NotImplementedLinux),
                ("BackupPerfRegistryToFileW", NotImplementedLinux),
                ("PdhCloseQuery", CrashesLinux),
                ("PdhCollectQueryData", CrashesLinux),
                ("PdhCollectQueryDataEx", CrashesLinux),
                ("PdhCollectQueryDataWithTime", CrashesLinux),
                ("PdhGetCounterTimeBase", CrashesLinux),
                ("PdhRemoveCounter", CrashesLinux),
                ("PdhSetCounterScaleFactor", CrashesLinux),
            ],
        ),
        (
            "Pipes",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Pipes/mod.rs"),
            vec![
                ("WaitNamedPipeW", CrashesWindows),
                ("GetNamedPipeClientComputerNameW", NotImplementedLinux),
                ("GetNamedPipeClientComputerNameA", NotImplementedLinux),
            ],
        ),
        (
            "Pnp",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/Enumeration/Pnp/mod.rs"
            ),
            vec![],
        ),
        (
            "Pointer",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Input/Pointer/mod.rs"
            ),
            vec![
                ("SkipPointerFrameMessages", NotImplementedLinux),
                ("IsMouseInPointerEnabled", NotImplementedLinux),
                ("GetUnpredictedMessagePos", NotImplementedLinux),
                ("GetPointerCursorId", NotImplementedLinux),
            ],
        ),
        (
            "PortableDevices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/PortableDevices/mod.rs"
            ),
            vec![],
        ),
        (
            "Power",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Power/mod.rs"),
            vec![
                ("PowerRestoreDefaultPowerSchemes", NotImplementedLinux),
                ("RegisterSuspendResumeNotification", NotImplementedLinux),
                ("PowerReplaceDefaultPowerSchemes", NotImplementedLinux),
                ("PowerOpenUserPowerKey", NotImplementedLinux),
                ("PowerOpenSystemPowerKey", NotImplementedLinux),
                ("DevicePowerOpen", NotImplementedLinux),
                ("DevicePowerEnumDevices", NotImplementedLinux),
                ("DevicePowerClose", NotImplementedLinux),
            ],
        ),
        (
            "ProcessSnapshotting",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Diagnostics/ProcessSnapshotting/mod.rs"
            ),
            vec![],
        ),
        (
            "ProcessStatus",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/ProcessStatus/mod.rs"
            ),
            vec![("K32EnumProcesses", CrashesWindows)],
        ),
        (
            "ProjectedFileSystem",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Storage/ProjectedFileSystem/mod.rs"
            ),
            vec![],
        ),
        (
            "PropertiesSystem",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Shell/PropertiesSystem/mod.rs"
            ),
            vec![
                ("PSLookupPropertyHandlerCLSID", InvalidNumberOfArguments),
                ("InitPropVariantFromResource", InvalidNumberOfArguments),
                ("InitPropVariantFromStringAsVector", InvalidNumberOfArguments),
                ("InitVariantFromResource", InvalidNumberOfArguments),
                ("ClearPropVariantArray", CrashesWindows),
                ("ClearVariantArray", CrashesWindows),
                ("PifMgr_OpenProperties", NotImplementedLinux),
                ("PifMgr_CloseProperties", NotImplementedLinux),
            ],
        ),
        (
            "QoS",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/QoS/mod.rs"
            ),
            vec![
                ("TcOpenInterfaceW", NotImplementedLinux),
                ("TcOpenInterfaceA", NotImplementedLinux),
                ("TcGetFlowNameW", NotImplementedLinux),
                ("TcGetFlowNameA", NotImplementedLinux),
                ("TcDeleteFlow", NotImplementedLinux),
                ("TcDeleteFilter", NotImplementedLinux),
                ("TcCloseInterface", NotImplementedLinux),
                ("QOSCloseHandle", NotImplementedLinux),
            ],
        ),
        (
            "Recovery",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Recovery/mod.rs"
            ),
            vec![
                ("ApplicationRecoveryInProgress", InvalidNumberOfArguments),
                ("UnregisterApplicationRecoveryCallback", NotImplementedLinux),
            ],
        ),
        (
            "Registry",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Registry/mod.rs"
            ),
            vec![
                ("RegDisablePredefinedCacheEx", NotImplementedLinux),
                ("RegConnectRegistryExW", NotImplementedLinux),
                ("RegConnectRegistryExA", NotImplementedLinux),
            ],
        ),
        (
            "RemoteDesktop",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/RemoteDesktop/mod.rs"
            ),
            vec![
                ("WTSWaitSystemEvent", Freeze),
                ("WTSIsChildSessionsEnabled", NotImplementedLinux),
                ("WTSGetChildSessionId", NotImplementedLinux),
            ],
        ),
        (
            "RemoteManagement",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/RemoteManagement/mod.rs"
            ),
            vec![
                ("RmJoinSession", NotImplementedLinux),
                ("RmGetFilterList", NotImplementedLinux),
                ("RmCancelCurrentTask", NotImplementedLinux),
            ],
        ),
        (
            "RestartManager",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/RestartManager/mod.rs"
            ),
            vec![
                ("RmJoinSession", NotImplementedLinux),
                ("RmGetFilterList", NotImplementedLinux),
                ("RmCancelCurrentTask", NotImplementedLinux),
            ],
        ),
        (
            "Restore",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Restore/mod.rs"),
            vec![],
        ),
        (
            "RightsManagement",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Data/RightsManagement/mod.rs"
            ),
            vec![
                ("DRMCheckSecurity", CrashesWindows),
                ("DRMClearAllRights", NotImplementedLinux),
                ("DRMAddRightWithUser", NotImplementedLinux),
                ("DRMAddLicense", NotImplementedLinux),
                ("DRMCloseEnvironmentHandle", NotImplementedLinux),
                ("DRMCloseHandle", NotImplementedLinux),
                ("DRMClosePubHandle", NotImplementedLinux),
                ("DRMCloseQueryHandle", NotImplementedLinux),
                ("DRMCloseSession", NotImplementedLinux),
                ("DRMCreateEnablingBitsDecryptor", NotImplementedLinux),
                ("DRMCreateEnablingBitsEncryptor", NotImplementedLinux),
                ("DRMCreateLicenseStorageSession", NotImplementedLinux),
                ("DRMCreateUser", NotImplementedLinux),
                ("DRMDecode", NotImplementedLinux),
                ("DRMDeconstructCertificateChain", NotImplementedLinux),
                ("DRMDecrypt", NotImplementedLinux),
                ("DRMDeleteLicense", NotImplementedLinux),
                ("DRMDuplicateEnvironmentHandle", NotImplementedLinux),
                ("DRMDuplicateHandle", NotImplementedLinux),
                ("DRMDuplicatePubHandle", NotImplementedLinux),
                ("DRMDuplicateSession", NotImplementedLinux),
                ("DRMEncode", NotImplementedLinux),
                ("DRMEncrypt", NotImplementedLinux),
                ("DRMEnumerateLicense", NotImplementedLinux),
                ("DRMGetApplicationSpecificData", NotImplementedLinux),
                ("DRMGetBoundLicenseAttributeCount", NotImplementedLinux),
                ("DRMGetBoundLicenseObject", NotImplementedLinux),
                ("DRMGetBoundLicenseObjectCount", NotImplementedLinux),
                ("DRMGetCertificateChainCount", NotImplementedLinux),
                ("DRMGetIntervalTime", NotImplementedLinux),
                ("DRMGetIssuanceLicenseTemplate", NotImplementedLinux),
                ("DRMGetMetaData", NotImplementedLinux),
                ("DRMGetNameAndDescription", NotImplementedLinux),
                ("DRMGetOwnerLicense", NotImplementedLinux),
                ("DRMGetRightExtendedInfo", NotImplementedLinux),
                ("DRMGetSecurityProvider", NotImplementedLinux),
                ("DRMGetServiceLocation", NotImplementedLinux),
                ("DRMGetUnboundLicenseAttributeCount", NotImplementedLinux),
                ("DRMGetUnboundLicenseObject", NotImplementedLinux),
                ("DRMGetUnboundLicenseObjectCount", NotImplementedLinux),
                ("DRMGetUserInfo", NotImplementedLinux),
                ("DRMGetUserRights", NotImplementedLinux),
                ("DRMGetUsers", NotImplementedLinux),
                ("DRMIsWindowProtected", NotImplementedLinux),
                ("DRMParseUnboundLicense", NotImplementedLinux),
                ("DRMRegisterProtectedWindow", NotImplementedLinux),
                ("DRMRegisterRevocationList", NotImplementedLinux),
                ("DRMRepair", NotImplementedLinux),
                ("DRMSetApplicationSpecificData", NotImplementedLinux),
                ("DRMSetIntervalTime", NotImplementedLinux),
                ("DRMSetMetaData", NotImplementedLinux),
                ("DRMSetNameAndDescription", NotImplementedLinux),
            ],
        ),
        (
            "Rpc",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Rpc/mod.rs"),
            vec![],
        ),
        (
            "Rras",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/Rras/mod.rs"
            ),
            vec![],
        ),
        (
            "Search",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Search/mod.rs"),
            vec![
                ("SQLCloseEnumServers", CrashesWindows),
                ("ODBCGetTryWaitValue", NotImplementedLinux),
                ("ODBCSetTryWaitValue", NotImplementedLinux),
                ("SQLGetNextEnumeration", NotImplementedLinux),
                ("SQLInitEnumServers", NotImplementedLinux),
                ("dbprtypeA", NotImplementedLinux),
                ("dbprtypeW", NotImplementedLinux),
            ],
        ),
        (
            "Security",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Security/mod.rs"),
            vec![
                ("GetLengthSid", CrashesWindows),
                ("EqualPrefixSid", CrashesWindows),
                ("EqualSid", CrashesWindows),
                ("CopySid", CrashesWindows),
                ("SetSecurityAccessMask", NotImplementedLinux),
                ("QuerySecurityAccessMask", NotImplementedLinux),
                ("GetCachedSigningLevel", NotImplementedLinux),
                ("CheckTokenMembershipEx", NotImplementedLinux),
                ("CheckTokenCapability", NotImplementedLinux),
            ],
        ),
        (
            "SecurityCenter",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/SecurityCenter/mod.rs"
            ),
            vec![("WscGetAntiMalwareUri", InvalidNumberOfArguments)],
        ),
        (
            "Sensors",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/Sensors/mod.rs"
            ),
            vec![],
        ),
        (
            "SerialCommunication",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/SerialCommunication/mod.rs"
            ),
            vec![("ComDBOpen", NotImplementedLinux)],
        ),
        (
            "Services",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Services/mod.rs"
            ),
            vec![("WaitServiceState", NotImplementedLinux)],
        ),
        (
            "SetupAndMigration",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/SetupAndMigration/mod.rs"
            ),
            vec![("OOBEComplete", NotImplementedLinux)],
        ),
        (
            "Shutdown",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Shutdown/mod.rs"
            ),
            vec![],
        ),
        (
            "Sip",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Cryptography/Sip/mod.rs"
            ),
            vec![],
        ),
        (
            "Snmp",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/Snmp/mod.rs"
            ),
            vec![
                ("SnmpEntityToStr", NotImplementedLinux),
                ("SnmpDuplicateVbl", NotImplementedLinux),
                ("SnmpDuplicatePdu", NotImplementedLinux),
                ("SnmpDeleteVb", NotImplementedLinux),
                ("SnmpCountVbl", NotImplementedLinux),
                ("SnmpClose", NotImplementedLinux),
                ("SnmpCleanupEx", NotImplementedLinux),
                ("SnmpCancelMsg", NotImplementedLinux),
                ("SnmpFreeContext", NotImplementedLinux),
                ("SnmpFreeEntity", NotImplementedLinux),
                ("SnmpFreePdu", NotImplementedLinux),
                ("SnmpFreeVbl", NotImplementedLinux),
                ("SnmpGetLastError", NotImplementedLinux),
                ("SnmpGetRetry", NotImplementedLinux),
                ("SnmpGetTimeout", NotImplementedLinux),
                ("SnmpListenEx", NotImplementedLinux),
                ("SnmpMgrOidToStr", NotImplementedLinux),
                ("SnmpMgrOpen", NotImplementedLinux),
                ("SnmpMgrStrToOid", NotImplementedLinux),
                ("SnmpRecvMsg", NotImplementedLinux),
                ("SnmpSendMsg", NotImplementedLinux),
                ("SnmpSetPort", NotImplementedLinux),
                ("SnmpSetRetry", NotImplementedLinux),
                ("SnmpSetTimeout", NotImplementedLinux),
                ("SnmpStrToEntity", NotImplementedLinux),
            ],
        ),
        (
            "SqlLite",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/SqlLite/mod.rs"),
            vec![],
        ),
        (
            "StationsAndDesktops",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/StationsAndDesktops/mod.rs"
            ),
            vec![],
        ),
        (
            "StructuredStorage",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Com/StructuredStorage/mod.rs"
            ),
            vec![("PropStgNameToFmtId", InvalidNumberOfArguments)],
        ),
        (
            "SubsystemForLinux",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/SubsystemForLinux/mod.rs"
            ),
            vec![
                ("WslLaunchInteractive", InvalidNumberOfArguments),
                ("WslLaunch", InvalidNumberOfArguments),
            ],
        ),
        (
            "SystemInformation",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/SystemInformation/mod.rs"
            ),
            vec![
                ("IsWow64GuestMachineSupported", InvalidNumberOfArguments),
                ("GetIntegratedDisplaySize", InvalidNumberOfArguments),
            ],
        ),
        (
            "SystemServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/SystemServices/mod.rs"
            ),
            vec![],
        ),
        (
            "TabletPC",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/TabletPC/mod.rs"),
            vec![],
        ),
        (
            "Tapi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Devices/Tapi/mod.rs"),
            vec![
                ("lineGenerateDigitsW", NotImplementedLinux),
                ("lineGatherDigitsW", NotImplementedLinux),
                ("lineCreateAgentW", NotImplementedLinux),
                ("lineCreateAgentSessionW", NotImplementedLinux),
                ("lineCreateAgentSessionA", NotImplementedLinux),
                ("lineCreateAgentA", NotImplementedLinux),
                ("lineBlindTransferW", NotImplementedLinux),
                ("lineGetAddressIDW", NotImplementedLinux),
                ("lineGetDevConfigW", NotImplementedLinux),
                ("lineGetIconW", NotImplementedLinux),
                ("lineHandoffW", NotImplementedLinux),
                ("lineParkW", NotImplementedLinux),
                ("linePickupW", NotImplementedLinux),
                ("lineProxyMessage", NotImplementedLinux),
                ("lineRedirectW", NotImplementedLinux),
                ("lineSetAgentActivity", NotImplementedLinux),
                ("lineSetAgentMeasurementPeriod", NotImplementedLinux),
                ("lineSetAgentSessionState", NotImplementedLinux),
                ("lineSetAgentState", NotImplementedLinux),
                ("lineSetAgentStateEx", NotImplementedLinux),
                ("lineSetCallTreatment", NotImplementedLinux),
                ("lineSetLineDevStatus", NotImplementedLinux),
                ("lineSetQueueMeasurementPeriod", NotImplementedLinux),
                ("lineSetTollListW", NotImplementedLinux),
                ("lineUnparkW", NotImplementedLinux),
                ("phoneConfigDialogW", NotImplementedLinux),
                ("phoneGetIDW", NotImplementedLinux),
                ("phoneGetIconW", NotImplementedLinux),
                ("tapiRequestDrop", NotImplementedLinux),
                ("tapiRequestMakeCallW", NotImplementedLinux),
                ("tapiRequestMediaCall", NotImplementedLinux),
                ("tapiRequestMediaCallA", NotImplementedLinux),
                ("tapiRequestMediaCallW", NotImplementedLinux),
            ],
        ),
        (
            "TextServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/TextServices/mod.rs"
            ),
            vec![("DoMsCtfMonitor", NotImplementedLinux), ("UninitLocalMsCtfMonitor", NotImplementedLinux)],
        ),
        (
            "Time",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Time/mod.rs"),
            vec![],
        ),
        (
            "ToolHelp",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/Diagnostics/ToolHelp/mod.rs"
            ),
            vec![],
        ),
        (
            "Touch",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/UI/Input/Touch/mod.rs"),
            vec![],
        ),
        (
            "TpmBaseServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/TpmBaseServices/mod.rs"
            ),
            vec![
                ("GetDeviceIDString", CrashesWindows),
                ("Tbsi_Revoke_Attestation", NotImplementedLinux),
                ("Tbsi_Get_TCG_Log_Ex", NotImplementedLinux),
                ("Tbsi_Create_Windows_Key", NotImplementedLinux),
                ("GetDeviceID", NotImplementedLinux),
            ],
        ),
        (
            "UI",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Cryptography/UI/mod.rs"
            ),
            vec![],
        ),
        (
            "UI2",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/Authorization/UI/mod.rs"
            ),
            vec![],
        ),
        (
            "Usb",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Devices/Usb/mod.rs"),
            vec![("WinUsb_GetAdjustedFrameNumber", NotImplementedLinux)],
        ),
        (
            "UserAccessLogging",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/System/UserAccessLogging/mod.rs"
            ),
            vec![],
        ),
        (
            "Vhd",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Storage/Vhd/mod.rs"),
            vec![
                ("GetVirtualDiskPhysicalPath", NotImplementedLinux),
                ("GetAllAttachedVirtualDiskPhysicalPaths", NotImplementedLinux),
                ("EnumerateVirtualDiskMetadata", NotImplementedLinux),
                ("CompleteForkVirtualDisk", NotImplementedLinux),
                ("BreakMirrorVirtualDisk", NotImplementedLinux),
                ("AddVirtualDiskParent", NotImplementedLinux),
            ],
        ),
        (
            "Vss",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Storage/Vss/mod.rs"),
            vec![],
        ),
        (
            "WNet",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WNet/mod.rs"
            ),
            vec![],
        ),
        (
            "WebDav",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WebDav/mod.rs"
            ),
            vec![
                ("DavInvalidateCache", NotImplementedLinux),
                ("DavGetTheLockOwnerOfTheFile", NotImplementedLinux),
                ("DavGetExtendedError", NotImplementedLinux),
                ("DavFlushFile", NotImplementedLinux),
                ("DavDeleteConnection", NotImplementedLinux),
                ("DavCancelConnectionsToServer", NotImplementedLinux),
                ("DavGetUNCFromHTTPPath", CrashesLinux),
                ("DavGetHTTPFromUNCPath", CrashesLinux),
            ],
        ),
        (
            "WebServicesOnDevices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Devices/WebServicesOnDevices/mod.rs"
            ),
            vec![("WSDUriEncode", NotImplementedLinux), ("WSDUriDecode", NotImplementedLinux)],
        ),
        (
            "WebSocket",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/WebSocket/mod.rs"
            ),
            vec![],
        ),
        (
            "WiFi",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WiFi/mod.rs"
            ),
            vec![
                ("WFDCancelOpenSession", NotImplementedLinux),
                ("WFDCloseHandle", NotImplementedLinux),
                ("WFDOpenHandle", NotImplementedLinux),
                ("WFDCloseSession", NotImplementedLinux),
            ],
        ),
        (
            "WinHttp",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/WinHttp/mod.rs"
            ),
            vec![("WinHttpSetProxySettingsPerUser", NotImplementedLinux)],
        ),
        (
            "WinInet",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/WinInet/mod.rs"
            ),
            vec![
                ("ExportCookieFileA", NotImplementedWindows),
                ("ExportCookieFileW", NotImplementedWindows),
                ("FindP3PPolicySymbol", NotImplementedWindows),
                ("GetDiskInfoA", NotImplementedWindows),
                ("ImportCookieFileA", NotImplementedWindows),
                ("ImportCookieFileW", NotImplementedWindows),
                ("InternalInternetGetCookie", NotImplementedWindows),
                ("IsDomainLegalCookieDomainA", NotImplementedWindows),
                ("IsDomainLegalCookieDomainW", NotImplementedWindows),
                ("FwpmDynamicKeywordUnsubscribe0", NotImplementedWindows),
                ("DoConnectoidsExist", NotImplementedWindows),
                ("IsProfilesEnabled", NotImplementedWindows),
                ("InternetUnlockRequestFile", CrashesWindows),
                ("InternetShowSecurityInfoByURLA", CrashesWindows),
                ("InternetShowSecurityInfoByURLW", CrashesWindows),
                ("InternetShowSecurityInfoByURL", CrashesWindows),
                ("AppCacheDeleteGroup", NotImplementedLinux),
                ("AppCacheDeleteIEGroup", NotImplementedLinux),
                ("AppCacheFreeIESpace", NotImplementedLinux),
                ("AppCacheFreeSpace", NotImplementedLinux),
                ("CreateUrlCacheEntryExW", NotImplementedLinux),
                ("GetUrlCacheHeaderData", NotImplementedLinux),
                ("HttpGetServerCredentials", NotImplementedLinux),
                ("HttpIsHostHstsEnabled", NotImplementedLinux),
                ("InternetAlgIdToStringA", NotImplementedLinux),
                ("InternetAlgIdToStringW", NotImplementedLinux),
                ("InternetConvertUrlFromWireToWideChar", NotImplementedLinux),
                ("InternetFortezzaCommand", NotImplementedLinux),
                ("InternetSecurityProtocolToStringA", NotImplementedLinux),
                ("InternetSecurityProtocolToStringW", NotImplementedLinux),
                ("InternetSetDialState", NotImplementedLinux),
                ("InternetSetDialStateA", NotImplementedLinux),
                ("InternetSetDialStateW", NotImplementedLinux),
                ("SetUrlCacheHeaderData", NotImplementedLinux),
                ("UpdateUrlCacheContentPath", NotImplementedLinux),
                ("UrlCacheContainerSetEntryMaximumAge", NotImplementedLinux),
                ("UrlCacheCreateContainer", NotImplementedLinux),
                ("UrlCacheFreeGlobalSpace", NotImplementedLinux),
                ("UrlCacheGetGlobalCacheSize", NotImplementedLinux),
                ("UrlCacheReloadSettings", NotImplementedLinux),
                ("UrlCacheServer", NotImplementedLinux),
            ],
        ),
        (
            "WinML",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/AI/MachineLearning/WinML/mod.rs"
            ),
            vec![],
        ),
        (
            "WinSock",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/WinSock/mod.rs"
            ),
            vec![],
        ),
        (
            "WinTrust",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Security/WinTrust/mod.rs"
            ),
            vec![
                ("OpenPersonalTrustDBDialog", ShowsDialogWindows),
                ("WintrustSetDefaultIncludePEPageHashes", NotImplementedLinux),
            ],
        ),
        (
            "WindowsFilteringPlatform",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsFilteringPlatform/mod.rs"
            ),
            vec![
                ("FwpmDynamicKeywordUnsubscribe0", NotImplementedWindows),
                ("FwpmFilterDeleteById0", NotImplementedLinux),
                ("FwpmConnectionUnsubscribe0", NotImplementedLinux),
                ("FwpmConnectionDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmCalloutUnsubscribeChanges0", NotImplementedLinux),
                ("FwpmCalloutDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmCalloutDeleteById0", NotImplementedLinux),
                ("FwpmFilterUnsubscribeChanges0", NotImplementedLinux),
                ("FwpmLayerDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmNetEventDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmNetEventUnsubscribe0", NotImplementedLinux),
                ("FwpmProviderContextDeleteById0", NotImplementedLinux),
                ("FwpmProviderContextDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmProviderContextUnsubscribeChanges0", NotImplementedLinux),
                ("FwpmProviderDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmProviderUnsubscribeChanges0", NotImplementedLinux),
                ("FwpmSessionDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmSubLayerDestroyEnumHandle0", NotImplementedLinux),
                ("FwpmSubLayerUnsubscribeChanges0", NotImplementedLinux),
                ("FwpmSystemPortsUnsubscribe0", NotImplementedLinux),
                ("FwpmTransactionAbort0", NotImplementedLinux),
                ("FwpmTransactionBegin0", NotImplementedLinux),
                ("FwpmTransactionCommit0", NotImplementedLinux),
                ("FwpmvSwitchEventUnsubscribe0", NotImplementedLinux),
                ("IPsecDospStateDestroyEnumHandle0", NotImplementedLinux),
                ("IPsecKeyManagerUnregisterAndDelete0", NotImplementedLinux),
                ("IPsecSaContextDeleteById0", NotImplementedLinux),
                ("IPsecSaContextDestroyEnumHandle0", NotImplementedLinux),
                ("IPsecSaContextExpire0", NotImplementedLinux),
                ("IPsecSaContextUnsubscribe0", NotImplementedLinux),
                ("IPsecSaDestroyEnumHandle0", NotImplementedLinux),
                ("IkeextSaDeleteById0", NotImplementedLinux),
                ("IkeextSaDestroyEnumHandle0", NotImplementedLinux),
            ],
        ),
        (
            "WindowsFirewall",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsFirewall/mod.rs"
            ),
            vec![],
        ),
        (
            "WindowsMediaFormat",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/WindowsMediaFormat/mod.rs"
            ),
            vec![],
        ),
        (
            "WindowsNetworkVirtualization",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsNetworkVirtualization/mod.rs"
            ),
            vec![],
        ),
        (
            "WindowsWebServices",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Networking/WindowsWebServices/mod.rs"
            ),
            vec![("WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable", InvalidNumberOfArguments)],
        ),
        (
            "Wmi",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/System/Wmi/mod.rs"),
            vec![],
        ),
        (
            "XAudio2",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Media/Audio/XAudio2/mod.rs"
            ),
            vec![],
        ),
        (
            "XboxController",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/UI/Input/XboxController/mod.rs"
            ),
            vec![],
        ),
        (
            "XmlLite",
            format!(
                "{}{}",
                get_windows_rs_folder(),
                "crates/libs/sys/src/Windows/Win32/Data/Xml/XmlLite/mod.rs"
            ),
            vec![],
        ),
        (
            "Xps",
            format!("{}{}", get_windows_rs_folder(), "crates/libs/sys/src/Windows/Win32/Storage/Xps/mod.rs"),
            vec![],
        ),
    ]
}

pub struct FileData {
    pub(crate) functions: BTreeMap<String, Vec<String>>,
}
impl FileData {
    pub fn new() -> Self {
        FileData {
            functions: Default::default(),
        }
    }
}
