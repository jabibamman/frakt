pub use clap::Parser;

/// Represents command line arguments for a client in a CLI application.
///
/// This struct is used to parse and store command line arguments specific to the client part of an application.
/// It leverages the `clap` crate for argument parsing.
///
/// ## Fields
/// - `hostname`: A string specifying the hostname. Defaults to "localhost".
/// - `port`: A 16-bit unsigned integer specifying the port number. Defaults to 8787.
/// - `worker_name`: A string specifying the name of the worker. Defaults to "worker".
///
/// ## Example
/// Command line usage might look like this:
/// ```sh
/// worker -H 192.168.1.0 -P 8787 -N my_group_name
/// ```
#[derive(Parser, Debug, Clone)]
pub struct CliClientArgs {
    /// Optional: The hostname of the client.
    /// Default: "localhost"
    #[clap(long = "hostname", default_value = "localhost")]
    pub hostname: String,

    /// Optional: The port number to connect on.
    /// Default: 8787
    #[clap(short = 'P', long = "port", default_value = "8787")]
    pub port: u16,

    /// Optional: The name of the worker.
    /// Default: "worker"
    #[clap(short = 'N', long = "name", default_value = "worker")]
    pub worker_name: String,

    /// Optional: Add a flag to enable/disable logging.
    /// Default: false
    #[clap(short = 'v', long = "verbose", default_value = "false")]
    pub verbose: bool,

    /// Optional: Add a flag to enable/disable debug mode.
    /// Default: false
    #[clap(short = 'd', long = "debug", default_value = "false")]
    pub debug: bool,

    /// Optional: Add a flag to enable/disable opening the browser.
    /// Default: false
    #[clap(short = 'o', long = "open", default_value = "false")]
    pub open: bool,

    /// Optional: Add a flag to save the image to a file.
    /// Default: false
    #[clap(short = 's', long = "save", default_value = "false")]
    pub save: bool,
}

/// Represents command line arguments for a server in a CLI application.
///
/// Similar to `CliClientArgs`, this struct is for parsing server-specific command line arguments.
/// It uses the `clap` crate for parsing.
///
/// ## Fields
/// - `hostname`: A string specifying the hostname. Defaults to "localhost".
/// - `port`: A 16-bit unsigned integer specifying the port number. Defaults to 8787.
/// - `verbose`: A boolean flag to enable/disable logging. Defaults to false.
/// - `debug`: A boolean flag to enable/disable debug mode. Defaults to false.
/// - `width`: A 16-bit unsigned integer specifying the width of the window. Defaults to 1200.
/// - `height`: A 16-bit unsigned integer specifying the height of the window. Defaults to 1200.
///
/// ## Example
/// Command line usage for the server might be:
/// ```sh
/// server -H 192.168.1.0 -P 8787
/// ```
#[derive(Parser, Debug, Clone)]
pub struct CliServerArgs {
    /// The hostname of the server.
    /// Default: "localhost"
    #[clap( long = "hostname", default_value = "localhost")]
    pub hostname: String,

    /// The port number the server listens on.
    /// Default: 8787
    #[clap(short = 'P', long = "port", default_value = "8787")]
    pub port: u16,

    /// Optional: Add a flag to enable/disable logging.
    /// Default: false
    #[clap(short = 'v', long = "verbose", default_value = "false")]
    pub verbose: bool,

    /// Optional: Add a flag to enable/disable debug mode.
    /// Default: false
    #[clap(short = 'd', long = "debug", default_value = "false")]
    pub debug: bool,

    /// Optional: Add a flag to edit the width and height of the window.
    /// Default: 1200
    #[clap(long = "width", default_value = "1200")]
    pub width: u16,

    /// Optional: Add a flag to edit the width and height of the window.
    /// Default: 1200
    #[clap(long = "height", default_value = "1200")]
    pub height: u16,
}

/// An enumeration representing the possible types of command line arguments.
///
/// This enum helps in differentiating between client and server command line arguments.
/// It is a common practice in CLI applications to have different sets of arguments for different modes (client/server).
///
/// ## Variants
/// - `Client(CliClientArgs)`: Command line arguments specific to the client.
/// - `Server(CliServerArgs)`: Command line arguments specific to the server.
#[derive(Clone)]
pub enum CliArgs {
    Client(CliClientArgs),
    Server(CliServerArgs),
}
