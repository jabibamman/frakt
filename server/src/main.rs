use std::env;

use cli::parser::{CliServerArgs, Parser};
use log::{error, info};
use server::services::server_runner::run_server;
use shared::types::error::FractalError;

#[tokio::main]
async fn main() -> Result<(), FractalError> {
    let cli_args: CliServerArgs = CliServerArgs::parse();
    shared::logger::init_logger(cli_args.verbose, cli_args.debug, cli_args.trace)?;
    env::set_var("RESOLUTION_WIDTH", cli_args.width.to_string());
    env::set_var("RESOLUTION_HEIGHT", cli_args.height.to_string());

    let address = format!("{}:{}", cli_args.hostname, cli_args.port);
    match run_server(address.as_str()) {
        Ok(_) => info!("Server stopped successfully!"),
        Err(_e) => error!("Could not start the server"),
    }

    Ok(())
}
