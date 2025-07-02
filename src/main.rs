use artisan_middleware::{
    identity::Identifier, resource_monitor::get_system_stats, version::aml_version
};
use artisan_middleware::dusa_collection_utils::{logger::{set_log_level, LogLevel}, types::stringy::Stringy, version::Version};
use lsb_release::LsbRelease;
use colored::*;  // Add the colored crate for text colorization

#[tokio::main]
async fn main() {
    set_log_level(LogLevel::Trace);
    let system: std::collections::HashMap<Stringy, Stringy> = get_system_stats();
    let lsb_failsafe: LsbRelease = LsbRelease {
        id: String::from("failsafe"),
        desc: String::from("System in a damaged state"),
        version: aml_version().to_string(),
        code_name: String::from("Wacky Whitfield"),
    };

    // Create new identifier if none 
    let identifier: Option<Identifier> = match Identifier::load_from_file() {
        Ok(data) => Some(data),
        Err(_) => None,
    };

    // identifier.save_to_file().unwrap();

    let id_info: String = if let Some(id) = identifier {
        id.id.to_string()
    } else {
        "No identity file loaded".to_string()
    };

    let ais_version: Version = aml_version();
    let system_version: LsbRelease = lsb_release::info().unwrap_or(lsb_failsafe);
    let system_hostname = gethostname::gethostname();

    // Add color to key parts of the welcome text using colored
    let welcome_text = format!(
        r#"
{header}

{subtitle}

Node id       : {machine_id}
Os Information   : {os_version}
Artisan Library  : {ais_version}
Hostname         : {hostname}
Memory used      : {mem_usage}

{greeting}

{support_message}

"#,
        header = format!(r#"
                 _    _                         _    _                   _
    /\          | |  (_)                       | |  | |                 (_) 
   /  \    _ __ | |_  _  ___   __ _  _ __      | |__| |  ___   ___ | |_     _ __    __ _
  / /\ \  | '__|| __|| |/ __| / _` || '_ \     | '__' | / _ \ /`__|| __|| || '_ \  / _` |
 / ____ \ | |   | |_ | |\__ \| (_| || | | |    | |  | || (_) |\__ \| |_ | || | | || (_| |
/_/    \_\|_|    \__||_||___/ \__,_||_| |_|    |_|  |_| \___/ |___/ \__||_||_| |_| \__, |
                                                                                    __/ |
                                                                                   |___/   
        "#).bold().blue(),
        
        subtitle = "Node at a glance:".bold().cyan(),
        machine_id = id_info.bold().purple(),
        os_version = format!("{} - {}", system_version.version, system_version.code_name).bold().cyan(),
        ais_version = ais_version.to_string().bold().cyan(),
        hostname = format!("{:?}", system_hostname).bold().cyan(),
        mem_usage = system.get(&Stringy::from("Used RAM")).unwrap_or(&Stringy::from("X.xx")).bold().cyan(),
        greeting = "Welcome!".bold().green(),
        support_message = "This server is hosted by Artisan Hosting. If you're reading this you are one of two people. 1. A poor bastard trying to help me fix some 
virtual machine issues. 2. A curious bastard trying to poke around and break things. In any case to both of you, Good luck, things may break quickly if, you
haven't read the poor and sparse documentation i've \"written\".".bold().bright_green(),
    );

    println!("{}", welcome_text);
}
