use log::{error, info};
use networking::{connect_to_server, process_fragment_task, receive_fragment_task, send_request};
use shared::{types::error::FractalError, utils::fragment_request_impl::FragmentRequestOperation};

mod fractal_generation;
mod image;
mod networking;

use cli::parser::{CliClientArgs, Parser};
use shared::types::messages::FragmentRequest;

fn main() -> Result<(), FractalError> {
    let cli_args: CliClientArgs = CliClientArgs::parse();
    shared::logger::init_logger(cli_args.verbose, cli_args.debug)?;

    let serialized_request =
        FragmentRequest::new(cli_args.worker_name.clone(), 1000).serialize()?;

    let mut stream = connect_to_server(&cli_args)?;
    send_request(&mut stream, &serialized_request)?;

    loop {
        match receive_fragment_task(&mut stream) {
            Ok(Some((fragment_task, data))) => {
                stream = process_fragment_task(fragment_task, data, &cli_args)?;
            }
            Ok(None) => {
                info!("No more tasks to process");
                break; 
            }
            Err(e) => {
                error!("Error receiving fragment task: {:?}", e);
                break; 
            }
        }
    }
    
    Ok(())
}
