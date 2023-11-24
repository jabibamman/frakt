pub use clap::Parser;

/// # Command line arguments for the CLI
///
/// > This struct is used to parse the command line arguments
///
/// ## Example
///
/// ```sh
/// worker -H 192.168.1.0 -P 8787 -N my_group_name
/// ```
#[derive(Parser, Debug)]
pub struct CliClientArgs {
    #[clap(short = 'H', long = "hostname", default_value = "localhost")]
    pub hostname: String,

    #[clap(short = 'P', long = "port", default_value = "8787")]
    pub port: u16,

    #[clap(short = 'N', long = "name", default_value = "worker")]
    pub worker_name: String,
}

/// # Command line arguments for the server
/// 
/// > This struct is used to parse the command line arguments
/// 
/// ## Example
/// 
/// ```sh
/// server -H 192.168.1.0 -P 8787
#[derive(Parser, Debug)]
pub struct CliServerArgs {
    #[clap(short = 'H', long = "hostname", default_value = "localhost")]
    pub hostname: String,

    #[clap(short = 'P', long = "port", default_value = "8787")]
    pub port: u16,
}


pub enum CliArgs {
    Client(CliClientArgs),
    Server(CliServerArgs),
} 