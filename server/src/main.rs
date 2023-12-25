use cli::parser::{CliServerArgs, Parser};
use log::{info, error};
use server::services::server_runner::run_server;

fn main() -> std::io::Result<()> {
    shared::logger::init_logger();

    let cli_args: CliServerArgs = CliServerArgs::parse();
    let address = format!("{}:{}", cli_args.hostname, cli_args.port);
    match run_server(address.as_str()) {
        Ok(_) => info!("Server stopped successfully!"),
        Err(_e) => error!("Could not start the server"),
    }

    Ok(())
}
