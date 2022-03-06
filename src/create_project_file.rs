use crate::FileData;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

pub fn create_project_file(
    file_data: &FileData,
    file_path: &str,
    create_renames: &HashMap<&str, &str>,
    ignored_functions: &HashSet<String>,
    ignored_arguments: &mut BTreeMap<String, u32>,
) {
    let header = r###"use windows::{Win32::Foundation::*, Win32::Graphics::Printing::*};
use windows::Win32::Data::RightsManagement::*;
use windows::Win32::Devices::AllJoyn::*;
use windows::Win32::Devices::BiometricFramework::*;
use windows::Win32::Devices::Bluetooth::*;
use windows::Win32::Devices::Communication::*;
use windows::Win32::Devices::DeviceAndDriverInstallation::*;
use windows::Win32::Devices::Display::*;
use windows::Win32::Devices::Fax::*;
use windows::Win32::Devices::HumanInterfaceDevice::*;
use windows::Win32::Devices::Sensors::*;
use windows::Win32::Devices::Tapi::*;
use windows::Win32::Devices::Usb::*;
use windows::Win32::Gaming::*;
use windows::Win32::Globalization::*;
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::Dwm::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::Graphics::Printing::PrintTicket::*;
use windows::Win32::Management::MobileDeviceManagementRegistration::*;
use windows::Win32::Media::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::Media::MediaFoundation::*;
use windows::Win32::Media::Multimedia::*;
use windows::Win32::NetworkManagement::Dhcp::*;
use windows::Win32::NetworkManagement::Dns::*;
use windows::Win32::NetworkManagement::IpHelper::*;
use windows::Win32::NetworkManagement::Multicast::*;
use windows::Win32::NetworkManagement::NetManagement::*;
use windows::Win32::NetworkManagement::NetShell::*;
use windows::Win32::NetworkManagement::P2P::*;
use windows::Win32::NetworkManagement::QoS::*;
use windows::Win32::NetworkManagement::Rras::*;
use windows::Win32::NetworkManagement::Snmp::*;
use windows::Win32::NetworkManagement::WNet::*;
use windows::Win32::NetworkManagement::WebDav::*;
use windows::Win32::NetworkManagement::WiFi::*;
use windows::Win32::NetworkManagement::WindowsConnectionManager::*;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::*;
use windows::Win32::NetworkManagement::WindowsFirewall::*;
use windows::Win32::NetworkManagement::WindowsNetworkVirtualization::*;
use windows::Win32::Networking::ActiveDirectory::*;
use windows::Win32::Networking::Clustering::*;
use windows::Win32::Networking::HttpServer::*;
use windows::Win32::Networking::Ldap::*;
use windows::Win32::Networking::WinHttp::*;
use windows::Win32::Networking::WinInet::*;
use windows::Win32::Networking::WinSock::*;
use windows::Win32::Networking::WindowsWebServices::*;
use windows::Win32::Security::*;
use windows::Win32::Security::Authentication::Identity::*;
use windows::Win32::Security::Authorization::*;
use windows::Win32::Security::Credentials::*;
use windows::Win32::Security::Cryptography::*;
use windows::Win32::Security::EnterpriseData::*;
use windows::Win32::Security::ExtensibleAuthenticationProtocol::*;
use windows::Win32::Security::Isolation::*;
use windows::Win32::Security::WinTrust::*;
use windows::Win32::Storage::CloudFilters::*;
use windows::Win32::Storage::DistributedFileSystem::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::Storage::Imapi::*;
use windows::Win32::Storage::InstallableFileSystems::*;
use windows::Win32::Storage::IscsiDisc::*;
use windows::Win32::Storage::Jet::*;
use windows::Win32::Storage::OfflineFiles::*;
use windows::Win32::Storage::Packaging::Appx::*;
use windows::Win32::Storage::ProjectedFileSystem::*;
use windows::Win32::Storage::Vhd::*;
use windows::Win32::System::AddressBook::*;
use windows::Win32::System::Antimalware::*;
use windows::Win32::System::ApplicationInstallationAndServicing::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::Com::StructuredStorage::*;
use windows::Win32::System::Com::Urlmon::*;
use windows::Win32::System::ComponentServices::*;
use windows::Win32::System::Console::*;
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::DeploymentServices::*;
use windows::Win32::System::DeveloperLicensing::*;
use windows::Win32::System::Diagnostics::*;
use windows::Win32::System::Diagnostics::Ceip::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::Diagnostics::Etw::*;
use windows::Win32::System::Environment::*;
use windows::Win32::System::ErrorReporting::*;
use windows::Win32::System::EventCollector::*;
use windows::Win32::System::EventLog::*;
use windows::Win32::System::EventNotificationService::*;
use windows::Win32::System::GroupPolicy::*;
use windows::Win32::System::HostComputeNetwork::*;
use windows::Win32::System::HostComputeSystem::*;
use windows::Win32::System::Hypervisor::*;
use windows::Win32::System::IO::*;
use windows::Win32::System::JobObjects::*;
use windows::Win32::System::Js::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Mailslots::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Ole::*;
use windows::Win32::System::Performance::*;
use windows::Win32::System::Pipes::*;
use windows::Win32::System::Power::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Recovery::*;
use windows::Win32::System::Registry::*;
use windows::Win32::System::RemoteDesktop::*;
use windows::Win32::System::RestartManager::*;
use windows::Win32::System::Rpc::*;
use windows::Win32::System::Search::*;
use windows::Win32::System::SecurityCenter::*;
use windows::Win32::System::Services::*;
use windows::Win32::System::Shutdown::*;
use windows::Win32::System::SqlLite::*;
use windows::Win32::System::StationsAndDesktops::*;
use windows::Win32::System::SubsystemForLinux::*;
use windows::Win32::System::SystemInformation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::TpmBaseServices::*;
use windows::Win32::System::UserAccessLogging::*;
use windows::Win32::System::WinRT::*;
use windows::Win32::System::WindowsProgramming::*;
use windows::Win32::UI::Accessibility::*;
use windows::Win32::UI::ColorSystem::*;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::Controls::Dialogs::*;
use windows::Win32::UI::HiDpi::*;
use windows::Win32::UI::Input::Ime::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::Input::Pointer::*;
use windows::Win32::UI::Magnification::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::Shell::PropertiesSystem::*;
use windows::Win32::UI::TextServices::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Web::MsHtml::*;
use windows::Win32::UI::Input::XboxController::*;
use windows::Win32::Media::WindowsMediaFormat::*;
use windows::Win32::Devices::WebServicesOnDevices::*;
use windows::Win32::System::SetupAndMigration::*;
use windows::Win32::Devices::SerialCommunication::*;
use windows::Win32::System::Performance::HardwareCounterProfiling::*;
use windows::Win32::Graphics::DirectComposition::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Storage::Compression::*;
use windows::Win32::Security::Cryptography::Catalog::*;
use windows::Win32::Security::AppLocker::*;
use windows::Win32::Security::Cryptography::*;
use windows::Win32::Security::DiagnosticDataQuery::*;
use windows::Win32::UI::Xaml::Diagnostics::*;
use windows::Win32::Graphics::Imaging::*;
use windows::Win32::Security::Cryptography::Sip::*;
use windows::Win32::UI::Input::Touch::*;
use windows::Win32::Storage::Xps::*;
use windows::Win32::Security::Cryptography::Certificates::*;
use windows::Win32::UI::InteractionContext::*;
use windows::Win32::Storage::FileHistory::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use windows::core::{GUID, PCSTR, PCWSTR};
use crate::basic_data::*;
use crate::more_bad_data::*;
use rand::prelude::*;
"###;

    // <<functions>>
    // <<number_of_functions>>
    // <<class>>
    let base_function = r###"""
pub fn z_<<function_name>>(file: &mut File, st: &SettingsTaker) {
    let functions: [(fn(&mut File) -> (), &str, &str); <<number_of_functions>>] = [<<functions>>]; // function, function_name_in_rust, function_name 

    println!("////////// Class <<class>>");

    let mut functions_to_check: Vec<(fn(&mut File, &SettingsTaker) -> (), &str, &str)> = Vec::new();    
    if !st.allowed_functions.is_empty() {
        functions
            .into_iter()
            .filter(|e| st.allowed_functions.contains(&e.2.to_string()))
            .for_each(|e| functions_to_check.push(e));
    } else {
        functions
            .into_iter()
            .filter(|e| !st.ignored_functions.contains(&e.2.to_string()))
            .for_each(|e| functions_to_check.push(e));
    }
     
    let number_of_function = if st.number_of_max_executed_function >= 0 {
        st.number_of_max_executed_function as usize
    } else {
        functions_to_check.len() * st.repeating_number as usize
    };
    
    // Random by default
    for _i in 0..number_of_function {
        // Missing some random functions
        if rand::random::<bool>() {
            let function = functions_to_check.choose(&mut rand::thread_rng()).unwrap().0;
            function(file);
        }
    }    
}
    "###;

    // <<<function_name>>>
    let single_function = r###"
    pub fn a_<<_<<<function_name>>>()  {

        "###;

    let file_name = format!("WinProject/src/z_{}.rs", file_path.to_lowercase());

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_name).unwrap();
    let mut file = BufWriter::new(file);

    writeln!(file, "{}", header.replace("<<<function_name>>>", &file_path.to_lowercase())).unwrap();

    let mut function_list: String = Default::default();
    let mut number_of_functions = 0;

    for (function_name, arguments) in &file_data.functions {
        // Check if function is supported
        let mut is_supported: bool = true;

        let mut function_info = FunctionInfo::new();

        if ignored_functions.contains(function_name) {
            continue;
        }

        for arg in arguments {
            let mut add_arg = ArgumentAdditional::new();
            add_arg.original_argument = arg.clone();

            if let Some(right_space) = arg.rfind(' ') {
                add_arg.argument_before = arg[..right_space].to_string();
                add_arg.argument_type = arg[right_space + 1..].to_string();
            } else {
                add_arg.argument_type = arg.clone();
            }

            if arg.contains("*const") || arg.contains("ffi::c_void") || arg.contains("*mut *mut ") {
                // println!("Not supported '''{}'''", arg); // Maybe TODO
                // add_to_ignore_arguments(ignored_arguments, arg); // TODO Renable this later
                is_supported = false;
            } else if arg.contains("*mut *mut ") {
                // println!("Not supported '''{}''' twice *mut", arg); // Maybe TODO
                add_to_ignore_arguments(ignored_arguments, arg);
                is_supported = false;
            } else if let Some(function_creator) = create_renames.get(add_arg.argument_type.as_str()) {
                // println!("Supported '''{}'''", arg);
                add_arg.function_name = function_creator.to_string();
                function_info.arguments.push(add_arg);
            } else {
                // println!("Not supported '''{}''' due missing function", arg);
                add_to_ignore_arguments(ignored_arguments, arg);
                is_supported = false;
            }
        }
        if !is_supported {
            continue;
        }

        let mut execute_arguments = "".to_string();
        let mut creation_of_arguments = "".to_string();
        for (index, additional_arguments) in function_info.arguments.iter().enumerate() {
            function_list += &format!("(a_{},\"{}\",\"{}\"),", file_path.to_lowercase(), file_path, function_name);
            if additional_arguments.argument_before.contains("mut") {
                creation_of_arguments += &format!("\t\tlet mut argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\t\tlet argument_{} = (&mut argument_{}.0,argument_{}.1);\n", index, index, index);
            } else {
                creation_of_arguments += &format!("\t\tlet argument_{} = {}();\n", index, additional_arguments.function_name);
            }

            execute_arguments += &format!("argument_{}.0", index);
            if arguments.len() - 1 != index {
                execute_arguments += ",";
            }
        }
        function_list.pop();
        number_of_functions += 1;

        writeln!(
            file,
            "\t\tprintln!(\"_____ Executing function \\\"{}\\\" from class \\\"{}\\\"\");",
            function_name, file_path
        )
        .unwrap();
        write!(file, "{}", creation_of_arguments).unwrap();
        writeln!(file, "\t\t{}({});\n", function_name, execute_arguments).unwrap();
    }

    write!(
        file,
        "{}",
        base_function
            .replace("<<functions>>", &function_list)
            .replace("<<number_of_functions>>", &number_of_functions.to_string())
            .replace("<<class>>", file_path),
    )
    .unwrap();

    // writeln!(file, "{}", footer).unwrap();
}
pub fn add_to_ignore_arguments(btreemap: &mut BTreeMap<String, u32>, key: &str) {
    if btreemap.contains_key(key) {
        *btreemap.get_mut(key).unwrap() += 1;
    } else {
        btreemap.insert(key.to_string(), 1);
    }
}

struct FunctionInfo {
    pub arguments: Vec<ArgumentAdditional>,
}

impl FunctionInfo {
    pub fn new() -> Self {
        FunctionInfo { arguments: Vec::new() }
    }
}

struct ArgumentAdditional {
    pub function_name: String,     // e.g. take_AABB
    pub original_argument: String, // E.g. *mut AABB
    pub argument_type: String,     // Like AABB from *AABB
    pub argument_before: String,   // Like *mut from *mut AABB
}
impl ArgumentAdditional {
    pub fn new() -> Self {
        ArgumentAdditional {
            function_name: "".to_string(),
            original_argument: "".to_string(),
            argument_type: "".to_string(),
            argument_before: "".to_string(),
        }
    }
}
