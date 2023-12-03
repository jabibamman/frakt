use cli::{
    operation::parse_to_address,
    parser::{CliArgs, CliServerArgs, Parser},
};
use log::{info, error};
use server::services::server_runner::run_server;

fn main() -> std::io::Result<()> {
    shared::logger::init_logger();

    let cli_args: CliArgs = CliArgs::Server(CliServerArgs::parse());
    let address = parse_to_address(cli_args);
    match run_server(address.as_str()) {
        Ok(_) => info!("Server stopped successfully!"),
        Err(_e) => error!("Could not start the server"),
    }

    Ok(())
}
