use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

use crate::TypeOfProblem::{CrashesLinux, CrashesWindows, NotImplementedLinux};
use crate::{FileData, TypeOfProblem, IGNORE_INVALID};

pub fn create_project_file(
    file_data: &FileData,
    class_name: &str,
    basic_renames: &HashMap<&str, &str>,
    automatic_renames: &HashMap<&str, String>,
    ignored_functions: &HashMap<String, TypeOfProblem>,
    ignored_arguments: &mut BTreeMap<String, u32>,
    non_creatable_arguments: &HashSet<&str>,
    strange_renames: &HashMap<&str, &str>,
) -> Vec<String> {
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
use windows::Win32::Graphics::Direct3D10::*;
use windows::Win32::System::WinRT::Direct3D11::*;
use windows::Win32::Graphics::Direct3D::Fxc::*;
use windows::Win32::System::Com::Marshal::*;
use windows::Win32::Security::Authorization::UI::*;
use windows::Win32::Media::Audio::XAudio2::*;
use windows::Win32::Data::Xml::XmlLite::*;
use windows::Win32::Storage::Cabinets::*;
use windows::Win32::System::CorrelationVector::*;
use windows::Win32::Graphics::Direct3D9on12::*;
use windows::Win32::Media::DirectShow::*;
use windows::Win32::Security::DirectoryServices::*;
use windows::Win32::Media::DxMediaObjects::*;
use windows::Win32::System::Iis::*;
use windows::Win32::UI::Input::*;
use windows::Win32::System::Kernel::*;
use windows::Win32::Security::LicenseProtection::*;
use windows::Win32::NetworkManagement::NetBios::*;
use windows::Win32::Devices::Enumeration::Pnp::*;
use windows::Win32::System::Diagnostics::ProcessSnapshotting::*;
use windows::Win32::UI::TabletPC::*;
use windows::Win32::System::Time::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::Networking::WebSocket::*;
use windows::Win32::Devices::PortableDevices::DMProcessConfigXMLFiltered;

use windows::Win32::System::ApplicationVerifier::*;
use windows::Win32::Graphics::DXCore::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::DirectDraw::*;
use windows::Win32::System::DistributedTransactionCoordinator::*;
use windows::Win32::Graphics::Direct3D::Dxc::*;
use windows::Win32::Storage::IndexServer::*;
use windows::Win32::System::Mapi::*;
use windows::Win32::NetworkManagement::NetworkDiagnosticsFramework::*;
use windows::Win32::System::Memory::NonVolatile::*;
use windows::Win32::System::RemoteManagement::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::Security::Cryptography::UI::*;
use windows::Win32::Media::Audio::DirectSound::*;

use windows::Win32::Devices::DeviceQuery::DevCloseObjectQuery;
use windows::Win32::Devices::DeviceQuery::DevCreateObjectQuery;
use windows::Win32::Devices::DeviceQuery::DevCreateObjectQueryEx;
use windows::Win32::Devices::DeviceQuery::DevCreateObjectQueryFromId;
use windows::Win32::Devices::DeviceQuery::DevCreateObjectQueryFromIdEx;
use windows::Win32::Devices::DeviceQuery::DevCreateObjectQueryFromIds;
use windows::Win32::Devices::DeviceQuery::DevCreateObjectQueryFromIdsEx;
use windows::Win32::Devices::DeviceQuery::DevFindProperty;
use windows::Win32::Devices::DeviceQuery::DevFreeObjectProperties;
use windows::Win32::Devices::DeviceQuery::DevFreeObjects;
use windows::Win32::Devices::DeviceQuery::DevGetObjectProperties;
use windows::Win32::Devices::DeviceQuery::DevGetObjectPropertiesEx;
use windows::Win32::Devices::DeviceQuery::DevGetObjects;
use windows::Win32::Devices::DeviceQuery::DevGetObjectsEx;
use windows::Win32::Media::KernelStreaming::KsCreateAllocator2;
use windows::Win32::Media::KernelStreaming::KsCreateAllocator;
use windows::Win32::Media::KernelStreaming::KsCreateClock2;
use windows::Win32::Media::KernelStreaming::KsCreateClock;
use windows::Win32::Media::KernelStreaming::KsCreatePin2;
use windows::Win32::Media::KernelStreaming::KsCreatePin;
use windows::Win32::Media::KernelStreaming::KsCreateTopologyNode2;
use windows::Win32::Media::KernelStreaming::KsCreateTopologyNode;
use windows::Win32::Storage::OperationRecorder::OperationEnd;
use windows::Win32::Storage::OperationRecorder::OperationStart;
use windows::Win32::System::PasswordManagement::MSChapSrvChangePassword2;
use windows::Win32::System::PasswordManagement::MSChapSrvChangePassword;
use windows::Win32::System::Restore::SRSetRestorePointA;
use windows::Win32::System::Restore::SRSetRestorePointW;
use windows::Win32::System::Wmi::MI_Application_InitializeV1;

use windows::core::{GUID, PCSTR, PCWSTR};
use crate::basic_data::*;
use crate::more_bad_data::*;
use rand::prelude::*;
use std::fs::File;
"###;

    // <<functions>>
    // <<number_of_functions>>
    // <<class_lowercase>>
    // <<class>>
    let base_function = r###"
pub fn z_<<class_lowercase>>(file: &mut File, st: &SettingsTaker) {
    let functions: [(fn(&mut File) -> (), &str); <<number_of_functions>>] = [<<functions>>]; // function, function_name_in_rust, function_name 

    // println!("////////// Class <<class>>");

    let mut functions_to_check: Vec<(fn(&mut File) -> (), &str)> = Vec::new();
    if !st.allowed_functions.is_empty() {
        functions
            .into_iter()
            .filter(|e| st.allowed_functions.contains(&e.1.to_string()))
            .for_each(|e| functions_to_check.push(e));
    } else {
        functions
            .into_iter()
            .filter(|e| !st.ignored_functions.contains(&e.1.to_string()))
            .for_each(|e| functions_to_check.push(e));
    }
     
    let number_of_function = if st.number_of_max_executed_function >= 0 {
        st.number_of_max_executed_function as usize
    } else {
        functions_to_check.len()
    };
    
    let repeating_number = if st.repeating_number > 0{
        st.repeating_number
    } else {
        1
    };
    
    // Random by default
    for i in 0..number_of_function {
        // Missing some random functions
        if st.random && rand::random::<bool>() {
            let function = functions_to_check.choose(&mut rand::thread_rng()).unwrap().0;
            for _z in 0..repeating_number{
                function(file);
            }
        } else if !st.random {
            let function = functions_to_check[i % functions_to_check.len()].0;
            for _z in 0..repeating_number{
                function(file);
            }
        }
    }    
}
    "###;

    // <<function_name>>
    // <<println>> - println which prints info about currently executed function
    // <<arguments>> - creating arguments
    // <<print_execution_code>> - Print about executing function
    // <<execution_code>> - Executing function
    let single_function = r###"
pub fn a_<<function_name>>(file : &mut File)  {
	<<println>>
<<arguments>>
    unsafe {
	    <<print_execution_code>>
	    <<execution_code>>  }
}"###;

    let file_name = format!("WinProject/src/z_{}.rs", class_name.to_lowercase());

    let _ = fs::remove_file(&file_name);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_name).unwrap();
    let mut file = BufWriter::new(file);

    writeln!(file, "{}", header.replace("<<<function_name>>>", &class_name.to_lowercase())).unwrap();

    let mut used_functions: Vec<_> = Vec::new();

    let mut function_list: String = Default::default();
    let mut number_of_functions = 0;
    let mut each_function_body: Vec<String> = Vec::new();

    for (function_name, arguments) in &file_data.functions {
        // Check if function is supported
        let mut is_supported: bool = true;

        let mut function_info = FunctionInfo::new();

        for arg in arguments {
            let mut add_arg = ArgumentAdditional::new();
            add_arg.original_argument = arg.clone();

            if let Some(right_space) = arg.rfind(' ') {
                add_arg.argument_before = arg[..right_space].to_string();
                add_arg.argument_type = arg[right_space + 1..].to_string();
            } else {
                add_arg.argument_type = arg.clone();
            }
            let spl: Vec<_> = add_arg.argument_type.split("::").collect();
            add_arg.latest_value = spl[spl.len() - 1].to_string();

            if let Some(function_creator) = basic_renames.get(add_arg.argument_type.as_str()) {
                // println!("Supported '''{}'''", arg);
                add_arg.function_name = function_creator.to_string();
                function_info.arguments.push(add_arg);
            } else {
                if non_creatable_arguments.contains(add_arg.latest_value.as_str()) {
                    // add_to_ignore_arguments(ignored_arguments, &add_arg.argument_type);
                    is_supported = false;
                    continue;
                };
                if let Some(function_name) = automatic_renames.get(add_arg.latest_value.as_str()) {
                    // println!("AUTOMATIC SUPPORTED: {} --- {}", add_arg.latest_value, function_name);
                    add_arg.function_name = function_name.clone();
                    function_info.arguments.push(add_arg);
                } else if let Some(function_name) = strange_renames.get(add_arg.latest_value.as_str()) {
                    // println!("AUTOMATIC SUPPORTED: {} --- {}", add_arg.latest_value, function_name);
                    add_arg.function_name = function_name.to_string();
                    function_info.arguments.push(add_arg);
                } else {
                    // println!("Not supported '''{}''' due missing function", arg);
                    add_to_ignore_arguments(ignored_arguments, &add_arg.argument_type);
                    is_supported = false;
                    continue;
                }
            }
        }
        if !is_supported {
            continue;
        }

        if let Some(f) = ignored_functions.get(function_name) {
            if IGNORE_INVALID || ![NotImplementedLinux, CrashesLinux, CrashesWindows].contains(f) {
                continue;
            }
            if *f == TypeOfProblem::Other {
                continue;
            }
        }

        used_functions.push(function_name.clone());

        let mut execute_arguments = "".to_string();
        let mut creation_of_arguments = "".to_string();
        function_list += &format!("(a_{},\"{}\"),", function_name, function_name);
        number_of_functions += 1;
        for (index, additional_arguments) in function_info.arguments.iter().enumerate() {
            if additional_arguments.argument_before.contains("*mut") {
                creation_of_arguments += &format!("\tlet mut argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let mut argument_{} = {{}};\", argument_{}.1));\n", index, index);
                let how_many = additional_arguments.argument_before.matches("*mut").count();
                let mut mut_iteration = "".to_string();

                for _i in 0..how_many {
                    mut_iteration += "*mut ";
                    creation_of_arguments += &format!("\tlet mut argument_{} : ({}_,_) = (&mut argument_{}.0,argument_{}.1);\n", index, mut_iteration, index, index);
                    creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let mut argument_{} : {} _ = {{}};\", argument_{}.1));\n", index, mut_iteration, index);
                }
            } else if additional_arguments.argument_before.contains("mut") {
                creation_of_arguments += &format!("\tlet mut argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let mut argument_{} = {{}};\", argument_{}.1));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} = (&mut argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} = &mut argument{};\"));\n", index, index);
            } else if additional_arguments.argument_before.contains("*const *const *const") {
                creation_of_arguments += &format!("\tlet argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} = {{}};\", argument_{}.1));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} : (*const _, _) = (&argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} : *const _ = &argument{};\"));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} : (*const *const _, _) = (&argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} : *const *const _ = &argument{};\"));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} : (*const *const *const _, _) = (&argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} : *const *const *const _ = &argument{};\"));\n", index, index);
            } else if additional_arguments.argument_before.contains("*const *const") {
                creation_of_arguments += &format!("\tlet argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} = {{}};\", argument_{}.1));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} : (*const _, _) = (&argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} : *const _ = &argument{};\"));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} : (*const *const _, _) = (&argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} : *const *const _ = &argument{};\"));\n", index, index);
            } else if additional_arguments.argument_before.contains("const") {
                creation_of_arguments += &format!("\tlet argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} = {{}};\", argument_{}.1));\n", index, index);
                creation_of_arguments += &format!("\tlet argument_{} : (*const _, _) = (&argument_{}.0,argument_{}.1);\n", index, index, index);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} : *const _ = &argument{};\"));\n", index, index);
            } else {
                creation_of_arguments += &format!("\tlet argument_{} = {}();\n", index, additional_arguments.function_name);
                creation_of_arguments += &format!("\tprint_and_save(file,format!(\"let argument_{} = {{}};\", argument_{}.1));\n", index, index);
            }

            execute_arguments += &format!("argument_{}.0", index);
            if arguments.len() - 1 != index {
                execute_arguments += ",";
            }
        }

        let print = format!(
            "print_and_save(file, \"// Executing function \\\"{}\\\" from class \\\"{}\\\"\".to_string());",
            function_name, class_name
        );
        let executed_arguments = format!("{}({});\n", function_name, execute_arguments);
        let print_executed_arguments = format!("print_and_save(file,format!(\"{}({});\"));", function_name, execute_arguments.replace(".0", ""));

        each_function_body.push(
            single_function
                .replace("<<function_name>>", function_name)
                .replace("<<println>>", &print)
                .replace("<<arguments>>", &creation_of_arguments)
                .replace("<<print_execution_code>>", &print_executed_arguments)
                .replace("<<execution_code>>", &executed_arguments),
        );
    }
    for i in each_function_body {
        writeln!(file, "{}", i).unwrap();
    }

    function_list.pop(); // Remove latest ,
    write!(
        file,
        "{}",
        base_function
            .replace("<<functions>>", &function_list)
            .replace("<<number_of_functions>>", &number_of_functions.to_string())
            .replace("<<class_lowercase>>", &class_name.to_lowercase())
            .replace("<<class>>", class_name),
    )
    .unwrap();

    // writeln!(file, "{}", footer).unwrap();
    used_functions
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
    pub function_name: String,     // e.g. "take_AABB"
    pub original_argument: String, // E.g. "*mut AABB"
    pub argument_type: String,     // Like "AABB" from "*AABB"
    pub argument_before: String,   // Like "*mut" from "*mut AABB"
    pub latest_value: String,      // "POINT" from "roman::POINT"
}

impl ArgumentAdditional {
    pub fn new() -> Self {
        ArgumentAdditional {
            function_name: "".to_string(),
            original_argument: "".to_string(),
            argument_type: "".to_string(),
            argument_before: "".to_string(),
            latest_value: "".to_string(),
        }
    }
}
