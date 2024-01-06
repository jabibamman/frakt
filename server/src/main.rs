use cli::parser::{CliServerArgs, Parser};
use log::{info, error};
use server::services::server_runner::run_server;
use shared::types::error::FractalError;

fn main() -> Result<(), FractalError> {
    let cli_args: CliServerArgs = CliServerArgs::parse();
    shared::logger::init_logger(cli_args.verbose, cli_args.debug)?;
    
    let address = format!("{}:{}", cli_args.hostname, cli_args.port);
    match run_server(address.as_str()) {
        Ok(_) => info!("Server stopped successfully!"),
        Err(_e) => error!("Could not start the server"),
    }

    Ok(())
}
