pub use clap::Parser;


/// # Command line arguments for the CLI
/// 
/// > This struct is used to parse the command line arguments
/// 
/// ## Example
/// 
/// ```sh
/// worker -H 192.168.1.0 -P 3000 -N my_group_name
/// ```
#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(
        short = 'H',
        long = "hostname",
        default_value = "localhost"
    )]
    pub hostname: String,

    #[clap(
        short = 'P',
        long = "port",
        default_value = "8787"
    )]
    pub port: u16,

    #[clap(
        short = 'N',
        long = "name",
        default_value = "worker"
    )]
    pub worker_name: String,
}