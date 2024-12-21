use artisan_middleware::{
    identity::Identifier, resource_monitor::get_system_stats, version::aml_version
};
use dusa_collection_utils::{log::{set_log_level, LogLevel}, stringy::Stringy, version::Version};
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
    let identifier: Identifier = Identifier::new().await.unwrap();
    identifier.save_to_file().unwrap();

    let id_info: Identifier = identifier;
    let ais_version: Version = aml_version();
    let system_version: LsbRelease = lsb_release::info().unwrap_or(lsb_failsafe);
    let system_hostname = gethostname::gethostname();

    // Add color to key parts of the welcome text using colored
    let welcome_text = format!(
        r#"
{header}

{subtitle}

machine id       : {machine_id}
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
        
        subtitle = "Your machine at a glance:".bold().cyan(),
        machine_id = id_info.id.to_string().bold().purple(),
        os_version = format!("{} - {}", system_version.version, system_version.code_name).bold().cyan(),
        ais_version = ais_version.to_string().bold().cyan(),
        // welcome_version = welcome_version.to_string().bold().cyan(),
        hostname = format!("{:?}", system_hostname).bold().cyan(),
        mem_usage = system.get(&Stringy::from("Used RAM")).unwrap_or(&Stringy::from("X.xx")).bold().cyan(),
        greeting = "Welcome!".bold().green(),
        support_message = "This server is hosted by Artisan Hosting. If you're reading this now would probably be a good time to get in contact your support engineer !!!
These environments rely on a lot of automation and software that's been designed for your specific use case. In other words this may break quickly if you aren't aware
of the ais platform, software and systems".bold().to_uppercase().bright_green(),
// to contact me at dwhitfield@artisanhosting.net or shoot me a text at 414-578-0988. Thank you for supporting me and Artisan Hosting.".bold().white()
    );

    println!("{}", welcome_text);
}
