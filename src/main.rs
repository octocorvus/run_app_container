use clap::Parser;
use run_app_container::{
    app_container::AppContainerProfile, cli::Cli, logger::Logger, process::IsolatedProcess,
};

static LOGGER: Logger = Logger;

fn main() {
    let cli = Cli::parse();

    log::set_logger(&LOGGER).unwrap();
    Logger::set_log_level(match cli.debug {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    });
    log::debug!("{:?}", cli);

    //let prefix = get_app_container_prefix();
    //let suffix = get_app_container_suffix();
    //let container_name = prefix + "." + &suffix;
    let container_name = cli.container_name;

    let app_container_profile =
        match AppContainerProfile::new(&container_name, &container_name, &container_name) {
            Ok(profile) => profile,
            Err(error) => {
                log::warn!("Failed to create AppContainerProfile: {}", error);
                log::info!("Trying to get existing AppContainerProfile");
                match AppContainerProfile::derive_from_name(&container_name) {
                    Ok(profile) => profile,
                    Err(error) => {
                        log::error!("Failed to get existing AppContainerProfile: {}", error);
                        return;
                    }
                }
            }
        };
    log::debug!("AppContainer SID is {:?}", app_container_profile.sid);

    // TODO: Set appropriate permissions before launch.

    // let exe_path = "\"C:\\Windows\\notepad.exe\"";
    log::info!("Trying to launch {}", cli.application_name);
    let _process = match IsolatedProcess::run(
        &cli.application_name,
        &cli.command_line,
        app_container_profile,
    ) {
        Ok(process) => {
            log::debug!("Process info: {:?}", process);
            process
        }
        Err(error) => {
            log::error!("Failed to run process: {}", error);
            return;
        }
    };

    // TODO: Wait for the process to complete.

    // TODO: Remove the permissions that are granted.
}
