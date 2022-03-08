use self::TypeOfProblem::*;
use std::collections::BTreeMap;

pub const WINDOWS_RS_FOLDER: &str = "/home/rafal/test/windows-rs/";
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
    "SubsystemForLinux",                   //api-ms-win-wsl-api-l1-1-0.dll
    "SystemInformation",                   // api-ms-win-core-sysinfo-l1-2-4.dll, api-ms-win-core-sysinfo-l1-2-3.dll
    "UserAccessLogging",                   // ualapi.dll
    "WNet",                                //ntlanman.dll
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
                                           // "Printer",
                                           // "Dwm",
                                           // "Gaming",
                                           // "Dxgi",
                                           // "BiometricFramework",
                                           // "Com",
                                           // "CloudFilters",
                                           // "Debug",
                                           // "DeploymentServices",
                                           // "FileSystem",
                                           // "Identity",
                                           // "MediaFoundation",
                                           // "Ole",
                                           // "P2P",
                                           // "PrintTicket",
                                           // "Printing",
                                           // "Printing2",
                                           // "Shell",
                                           // "Threading",
                                           // "Urlmon",
                                           // "WinRT",
                                           // "WindowsConnectionManager",
                                           // "WindowsProgramming",
                                           // "Accessibility",
                                           // "ActiveDirectory",
                                           // "AddressBook",
                                           // "AllJoyn",
                                           // "Antimalware",
                                           // "AppLocker",
                                           // "ApplicationInstallationAndServicing",
                                           // "ApplicationVerifier",
                                           // "Appx",
                                           // "Audio",
                                           // "Authorization",
                                           // "Bluetooth",
                                           // "Cabinets",
                                           // "CallObj",
                                           // "Catalog",
                                           // "Ceip",
                                           // "Clustering",
                                           // "ColorSystem",
                                           // "Communication",
                                           // "ComponentServices",
                                           // "CompositionSwapchain",
                                           // "Compression",
                                           // "Console",
                                           // "Controls",
                                           // "CorrelationVector",
                                           // "Credentials",
                                           // "Cryptography",
                                           // "DXCore",
                                           // "DataExchange",
                                           // "DeveloperLicensing",
                                           // "DeviceAccess",
                                           // "DeviceAndDriverInstallation",
                                           // "DeviceQuery",
                                           // "Dhcp",
                                           // "Dialogs",
                                           // "Direct2D",
                                           // "Direct3D10",
                                           // "Direct3D11",
                                           // "Direct3D112",
                                           // "Direct3D11on12",
                                           // "Direct3D12",
                                           // "Direct3D9",
                                           // "Direct3D9on12",
                                           // "DirectComposition",
                                           // "DirectDraw",
                                           // "DirectML",
                                           // "DirectShow",
                                           // "DirectSound",
                                           // "DirectWrite",
                                           // "DirectoryServices",
                                           // "Display",
                                           // "DistributedFileSystem",
                                           // "DistributedTransactionCoordinator",
                                           // "Dns",
                                           // "DxMediaObjects",
                                           // "Dxc",
                                           // "EnterpriseData",
                                           // "Environment",
                                           // "ErrorReporting",
                                           // "Etw",
                                           // "EventCollector",
                                           // "EventLog",
                                           // "EventNotificationService",
                                           // "ExtensibleAuthenticationProtocol",
                                           // "Fax",
                                           // "Foundation",
                                           // "Fxc",
                                           // "Gdi",
                                           // "Globalization",
                                           // "GroupPolicy",
                                           // "HardwareCounterProfiling",
                                           // "HiDpi",
                                           // "HostComputeNetwork",
                                           // "HostComputeSystem",
                                           // "HttpServer",
                                           // "HumanInterfaceDevice",
                                           // "Hypervisor",
                                           // "IO",
                                           // "Iis",
                                           // "Imaging",
                                           // "Imapi",
                                           // "Ime",
                                           // "IndexServer",
                                           // "Input",
                                           // "InstallableFileSystems",
                                           // "InteractionContext",
                                           // "IpHelper",
                                           // "IscsiDisc",
                                           // "Isolation",
                                           // "Jet",
                                           // "JobObjects",
                                           // "Js",
                                           // "Kernel",
                                           // "KernelStreaming",
                                           // "KeyboardAndMouse",
                                           // "Ldap",
                                           // "LibraryLoader",
                                           // "LicenseProtection",
                                           // "Magnification",
                                           // "Mailslots",
                                           // "Mapi",
                                           // "Marshal",
                                           // "Media",
                                           // "Memory",
                                           // "MobileDeviceManagementRegistration",
                                           // "MsHtml",
                                           // "Multicast",
                                           // "Multimedia",
                                           // "NetBios",
                                           // "NetManagement",
                                           // "NetShell",
                                           // "NetworkDiagnosticsFramework",
                                           // "NonVolatile",
                                           // "OfflineFiles",
                                           // "OpenGL",
                                           // "OperationRecorder",
                                           // "PasswordManagement",
                                           // "Pdf",
                                           // "Performance",
                                           // "Pipes",
                                           // "Pnp",
                                           // "Pointer",
                                           // "PortableDevices",
                                           // "Power",
                                           // "ProcessSnapshotting",
                                           // "ProcessStatus",
                                           // "ProjectedFileSystem",
                                           // "PropertiesSystem",
                                           // "QoS",
                                           // "Recovery",
                                           // "Registry",
                                           // "RemoteDesktop",
                                           // "RemoteManagement",
                                           // "RestartManager",
                                           // "Restore",
                                           // "RightsManagement",
                                           // "Rpc",
                                           // "Rras",
                                           // "Search",
                                           // "Security",
                                           // "SecurityCenter",
                                           // "Sensors",
                                           // "SerialCommunication",
                                           // "Services",
                                           // "SetupAndMigration",
                                           // "Shutdown",
                                           // "Sip",
                                           // "Snmp",
                                           // "SqlLite",
                                           // "StationsAndDesktops",
                                           // "StructuredStorage",
                                           // "SubsystemForLinux",
                                           // "SystemInformation",
                                           // "SystemServices",
                                           // "TabletPC",
                                           // "Tapi",
                                           // "TextServices",
                                           // "Time",
                                           // "ToolHelp",
                                           // "Touch",
                                           // "TpmBaseServices",
                                           // "UI",
                                           // "UI2",
                                           // "Usb",
                                           // "UserAccessLogging",
                                           // "Vhd",
                                           // "Vss",
                                           // "WNet",
                                           // "WebDav",
                                           // "WebServicesOnDevices",
                                           // "WebSocket",
                                           // "WiFi",
                                           // "WinHttp",
                                           // "WinML",
                                           // "WinSock",
                                           // "WinTrust",
                                           // "WindowsFilteringPlatform",
                                           // "WindowsFirewall",
                                           // "WindowsMediaFormat",
                                           // "WindowsNetworkVirtualization",
                                           // "WindowsWebServices",
                                           // "Wmi",
                                           // "XAudio2",
                                           // "XboxController",
                                           // "XmlLite",
                                           // "Xps",
];

// List of classes which are handled manually(because have already set exception list)
pub const MANUAL_CLASSES: &[&str] = &[
    "Printer",
    "Dwm",
    "Gaming",
    "Dxgi",
    "Com",
    "BiometricFramework",
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
    "WindowsAndMessaging",
    "WindowsConnectionManager",
    "WindowsProgramming",
];

pub enum TypeOfProblem {
    NotImplementedLinux,
    NotImplementedWindows,
    InvalidNumberOfArguments,
    ShowsDialogWindows,
    CrashesWindows,
    //CrashesLinux,
    CrashAutomatic, // Crashes are normal for this code
    Freeze,
    Other,
}

pub fn load_settings() -> Vec<(&'static str, String, Vec<(&'static str, TypeOfProblem)>)> {
    vec![
        (
            "Printer",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"),
            vec![
                ("AddPrintDeviceObject", InvalidNumberOfArguments),
                ("CloseSpoolFileHandle", NotImplementedLinux),
                ("CommitSpoolData", NotImplementedLinux),
                ("DeletePrinterIC", NotImplementedLinux),
                ("GdiGetPageCount", NotImplementedLinux),
                ("CloseSpoolFileHandle", NotImplementedLinux),
                ("DeleteJobNamedProperty", NotImplementedLinux),
                ("DeletePrinterDataA", NotImplementedLinux),
                ("DeletePrinterDataW", NotImplementedLinux),
                ("DeletePrinterDriverPackageA", NotImplementedLinux),
                ("DeletePrinterDriverPackageW", NotImplementedLinux),
                ("DeletePrinterKeyA", NotImplementedLinux),
                ("DeletePrinterKeyW", NotImplementedLinux),
                ("GdiDeleteSpoolFileHandle", NotImplementedLinux),
                ("GdiEndDocEMF", NotImplementedLinux),
                ("GdiEndPageEMF", NotImplementedLinux),
                ("GdiGetDC", NotImplementedLinux),
                ("GdiStartPageEMF", NotImplementedLinux),
                ("InstallPrinterDriverFromPackageA", NotImplementedLinux),
                ("InstallPrinterDriverFromPackageW", NotImplementedLinux),
                ("PrinterMessageBoxA", NotImplementedLinux),
                ("PrinterMessageBoxW", NotImplementedLinux),
                ("ProvidorFindClosePrinterChangeNotification", NotImplementedLinux),
                ("RemovePrintDeviceObject", NotImplementedLinux),
                ("RouterAllocBidiMem", NotImplementedLinux),
                ("RouterAllocBidiResponseContainer", NotImplementedLinux),
                ("RouterAllocPrinterNotifyInfo", NotImplementedLinux),
                ("SpoolerFindClosePrinterChangeNotification", NotImplementedLinux),
                ("UnRegisterForPrintAsyncNotifications", NotImplementedLinux),
                ("UpdatePrintDeviceObject", NotImplementedLinux),
                ("WaitForPrinterChange", NotImplementedLinux),
                ("GdiGetPageHandle", NotImplementedLinux),
                ("GdiGetSpoolFileHandle", NotImplementedLinux),
                ("GdiResetDCEMF", NotImplementedLinux),
                ("GdiGetPageHandle", NotImplementedLinux),
                ("GdiGetPageHandle", NotImplementedLinux),
                ("GetPrinterDriverPackagePathA", NotImplementedLinux),
                ("GetSpoolFileHandle", NotImplementedLinux),
                ("GetPrintOutputInfo", InvalidNumberOfArguments),
                ("GetPrinterDriver2A", InvalidNumberOfArguments),
                ("GetPrinterDriver2W", InvalidNumberOfArguments),
                ("GetPrinterDriverPackagePathW", InvalidNumberOfArguments),
                ("SplIsSessionZero", InvalidNumberOfArguments),
                ("CorePrinterDriverInstalledW", InvalidNumberOfArguments),
                ("CorePrinterDriverInstalledA", InvalidNumberOfArguments),
                ("ConnectToPrinterDlg", ShowsDialogWindows),
            ],
        ),
        (
            "Dwm",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Dwm/mod.rs"),
            vec![
                ("DwmRegisterThumbnail", InvalidNumberOfArguments),
                ("DwmSetDxFrameDuration", NotImplementedLinux),
                ("DwmIsCompositionEnabled", InvalidNumberOfArguments),
                ("DwmGetGraphicsStreamClient", InvalidNumberOfArguments),
                ("DwmQueryThumbnailSourceSize", InvalidNumberOfArguments),
            ],
        ),
        (
            "Gaming",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Gaming/mod.rs"),
            vec![("GetExpandedResourceExclusiveCpuCount", InvalidNumberOfArguments)],
        ),
        (
            "Dxgi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Dxgi/mod.rs"),
            vec![("DXGIDeclareAdapterRemovalSupport", NotImplementedLinux)],
        ),
        (
            "BiometricFramework",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/BiometricFramework/mod.rs"
            ),
            vec![("WinBioEnrollCapture", CrashesWindows), ("WinBioLocateSensor", CrashesWindows)],
        ),
        (
            "Com",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/mod.rs"),
            vec![
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
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/CloudFilters/mod.rs"),
            vec![("CfGetTransferKey", CrashesWindows)],
        ),
        (
            "Debug",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Debug/mod.rs"
            ),
            vec![
                ("FatalExit", CrashAutomatic),
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/DeploymentServices/mod.rs"
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
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/FileSystem/mod.rs"),
            vec![("WofGetDriverVersion", CrashesWindows), ("WofWimAddEntry", CrashesWindows)],
        ),
        (
            "Identity",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Authentication/Identity/mod.rs"
            ),
            vec![("SLGetWindowsInformationDWORD", CrashesWindows)],
        ),
        (
            "MediaFoundation",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/MediaFoundation/mod.rs"
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
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Ole/mod.rs"),
            vec![
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/P2P/mod.rs"
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Printing/PrintTicket/mod.rs"
            ),
            vec![("PTQuerySchemaVersionSupport", InvalidNumberOfArguments)],
        ),
        (
            "Printing",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"),
            vec![
                ("CorePrinterDriverInstalledA", InvalidNumberOfArguments),
                ("CorePrinterDriverInstalledW", InvalidNumberOfArguments),
                ("RouterAllocBidiMem", CrashesWindows),
                ("SHRegSetPathA", CrashesWindows),
                ("ConnectToPrinterDlg", ShowsDialogWindows),
            ],
        ),
        (
            "Printing2",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Xps/Printing/mod.rs"),
            vec![],
        ),
        (
            "Shell",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Shell/mod.rs"),
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
            ],
        ),
        (
            "Threading",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Threading/mod.rs"),
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
            ],
        ),
        (
            "Urlmon",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/Urlmon/mod.rs"),
            vec![
                ("IEInstallScope", InvalidNumberOfArguments),
                ("FindMediaType", InvalidNumberOfArguments),
                ("GetClassURL", InvalidNumberOfArguments),
                ("CoInternetCompareUrl", CrashesWindows),
            ],
        ),
        (
            "WinRT",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/WinRT/mod.rs"),
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
            ],
        ),
        (
            "WindowsAndMessaging",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/WindowsAndMessaging/mod.rs"
            ),
            vec![("MrmGetPriFileContentChecksum", InvalidNumberOfArguments)],
        ),
        (
            "WindowsConnectionManager",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsConnectionManager/mod.rs"
            ),
            vec![("OnDemandGetRoutingHint", InvalidNumberOfArguments)],
        ),
        (
            "WindowsProgramming",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/WindowsProgramming/mod.rs"
            ),
            vec![
                ("QueryAuxiliaryCounterFrequency", InvalidNumberOfArguments),
                ("WldpIsDynamicCodePolicyEnabled", InvalidNumberOfArguments),
            ],
        ),
        // ////////////////////////
        // ////////////////////////
        // ////////////////////////
        // ////////////////////////
        // AUTOMATICALLY GENERATED
        // ////////////////////////
        // ////////////////////////
        // ////////////////////////
        // ////////////////////////
        (
            "Accessibility",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Accessibility/mod.rs"),
            vec![],
        ),
        (
            "ActiveDirectory",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/ActiveDirectory/mod.rs"
            ),
            vec![],
        ),
        (
            "AddressBook",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/AddressBook/mod.rs"),
            vec![],
        ),
        (
            "AllJoyn",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/AllJoyn/mod.rs"),
            vec![],
        ),
        (
            "Antimalware",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Antimalware/mod.rs"),
            vec![],
        ),
        (
            "AppLocker",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/AppLocker/mod.rs"),
            vec![],
        ),
        (
            "ApplicationInstallationAndServicing",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/ApplicationInstallationAndServicing/mod.rs"
            ),
            vec![],
        ),
        (
            "ApplicationVerifier",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/ApplicationVerifier/mod.rs"
            ),
            vec![],
        ),
        (
            "Appx",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Packaging/Appx/mod.rs"
            ),
            vec![
                ("GetResolvedPackageFullNameForPackageDependency", InvalidNumberOfArguments),
                ("CheckIsMSIXPackage", InvalidNumberOfArguments),
                ("DeactivatePackageVirtualizationContext", NotImplementedWindows),
                ("DeletePackageDependency", NotImplementedWindows),
                ("GetCurrentPackageVirtualizationContext", NotImplementedWindows),
            ],
        ),
        (
            "Audio",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/Audio/mod.rs"),
            vec![],
        ),
        (
            "Authorization",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Authorization/mod.rs"
            ),
            vec![],
        ),
        (
            "Bluetooth",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Bluetooth/mod.rs"),
            vec![
                ("BluetoothFindRadioClose", CrashesWindows),
                ("BluetoothFindDeviceClose", CrashesWindows),
                ("BluetoothFindNextRadio", CrashesWindows),
                ("BluetoothUnregisterAuthentication", CrashesWindows),
            ],
        ),
        (
            "Cabinets",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Cabinets/mod.rs"),
            vec![],
        ),
        (
            "CallObj",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/CallObj/mod.rs"),
            vec![],
        ),
        (
            "Catalog",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Cryptography/Catalog/mod.rs"
            ),
            vec![
                ("CryptCATAdminReleaseContext", CrashesWindows),
                ("CryptCATAdminReleaseCatalogContext", CrashesWindows),
                ("CryptCATAdminAddCatalog", CrashesWindows),
                ("CryptCATAdminCalcHashFromFileHandle2", CrashesWindows),
            ],
        ),
        (
            "Ceip",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Ceip/mod.rs"
            ),
            vec![],
        ),
        (
            "Certificates",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Cryptography/Certificates/mod.rs"
            ),
            vec![],
        ),
        (
            "Clustering",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/Clustering/mod.rs"
            ),
            vec![],
        ),
        (
            "ColorSystem",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/ColorSystem/mod.rs"),
            vec![],
        ),
        (
            "Communication",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Communication/mod.rs"
            ),
            vec![],
        ),
        (
            "ComponentServices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/ComponentServices/mod.rs"
            ),
            vec![],
        ),
        (
            "CompositionSwapchain",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/CompositionSwapchain/mod.rs"
            ),
            vec![],
        ),
        (
            "Compression",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Compression/mod.rs"),
            vec![("CloseDecompressor", CrashesWindows), ("ResetDecompressor", CrashesWindows)],
        ),
        (
            "Console",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Console/mod.rs"),
            vec![("FreeConsole", CrashesWindows)],
        ),
        (
            "Controls",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Controls/mod.rs"),
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
            ],
        ),
        (
            "CorrelationVector",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/CorrelationVector/mod.rs"
            ),
            vec![],
        ),
        (
            "Credentials",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Credentials/mod.rs"),
            vec![("CredUIParseUserNameA", CrashesWindows), ("CredGetSessionTypes", CrashesWindows)],
        ),
        (
            "Cryptography",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Cryptography/mod.rs"
            ),
            vec![],
        ),
        (
            "DXCore",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DXCore/mod.rs"),
            vec![],
        ),
        (
            "DataExchange",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/DataExchange/mod.rs"),
            vec![
                ("GetAtomNameA", CrashesWindows),
                ("GlobalGetAtomNameW", CrashesWindows),
                ("GlobalGetAtomNameA", CrashesWindows),
                ("GetAtomNameW", CrashesWindows),
            ],
        ),
        (
            "DeveloperLicensing",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/DeveloperLicensing/mod.rs"
            ),
            vec![],
        ),
        (
            "DeviceAccess",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/DeviceAccess/mod.rs"),
            vec![],
        ),
        (
            "DeviceAndDriverInstallation",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/DeviceAndDriverInstallation/mod.rs"
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
            ],
        ),
        (
            "DeviceQuery",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/DeviceQuery/mod.rs"),
            vec![],
        ),
        (
            "Dhcp",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/Dhcp/mod.rs"
            ),
            vec![],
        ),
        (
            "DiagnosticDataQuery",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/DiagnosticDataQuery/mod.rs"
            ),
            vec![
                ("DdqGetDiagnosticRecordPayload", InvalidNumberOfArguments),
                ("DdqGetDiagnosticReportStoreReportCount", InvalidNumberOfArguments),
            ],
        ),
        (
            "Diagnostics",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Xaml/Diagnostics/mod.rs"),
            vec![],
        ),
        (
            "Dialogs",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Controls/Dialogs/mod.rs"),
            vec![],
        ),
        (
            "Direct2D",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct2D/mod.rs"),
            vec![],
        ),
        (
            "Direct3D10",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D10/mod.rs"),
            vec![],
        ),
        (
            "Direct3D11",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/WinRT/Direct3D11/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D112",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D11/mod.rs"),
            vec![],
        ),
        (
            "Direct3D11on12",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D11on12/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D12",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D12/mod.rs"),
            vec![],
        ),
        (
            "Direct3D9",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D9/mod.rs"),
            vec![],
        ),
        (
            "Direct3D9on12",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D9on12/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectComposition",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectComposition/mod.rs"
            ),
            vec![("DCompositionBoostCompositorClock", NotImplementedWindows)],
        ),
        (
            "DirectDraw",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectDraw/mod.rs"),
            vec![],
        ),
        (
            "DirectML",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/AI/MachineLearning/DirectML/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectShow",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/DirectShow/mod.rs"),
            vec![],
        ),
        (
            "DirectSound",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/Audio/DirectSound/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectWrite",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectWrite/mod.rs"),
            vec![],
        ),
        (
            "DirectoryServices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/DirectoryServices/mod.rs"
            ),
            vec![],
        ),
        (
            "Display",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Display/mod.rs"),
            vec![("EngUnicodeToMultiByteN", CrashesWindows), ("EngMultiByteToUnicodeN", CrashesWindows)],
        ),
        (
            "DistributedFileSystem",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/DistributedFileSystem/mod.rs"
            ),
            vec![],
        ),
        (
            "DistributedTransactionCoordinator",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/DistributedTransactionCoordinator/mod.rs"
            ),
            vec![],
        ),
        (
            "Dns",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/Dns/mod.rs"
            ),
            vec![],
        ),
        (
            "DxMediaObjects",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/DxMediaObjects/mod.rs"),
            vec![],
        ),
        (
            "Dxc",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D/Dxc/mod.rs"
            ),
            vec![],
        ),
        (
            "EnterpriseData",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/EnterpriseData/mod.rs"
            ),
            vec![],
        ),
        (
            "Environment",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Environment/mod.rs"),
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
            ],
        ),
        (
            "ErrorReporting",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/ErrorReporting/mod.rs"
            ),
            vec![("WerRegisterAppLocalDump", CrashesWindows)],
        ),
        (
            "Etw",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Etw/mod.rs"
            ),
            vec![],
        ),
        (
            "EventCollector",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/EventCollector/mod.rs"
            ),
            vec![],
        ),
        (
            "EventLog",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/EventLog/mod.rs"),
            vec![],
        ),
        (
            "EventNotificationService",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/EventNotificationService/mod.rs"
            ),
            vec![],
        ),
        (
            "ExtensibleAuthenticationProtocol",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/ExtensibleAuthenticationProtocol/mod.rs"
            ),
            vec![],
        ),
        (
            "Fax",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Fax/mod.rs"),
            vec![],
        ),
        (
            "FileHistory",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/FileHistory/mod.rs"),
            vec![("FhServiceOpenPipe", InvalidNumberOfArguments)],
        ),
        (
            "Foundation",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Foundation/mod.rs"),
            vec![],
        ),
        (
            "Fxc",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D/Fxc/mod.rs"
            ),
            vec![],
        ),
        (
            "Gdi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Gdi/mod.rs"),
            vec![
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
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Globalization/mod.rs"),
            vec![("GetDistanceOfClosestLanguageInList", InvalidNumberOfArguments)],
        ),
        (
            "GroupPolicy",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/GroupPolicy/mod.rs"),
            vec![],
        ),
        (
            "HardwareCounterProfiling",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Performance/HardwareCounterProfiling/mod.rs"
            ),
            vec![],
        ),
        (
            "HiDpi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/HiDpi/mod.rs"),
            vec![],
        ),
        (
            "HostComputeNetwork",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/HostComputeNetwork/mod.rs"
            ),
            vec![],
        ),
        (
            "HostComputeSystem",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/HostComputeSystem/mod.rs"
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/HttpServer/mod.rs"
            ),
            vec![],
        ),
        (
            "HumanInterfaceDevice",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/HumanInterfaceDevice/mod.rs"
            ),
            vec![("HidD_FreePreparsedData", CrashesWindows)],
        ),
        (
            "Hypervisor",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Hypervisor/mod.rs"),
            vec![],
        ),
        (
            "IO",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/IO/mod.rs"),
            vec![],
        ),
        (
            "Iis",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Iis/mod.rs"),
            vec![],
        ),
        (
            "Imaging",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Imaging/mod.rs"),
            vec![("WICMapShortNameToGuid", InvalidNumberOfArguments)],
        ),
        (
            "Imapi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Imapi/mod.rs"),
            vec![],
        ),
        (
            "Ime",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/Ime/mod.rs"),
            vec![("ImmInstallIMEA", CrashesWindows)],
        ),
        (
            "IndexServer",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/IndexServer/mod.rs"),
            vec![],
        ),
        (
            "Input",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/mod.rs"),
            vec![],
        ),
        (
            "InstallableFileSystems",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/InstallableFileSystems/mod.rs"
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/InteractionContext/mod.rs"
            ),
            vec![("CreateInteractionContext", InvalidNumberOfArguments)],
        ),
        (
            "IpHelper",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/IpHelper/mod.rs"
            ),
            vec![("GetIpErrorString", CrashesWindows)],
        ),
        (
            "IscsiDisc",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/IscsiDisc/mod.rs"),
            vec![],
        ),
        (
            "Isolation",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Isolation/mod.rs"),
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
            ],
        ),
        (
            "Jet",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Jet/mod.rs"),
            vec![],
        ),
        (
            "JobObjects",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/JobObjects/mod.rs"),
            vec![],
        ),
        (
            "Js",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Js/mod.rs"),
            vec![],
        ),
        (
            "Kernel",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Kernel/mod.rs"),
            vec![],
        ),
        (
            "KernelStreaming",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/KernelStreaming/mod.rs"
            ),
            vec![],
        ),
        (
            "KeyboardAndMouse",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/KeyboardAndMouse/mod.rs"
            ),
            vec![("GetKeyNameTextA", CrashesWindows), ("GetKeyboardLayoutNameA", CrashesWindows)],
        ),
        (
            "Ldap",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/Ldap/mod.rs"),
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
            ],
        ),
        (
            "LibraryLoader",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/LibraryLoader/mod.rs"),
            vec![
                ("FreeLibraryAndExitThread", CrashAutomatic),
                ("GetDllDirectoryA", CrashesWindows),
                ("GetDllDirectoryW", CrashesWindows),
                ("GetModuleFileNameA", CrashesWindows),
                ("AddDllDirectory", CrashesWindows),
                ("GetModuleFileNameW", CrashesWindows),
            ],
        ),
        (
            "LicenseProtection",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/LicenseProtection/mod.rs"
            ),
            vec![],
        ),
        (
            "Magnification",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Magnification/mod.rs"),
            vec![],
        ),
        (
            "Mailslots",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Mailslots/mod.rs"),
            vec![],
        ),
        (
            "Mapi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Mapi/mod.rs"),
            vec![],
        ),
        (
            "Marshal",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/Marshal/mod.rs"),
            vec![],
        ),
        (
            "Media",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/mod.rs"),
            vec![],
        ),
        (
            "Memory",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Memory/mod.rs"),
            vec![],
        ),
        (
            "MobileDeviceManagementRegistration",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Management/MobileDeviceManagementRegistration/mod.rs"
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
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Web/MsHtml/mod.rs"),
            vec![
                ("RatingObtainQueryW", InvalidNumberOfArguments),
                ("RatingObtainQuery", InvalidNumberOfArguments),
            ],
        ),
        (
            "Multicast",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/Multicast/mod.rs"
            ),
            vec![],
        ),
        (
            "Multimedia",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/Multimedia/mod.rs"),
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
            ],
        ),
        (
            "NetBios",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetBios/mod.rs"
            ),
            vec![],
        ),
        (
            "NetManagement",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetManagement/mod.rs"
            ),
            vec![("RouterAssert", CrashesWindows), ("NetScheduleJobAdd", CrashesWindows)],
        ),
        (
            "NetShell",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetShell/mod.rs"
            ),
            vec![],
        ),
        (
            "NetworkDiagnosticsFramework",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/NetworkDiagnosticsFramework/mod.rs"
            ),
            vec![],
        ),
        (
            "NonVolatile",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Memory/NonVolatile/mod.rs"
            ),
            vec![],
        ),
        (
            "OfflineFiles",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/OfflineFiles/mod.rs"),
            vec![],
        ),
        (
            "OpenGL",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/OpenGL/mod.rs"),
            vec![],
        ),
        (
            "OperationRecorder",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/OperationRecorder/mod.rs"
            ),
            vec![],
        ),
        (
            "PasswordManagement",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/PasswordManagement/mod.rs"
            ),
            vec![],
        ),
        (
            "Pdf",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/WinRT/Pdf/mod.rs"),
            vec![],
        ),
        (
            "Performance",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Performance/mod.rs"),
            vec![("PerfEnumerateCounterSet", CrashesWindows)],
        ),
        (
            "Pipes",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Pipes/mod.rs"),
            vec![("WaitNamedPipeW", CrashesWindows)],
        ),
        (
            "Pnp",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Enumeration/Pnp/mod.rs"
            ),
            vec![],
        ),
        (
            "Pointer",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/Pointer/mod.rs"),
            vec![],
        ),
        (
            "PortableDevices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/PortableDevices/mod.rs"
            ),
            vec![],
        ),
        (
            "Power",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Power/mod.rs"),
            vec![],
        ),
        (
            "ProcessSnapshotting",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Diagnostics/ProcessSnapshotting/mod.rs"
            ),
            vec![],
        ),
        (
            "ProcessStatus",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/ProcessStatus/mod.rs"),
            vec![("K32EnumProcesses", CrashesWindows)],
        ),
        (
            "ProjectedFileSystem",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/ProjectedFileSystem/mod.rs"
            ),
            vec![],
        ),
        (
            "PropertiesSystem",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Shell/PropertiesSystem/mod.rs"
            ),
            vec![
                ("PSLookupPropertyHandlerCLSID", InvalidNumberOfArguments),
                ("InitPropVariantFromResource", InvalidNumberOfArguments),
                ("InitPropVariantFromStringAsVector", InvalidNumberOfArguments),
                ("InitVariantFromResource", InvalidNumberOfArguments),
                ("ClearPropVariantArray", CrashesWindows),
                ("ClearVariantArray", CrashesWindows),
            ],
        ),
        (
            "QoS",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/QoS/mod.rs"
            ),
            vec![],
        ),
        (
            "Recovery",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Recovery/mod.rs"),
            vec![("ApplicationRecoveryInProgress", InvalidNumberOfArguments)],
        ),
        (
            "Registry",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Registry/mod.rs"),
            vec![],
        ),
        (
            "RemoteDesktop",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/RemoteDesktop/mod.rs"),
            vec![("WTSWaitSystemEvent", Freeze)],
        ),
        (
            "RemoteManagement",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/RemoteManagement/mod.rs"
            ),
            vec![],
        ),
        (
            "RestartManager",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/RestartManager/mod.rs"
            ),
            vec![],
        ),
        (
            "Restore",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Restore/mod.rs"),
            vec![],
        ),
        (
            "RightsManagement",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Data/RightsManagement/mod.rs"
            ),
            vec![("DRMCheckSecurity", CrashesWindows)],
        ),
        (
            "Rpc",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Rpc/mod.rs"),
            vec![],
        ),
        (
            "Rras",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/Rras/mod.rs"
            ),
            vec![],
        ),
        (
            "Search",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Search/mod.rs"),
            vec![("SQLCloseEnumServers", CrashesWindows)],
        ),
        (
            "Security",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/mod.rs"),
            vec![
                ("GetLengthSid", CrashesWindows),
                ("EqualPrefixSid", CrashesWindows),
                ("EqualSid", CrashesWindows),
                ("CopySid", CrashesWindows),
            ],
        ),
        (
            "SecurityCenter",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SecurityCenter/mod.rs"
            ),
            vec![("WscGetAntiMalwareUri", InvalidNumberOfArguments)],
        ),
        (
            "Sensors",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Sensors/mod.rs"),
            vec![],
        ),
        (
            "SerialCommunication",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/SerialCommunication/mod.rs"
            ),
            vec![],
        ),
        (
            "Services",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Services/mod.rs"),
            vec![],
        ),
        (
            "SetupAndMigration",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SetupAndMigration/mod.rs"
            ),
            vec![],
        ),
        (
            "Shutdown",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Shutdown/mod.rs"),
            vec![],
        ),
        (
            "Sip",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Cryptography/Sip/mod.rs"
            ),
            vec![],
        ),
        (
            "Snmp",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/Snmp/mod.rs"
            ),
            vec![],
        ),
        (
            "SqlLite",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SqlLite/mod.rs"),
            vec![],
        ),
        (
            "StationsAndDesktops",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/StationsAndDesktops/mod.rs"
            ),
            vec![],
        ),
        (
            "StructuredStorage",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/StructuredStorage/mod.rs"
            ),
            vec![("PropStgNameToFmtId", InvalidNumberOfArguments)],
        ),
        (
            "SubsystemForLinux",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SubsystemForLinux/mod.rs"
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SystemInformation/mod.rs"
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
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SystemServices/mod.rs"
            ),
            vec![],
        ),
        (
            "TabletPC",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/TabletPC/mod.rs"),
            vec![],
        ),
        (
            "Tapi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Tapi/mod.rs"),
            vec![],
        ),
        (
            "TextServices",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/TextServices/mod.rs"),
            vec![],
        ),
        (
            "Time",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Time/mod.rs"),
            vec![],
        ),
        (
            "ToolHelp",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Diagnostics/ToolHelp/mod.rs"
            ),
            vec![],
        ),
        (
            "Touch",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/Touch/mod.rs"),
            vec![],
        ),
        (
            "TpmBaseServices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/TpmBaseServices/mod.rs"
            ),
            vec![("GetDeviceIDString", CrashesWindows)],
        ),
        (
            "UI",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Cryptography/UI/mod.rs"
            ),
            vec![],
        ),
        (
            "UI2",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Authorization/UI/mod.rs"
            ),
            vec![],
        ),
        (
            "Usb",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/Usb/mod.rs"),
            vec![],
        ),
        (
            "UserAccessLogging",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/UserAccessLogging/mod.rs"
            ),
            vec![],
        ),
        (
            "Vhd",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Vhd/mod.rs"),
            vec![],
        ),
        (
            "Vss",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Vss/mod.rs"),
            vec![],
        ),
        (
            "WNet",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WNet/mod.rs"
            ),
            vec![],
        ),
        (
            "WebDav",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WebDav/mod.rs"
            ),
            vec![],
        ),
        (
            "WebServicesOnDevices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/WebServicesOnDevices/mod.rs"
            ),
            vec![],
        ),
        (
            "WebSocket",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/WebSocket/mod.rs"),
            vec![],
        ),
        (
            "WiFi",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WiFi/mod.rs"
            ),
            vec![],
        ),
        (
            "WinHttp",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/WinHttp/mod.rs"),
            vec![],
        ),
        (
            "WinInet",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/WinInet/mod.rs"),
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
            ],
        ),
        (
            "WinML",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/AI/MachineLearning/WinML/mod.rs"
            ),
            vec![],
        ),
        (
            "WinSock",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/WinSock/mod.rs"),
            vec![],
        ),
        (
            "WinTrust",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/WinTrust/mod.rs"),
            vec![("OpenPersonalTrustDBDialog", ShowsDialogWindows)],
        ),
        (
            "WindowsFilteringPlatform",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsFilteringPlatform/mod.rs"
            ),
            vec![("FwpmDynamicKeywordUnsubscribe0", NotImplementedWindows)],
        ),
        (
            "WindowsFirewall",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsFirewall/mod.rs"
            ),
            vec![],
        ),
        (
            "WindowsMediaFormat",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/WindowsMediaFormat/mod.rs"
            ),
            vec![],
        ),
        (
            "WindowsNetworkVirtualization",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsNetworkVirtualization/mod.rs"
            ),
            vec![],
        ),
        (
            "WindowsWebServices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/WindowsWebServices/mod.rs"
            ),
            vec![("WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable", InvalidNumberOfArguments)],
        ),
        (
            "Wmi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Wmi/mod.rs"),
            vec![],
        ),
        (
            "XAudio2",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/Audio/XAudio2/mod.rs"),
            vec![],
        ),
        (
            "XboxController",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/XboxController/mod.rs"
            ),
            vec![],
        ),
        (
            "XmlLite",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Data/Xml/XmlLite/mod.rs"),
            vec![],
        ),
        (
            "Xps",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Xps/mod.rs"),
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
