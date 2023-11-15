pub use clap::Parser;

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