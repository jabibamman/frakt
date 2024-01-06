use networking::{connect_to_server, send_request, receive_fragment_task, process_fragment_task};
use shared::{utils::fragment_request_impl::FragmentRequestOperation, types::error::FractalError};

mod fractal_generation;
mod image;
mod networking;

use cli::parser::{CliClientArgs, Parser};
use shared::types::messages::FragmentRequest;

fn main() -> Result<(), FractalError> {
    let cli_args: CliClientArgs = CliClientArgs::parse();
    shared::logger::init_logger(cli_args.verbose, cli_args.debug)?;

    let serialized_request = FragmentRequest::new(cli_args.worker_name.clone(), 1000).serialize()?;
    
    let mut stream = connect_to_server(&cli_args)?;
    send_request(&mut stream, &serialized_request)?;
    let fragment_task = receive_fragment_task(&mut stream)?;

    if let Some(task) = fragment_task {
        process_fragment_task(task, cli_args.open)?;
    } 

    Ok(())
}