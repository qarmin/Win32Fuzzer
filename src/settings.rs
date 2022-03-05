use std::collections::BTreeMap;

const WINDOWS_RS_FOLDER: &str = "/home/rafal/test/windows-rs/";
pub const DISABLED_CLASSES: &[&str] = &[
    "Gaming", // To run require(at least) api-ms-win-gaming-expandedresources-l1-1-0.dll
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
            "CompositionSwapchain",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/CompositionSwapchain/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct2D",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct2D/mod.rs"),
            vec![],
        ),
        (
            "Direct3D_Fxc",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D/Fxc/mod.rs"
            ),
            vec![],
        ),
        (
            "Direct3D_Dxc",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D/Dxc/mod.rs"
            ),
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
            "Direct3D10",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Direct3D10/mod.rs"),
            vec![],
        ),
        (
            "Direct3D11",
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
            "DirectDraw",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectDraw/mod.rs"),
            vec![],
        ),
        (
            "DirectComposition",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectComposition/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectManipulation",
            format!(
                "{}{}",
                WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectManipulation/mod.rs"
            ),
            vec![],
        ),
        (
            "DirectWrite",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DirectWrite/mod.rs"),
            vec![],
        ),
        (
            "Dwm",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Dwm/mod.rs"),
            vec![
                "DwmRegisterThumbnail",  // Expected 2 arguments, found 3
                "DwmSetDxFrameDuration", // Not implemented
            ],
        ),
        (
            "DXCore",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/DXCore/mod.rs"),
            vec![],
        ),
        (
            "Dxgi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Dxgi/mod.rs"),
            vec![
                "DXGIDeclareAdapterRemovalSupport", // Not Implemented
            ],
        ),
        (
            "Gdi",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Gdi/mod.rs"),
            vec![],
        ),
        (
            "Hlsl",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Hlsl/mod.rs"),
            vec![],
        ),
        (
            "Imaging",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/Imaging/mod.rs"),
            vec![],
        ),
        (
            "OpenGL",
            format!("{}{}", WINDOWS_RS_FOLDER, "crates/libs/sys/src/Windows/Win32/Graphics/OpenGL/mod.rs"),
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
