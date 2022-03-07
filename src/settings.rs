use std::collections::BTreeMap;

pub const WINDOWS_RS_FOLDER: &str = "/home/rafal/test/windows-rs/";
pub const DISABLED_CLASSES: &[&str] = &[
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

pub fn load_settings() -> Vec<(&'static str, String, Vec<&'static str>)> {
    vec![
        (
            "Printer",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"),
            vec![
                "AddPrintDeviceObject",                       // Difference in number of arguments
                "CloseSpoolFileHandle",                       // Not implemented
                "CommitSpoolData",                            // Not implemented
                "DeletePrinterIC",                            // Not implemented
                "GdiGetPageCount",                            // Not implemented
                "CloseSpoolFileHandle",                       // Not implemented
                "DeleteJobNamedProperty",                     // Not implemented
                "DeletePrinterDataA",                         // Not implemented
                "DeletePrinterDataW",                         // Not implemented
                "DeletePrinterDriverPackageA",                // Not implemented
                "DeletePrinterDriverPackageW",                // Not implemented
                "DeletePrinterKeyA",                          // Not implemented
                "DeletePrinterKeyW",                          // Not implemented
                "GdiDeleteSpoolFileHandle",                   // Not implemented
                "GdiEndDocEMF",                               // Not implemented
                "GdiEndPageEMF",                              // Not implemented
                "GdiGetDC",                                   // Not implemented
                "GdiStartPageEMF",                            // Not implemented
                "InstallPrinterDriverFromPackageA",           // Not implemented
                "InstallPrinterDriverFromPackageW",           // Not implemented
                "PrinterMessageBoxA",                         // Not implemented
                "PrinterMessageBoxW",                         // Not implemented
                "ProvidorFindClosePrinterChangeNotification", // Not implemented
                "RemovePrintDeviceObject",                    // Not implemented
                "RouterAllocBidiMem",                         // Not implemented
                "RouterAllocBidiResponseContainer",           // Not implemented
                "RouterAllocPrinterNotifyInfo",               // Not implemented
                "SpoolerFindClosePrinterChangeNotification",  // Not implemented
                "UnRegisterForPrintAsyncNotifications",       // Not implemented
                "UpdatePrintDeviceObject",                    // Not implemented
                "WaitForPrinterChange",                       // Not implemented
                "GdiGetPageHandle",                           // Not implemented
                "GdiGetSpoolFileHandle",                      // Not implemented
                "GdiResetDCEMF",                              // Not implemented
                "GdiGetPageHandle",                           // Not implemented
                "GdiGetPageHandle",                           // Not implemented
                "GetPrinterDriverPackagePathA",               // Not implemented
                "GetSpoolFileHandle",                         // Not implemented
                "GetPrintOutputInfo",                         //
                "GetPrinterDriver2A",                         //
                "GetPrinterDriver2W",                         //
                "GetPrinterDriverPackagePathW",               //
                "SplIsSessionZero",                           //
                "CorePrinterDriverInstalledW",                //
                "CorePrinterDriverInstalledA",                //
            ],
        ),
        (
            "Dwm",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Dwm/mod.rs"),
            vec![
                "DwmRegisterThumbnail",        // Expected 2 arguments, found 3
                "DwmSetDxFrameDuration",       // Not implemented
                "DwmIsCompositionEnabled",     //
                "DwmGetGraphicsStreamClient",  //
                "DwmQueryThumbnailSourceSize", //
            ],
        ),
        (
            "Gaming",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Gaming/mod.rs"),
            vec![
                "GetExpandedResourceExclusiveCpuCount", // Expected 0, found 1 argument
            ],
        ),
        (
            "Dxgi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Dxgi/mod.rs"),
            vec![
                "DXGIDeclareAdapterRemovalSupport", // Not Implemented
            ],
        ),
        (
            "BiometricFramework",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Devices/BiometricFramework/mod.rs"
            ),
            vec![
                "WinBioEnrollCapture", // Invalid number of arguments
                "WinBioLocateSensor",  // Invalid number of arguments
            ],
        ),
        (
            "Com",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/mod.rs"),
            vec![
                "CoGetCallerTID",              // Invalid number of arguments
                "CoGetContextToken",           // Invalid number of arguments
                "CLSIDFromProgID",             //
                "CLSIDFromProgIDEx",           //
                "CLSIDFromString",             //
                "CoCreateGuid",                //
                "CoFileTimeNow",               //
                "CoGetCurrentLogicalThreadId", //
                "GetClassFile",                //
                "IIDFromString",               //
            ],
        ),
        (
            "CloudFilters",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/CloudFilters/mod.rs"),
            vec![
                "CfGetTransferKey", // Invalid number of arguments
            ],
        ),
        (
            "Debug",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Diagnostics/Debug/mod.rs"
            ),
            vec![
                "FatalExit",              // Crashes Everything
                "SymFunctionTableAccess", // TODO not exists, why?
                "SymGetModuleBase",       // TODO not exists, why?
                "SymLoadModule",          // TODO not exists, why?
                "SymUnloadModule",        // TODO not exists, why?
                "Beep",                   // Beeps, useless and stops executing of app
            ],
        ),
        (
            "DeploymentServices",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/DeploymentServices/mod.rs"
            ),
            vec![
                "WdsCliGetEnumerationFlags",              // Invalid number of arguments
                "WdsCliFindFirstImage",                   // Invalid number of arguments
                "WdsCliGetImageHandleFromFindHandle",     // Invalid number of arguments
                "WdsCliGetImageHandleFromTransferHandle", // Invalid number of arguments
                "WdsCliGetImageIndex",                    // Invalid number of arguments
                "WdsCliGetImageSize",                     // Invalid number of arguments
                "WdsCliGetTransferSize",                  // Invalid number of arguments
                "WdsCliGetImageVersion",                  //
                "WdsCliGetImagePath",                     //
                "WdsCliGetImageNamespace",                //
                "WdsCliGetImageName",                     //
                "WdsCliGetImageLanguage",                 //
                "WdsCliGetImageHalName",                  //
                "WdsCliGetImageGroup",                    //
                "WdsCliGetImageDescription",              //
                "WdsCliGetDriverQueryXml",                //
            ],
        ),
        (
            "FileSystem",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/FileSystem/mod.rs"),
            vec![
                "WofGetDriverVersion", // Invalid number of arguments
                "WofWimAddEntry",      // Invalid number of arguments
            ],
        ),
        (
            "Identity",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/Authentication/Identity/mod.rs"
            ),
            vec![
                "SLGetWindowsInformationDWORD", // Invalid number of arguments
            ],
        ),
        (
            "MediaFoundation",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Media/MediaFoundation/mod.rs"
            ),
            vec![
                "MFAllocateSerialWorkQueue",        //
                "MFAllocateWorkQueue",              //
                "MFFrameRateToAverageTimePerFrame", //
                "MFGetPlaneSize",                   //
                "MFGetStrideForBitmapInfoHeader",   //
                "MFGetTimerPeriodicity",            //
                "MFGetWorkQueueMMCSSPriority",      //
                "MFGetWorkQueueMMCSSTaskId",        //
            ],
        ),
        (
            "Ole",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Ole/mod.rs"),
            vec![
                "VarI4FromI8",     //
                "VarI4FromStr",    //
                "VarI4FromUI4",    //
                "VarI4FromUI8",    //
                "VarI8FromStr",    //
                "VarI8FromUI4",    //
                "VarI8FromUI4",    //
                "VarI8FromUI8",    //
                "VarUI4FromI4",    //
                "VarUI4FromI8",    //
                "VarUI4FromStr",   //
                "VarUI4FromUI8",   //
                "VarUI8FromI8",    //
                "VarUI8FromStr",   //
                "VarUI8FromUI4",   //
                "VarDateFromI4",   //
                "VarDateFromI8",   //
                "VarDateFromR4",   //
                "VarDateFromR8",   //
                "VarDateFromStr",  //
                "VarDateFromUI2",  //
                "VarDateFromUI4",  //
                "VarDateFromUI8",  //
                "VarI4FromDate",   //
                "VarI4FromR4",     //
                "VarI4FromR8",     //
                "VarI4FromUI2",    //
                "VarI8FromDate",   //
                "VarI8FromR4",     //
                "VarI8FromR8",     //
                "VarI8FromUI2",    //
                "VarR4FromDate",   //
                "VarR4FromI4",     //
                "VarR4FromI8",     //
                "VarR4FromR8",     //
                "VarR4FromStr",    //
                "VarR4FromUI2",    //
                "VarR4FromUI4",    //
                "VarR4FromUI8",    //
                "VarR8FromDate",   //
                "VarR8FromI4",     //
                "VarR8FromI8",     //
                "VarR8FromR4",     //
                "VarR8FromStr",    //
                "VarR8FromUI2",    //
                "VarR8FromUI4",    //
                "VarR8FromUI8",    //
                "VarR8Pow",        //
                "VarR8Round",      //
                "VarUI2FromDate",  //
                "VarUI2FromI4",    //
                "VarUI2FromI8",    //
                "VarUI2FromR4",    //
                "VarUI2FromStr",   //
                "VarUI2FromUI4",   //
                "VarUI2FromUI8",   //
                "VarUI4FromDate",  //
                "VarUI4FromR4",    //
                "VarUI4FromR8",    //
                "VarUI4FromUI2",   //
                "VarUI8FromDate",  //
                "VarUI8FromR4",    //
                "VarUI8FromR8",    //
                "VarUI8FromUI2",   //
                "VarBoolFromDate", //
                "VarBoolFromI2",   //
                "VarBoolFromI4",   //
                "VarBoolFromI8",   //
                "VarBoolFromR4",   //
                "VarBoolFromR8",   //
                "VarBoolFromStr",  //
                "VarBoolFromUI1",  //
                "VarBoolFromUI2",  //
                "VarBoolFromUI4",  //
                "VarBoolFromUI8",  //
                "VarDateFromBool", //
                "VarDateFromI2",   //
                "VarDateFromUI1",  //
                "VarI2FromBool",   //
                "VarI2FromDate",   //
                "VarI2FromI4",     //
                "VarI2FromI8",     //
                "VarI2FromR4",     //
                "VarI2FromR8",     //
                "VarI2FromStr",    //
                "VarI2FromUI1",    //
                "VarI2FromUI2",    //
                "VarI2FromUI4",    //
                "VarI2FromUI8",    //
                "VarI4FromBool",   //
                "VarI4FromI2",     //
                "VarI4FromUI1",    //
                "VarI8FromBool",   //
                "VarI8FromI2",     //
                "VarI8FromUI1",    //
                "VarR4FromBool",   //
                "VarR4FromI2",     //
                "VarR4FromUI1",    //
                "VarR8FromBool",   //
                "VarR8FromI2",     //
                "VarR8FromUI1",    //
                "VarUI1FromBool",  //
                "VarUI1FromDate",  //
                "VarUI1FromI2",    //
                "VarUI1FromI4",    //
                "VarUI1FromI8",    //
                "VarUI1FromR4",    //
                "VarUI1FromR8",    //
                "VarUI1FromStr",   //
                "VarUI1FromUI2",   //
                "VarUI1FromUI4",   //
                "VarUI1FromUI8",   //
                "VarUI2FromBool",  //
                "VarUI2FromI2",    //
                "VarUI2FromUI1",   //
                "VarUI4FromBool",  //
                "VarUI4FromI2",    //
                "VarUI4FromUI1",   //
                "VarUI8FromBool",  //
                "VarUI8FromI2",    //
                "VarUI8FromUI1",   //
                "VarBoolFromI1",   //
                "VarBstrFromBool", //
                "VarBstrFromDate", //
                "VarBstrFromI1",   //
                "VarBstrFromI2",   //
                "VarBstrFromI4",   //
                "VarBstrFromI8",   //
                "VarBstrFromR4",   //
                "VarBstrFromR8",   //
                "VarBstrFromUI1",  //
                "VarBstrFromUI2",  //
                "VarBstrFromUI4",  //
                "VarBstrFromUI8",  //
                "VarDateFromI1",   //
                "VarI2FromI1",     //
                "VarI4FromI1",     //
                "VarI8FromI1",     //
                "VarMonthName",    //
                "VarR4FromI1",     //
                "VarUI1FromI1",    //
                "VarUI2FromI1",    //
                "VarUI4FromI1",    //
                "VarUI8FromI1",    //
                "VarWeekdayName",  //
                "VarBoolFromCy",   //
                "VarBstrFromCy",   //
                "VarCyAbs",        //
                "VarCyAdd",        //
                "VarCyFix",        //
                "VarCyFromBool",   //
                "VarCyFromDate",   //
                "VarCyFromI1",     //
                "VarCyFromI2",     //
                "VarCyFromI4",     //
                "VarCyFromI8",     //
                "VarCyFromR4",     //
                "VarCyFromR8",     //
                "VarCyFromStr",    //
                "VarCyFromUI1",    //
                "VarCyFromUI2",    //
                "VarCyFromUI4",    //
                "VarCyFromUI8",    //
                "VarCyInt",        //
                "VarCyMul",        //
                "VarCyMulI4",      //
                "VarCyMulI8",      //
                "VarCyNeg",        //
                "VarCyRound",      //
                "VarCySub",        //
                "VarDateFromCy",   //
                "VarDecFromBool",  //
                "VarDecFromCy",    //
                "VarDecFromDate",  //
                "VarDecFromI1",    //
                "VarDecFromI2",    //
                "VarDecFromI4",    //
                "VarDecFromI8",    //
                "VarDecFromR4",    //
                "VarDecFromR8",    //
                "VarDecFromStr",   //
                "VarDecFromUI1",   //
                "VarDecFromUI2",   //
                "VarDecFromUI4",   //
                "VarDecFromUI8",   //
                "VarI4FromCy",     //
                "VarI8FromCy",     //
                "VarUI1FromCy",    //
                "VarUI2FromCy",    //
                "VarUI4FromCy",    //
                "VarUI8FromCy",    //
            ],
        ),
        (
            "P2P",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/P2P/mod.rs"
            ),
            vec![
                "PeerIdentityGetCryptKey",     //
                "PeerCollabGetSigninOptions",  //
                "PeerNameToPeerHostName",      //
                "PeerIdentityImport",          //
                "PeerIdentityGetXML",          //
                "PeerIdentityGetFriendlyName", //
                "PeerIdentityGetDefault",      //
                "PeerIdentityExport",          //
                "PeerIdentityCreate",          //
                "PeerHostNameToPeerName",      //
                "PeerCreatePeerName",          //
                "PeerCollabGetEndpointName",   //
                "PeerCollabExportContact",     //
            ],
        ),
        (
            "PrintTicket",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Printing/PrintTicket/mod.rs"
            ),
            vec![
                "PTQuerySchemaVersionSupport", //
            ],
        ),
        (
            "Printing",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Printing/mod.rs"),
            vec![
                "AddPrintDeviceObject",        // ?
                "CorePrinterDriverInstalledA", //
                "CorePrinterDriverInstalledW", //
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
                "RegisterScaleChangeEvent",                //
                "SHGetDriveMedia",                         //
                "SHIsFileAvailableOffline",                //
                "SHStrDupW",                               //
                "SHStrDupA",                               //
                "PathCchSkipRoot",                         //
                "PathCchFindExtension",                    //
                "PathAllocCombine",                        //
                "PathAllocCanonicalize",                   //
                "HlinkTranslateURL",                       //
                "HlinkGetValueFromParams",                 //
                "HlinkGetSpecialReference",                //
                "GetCurrentProcessExplicitAppUserModelID", //
                "SHCLSIDFromString",                       //
            ],
        ),
        (
            "Threading",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Threading/mod.rs"),
            vec![
                "ExitProcess",          // Just crashes app
                "ExitThread",           // Just crashes app
                "GetThreadDescription", //
            ],
        ),
        (
            "Urlmon",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Com/Urlmon/mod.rs"),
            vec![
                "IEInstallScope", //
                "FindMediaType",  //
                "GetClassURL",    //
            ],
        ),
        (
            "WinRT",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/WinRT/mod.rs"),
            vec![
                "RoGetApartmentIdentifier",            //
                "RoGetErrorReportingFlags",            //
                "WindowsCompareStringOrdinal",         //
                "WindowsConcatString",                 //
                "WindowsCreateString",                 //
                "WindowsDuplicateString",              //
                "WindowsReplaceString",                //
                "WindowsStringHasEmbeddedNull",        //
                "WindowsSubstring",                    //
                "WindowsSubstringWithSpecifiedLength", //
                "WindowsTrimStringEnd",                //
                "WindowsTrimStringStart",              //
            ],
        ),
        (
            "WindowsAndMessaging",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/WindowsAndMessaging/mod.rs"
            ),
            vec![
                "MrmGetPriFileContentChecksum", //
            ],
        ),
        (
            "WindowsConnectionManager",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsConnectionManager/mod.rs"
            ),
            vec![
                "OnDemandGetRoutingHint", //
            ],
        ),
        (
            "WindowsProgramming",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/WindowsProgramming/mod.rs"
            ),
            vec![
                "QueryAuxiliaryCounterFrequency", //
                "WldpIsDynamicCodePolicyEnabled", //
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
                "GetResolvedPackageFullNameForPackageDependency", //
                "CheckIsMSIXPackage",                             //
                "DeactivatePackageVirtualizationContext", // Not runs on Windows - Nie znaleziono punktu wejścia procedury DeactivatePackageVirtualizationContext w bibliotece
                "DeletePackageDependency",                // Not runs on Windows
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
            vec![],
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
            vec![],
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
            vec![],
        ),
        (
            "Console",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Console/mod.rs"),
            vec![],
        ),
        (
            "Controls",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Controls/mod.rs"),
            vec![
                "GetThemeTransitionDuration", //
                "GetThemeSysInt",             //
                "GetThemeInt",                //
                "GetThemeEnumValue",          //
                "GetThemeColor",              //
                "GetBufferedPaintTargetRect", //
                "GetThemeRect",               //
                "GetThemePosition",           //
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
            vec![],
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
            vec![],
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
            vec![],
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
                "DdqGetDiagnosticRecordPayload",          //
                "DdqGetDiagnosticReportStoreReportCount", //
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
            vec![
                "DCompositionBoostCompositorClock", // Not run on Windows
            ],
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
            vec![],
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
            vec![],
        ),
        (
            "ErrorReporting",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/ErrorReporting/mod.rs"
            ),
            vec![],
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
            vec![
                "FhServiceOpenPipe", //
            ],
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
                "TransparentBlt", // Not runs on Windows
            ],
        ),
        (
            "Globalization",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Globalization/mod.rs"),
            vec![
                "GetDistanceOfClosestLanguageInList", //
            ],
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
                "HcsModifyServiceSettings",                   //
                "HcsGetServiceProperties",                    //
                "HcsGetProcessorCompatibilityFromSavedState", //
                "HcsGetLayerVhdMountPath",                    //
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
            vec![],
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
            vec![
                "WICMapShortNameToGuid", //
            ],
        ),
        (
            "Imapi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Storage/Imapi/mod.rs"),
            vec![],
        ),
        (
            "Ime",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/Input/Ime/mod.rs"),
            vec![],
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
            vec![],
        ),
        (
            "InteractionContext",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/UI/InteractionContext/mod.rs"
            ),
            vec![
                "CreateInteractionContext", //
            ],
        ),
        (
            "IpHelper",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/IpHelper/mod.rs"
            ),
            vec![],
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
                "IsProcessInIsolatedWindowsEnvironment",                               //
                "IsProcessInIsolatedContainer",                                        //
                "GetAppContainerFolderPath",                                           //
                "DeriveAppContainerSidFromAppContainerName",                           //
                "DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName", //
                "GetAppContainerRegistryLocation",                                     //
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
            vec![],
        ),
        (
            "Ldap",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Networking/Ldap/mod.rs"),
            vec![],
        ),
        (
            "LibraryLoader",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/LibraryLoader/mod.rs"),
            vec![
                "FreeLibraryAndExitThread", // Looks that crashes it
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
                "RegisterDeviceWithLocalManagement", //
                "IsMdmUxWithoutAadAllowed",          //
                "IsManagementRegistrationAllowed",   //
                "ApplyLocalManagementSyncML",        //
            ],
        ),
        (
            "MsHtml",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Web/MsHtml/mod.rs"),
            vec![
                "RatingObtainQueryW", //
                "RatingObtainQuery",  //
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
            vec![],
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
            vec![],
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
            vec![],
        ),
        (
            "Pipes",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Pipes/mod.rs"),
            vec![],
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
            vec![],
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
                "PSLookupPropertyHandlerCLSID",      //
                "InitPropVariantFromResource",       //
                "InitPropVariantFromStringAsVector", //
                "InitVariantFromResource",           //
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
            vec![
                "ApplicationRecoveryInProgress", //
            ],
        ),
        (
            "Registry",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/Registry/mod.rs"),
            vec![],
        ),
        (
            "RemoteDesktop",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/RemoteDesktop/mod.rs"),
            vec![],
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
            vec![],
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
            vec![],
        ),
        (
            "Security",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Security/mod.rs"),
            vec![],
        ),
        (
            "SecurityCenter",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SecurityCenter/mod.rs"
            ),
            vec![
                "WscGetAntiMalwareUri", //
            ],
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
            vec![
                "PropStgNameToFmtId", //
            ],
        ),
        (
            "SubsystemForLinux",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SubsystemForLinux/mod.rs"
            ),
            vec![
                "WslLaunchInteractive", //
                "WslLaunch",            //
            ],
        ),
        (
            "SystemInformation",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/System/SystemInformation/mod.rs"
            ),
            vec![
                "IsWow64GuestMachineSupported", //
                "GetIntegratedDisplaySize",     //
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
            vec![],
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
                "ExportCookieFileA",              // Not runs on windows
                "ExportCookieFileW",              // Not runs on windows
                "FindP3PPolicySymbol",            // Not runs on windows
                "GetDiskInfoA",                   // Not runs on windows
                "ImportCookieFileA",              // Not runs on windows
                "ImportCookieFileW",              // Not runs on windows
                "InternalInternetGetCookie",      // Not runs on windows
                "IsDomainLegalCookieDomainA",     // Not runs on windows
                "IsDomainLegalCookieDomainW",     // Not runs on windows
                "FwpmDynamicKeywordUnsubscribe0", // Not runs on windows
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
            vec![],
        ),
        (
            "WindowsFilteringPlatform",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/NetworkManagement/WindowsFilteringPlatform/mod.rs"
            ),
            vec![
                "FwpmDynamicKeywordUnsubscribe0", // Not runs on Windows
            ],
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
            vec![
                "WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable", //
            ],
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
