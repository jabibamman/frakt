use cli::{
    operation::parse_to_address,
    parser::{CliArgs, CliServerArgs, Parser},
};
use server::services::server_runner::run_server;

fn main() -> std::io::Result<()> {
    let cli_args: CliArgs = CliArgs::Server(CliServerArgs::parse());
    let address = parse_to_address(cli_args);
    match run_server(address.as_str()) {
        Ok(_) => println!("[SERVER] Server stopped."),
        Err(e) => println!("[SERVER] Server stopped with error: {}", e),
    }

    Ok(())
}
