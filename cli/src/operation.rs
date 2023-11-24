use crate::parser::CliArgs;

/// Parses `CliArgs` to create a formatted address string.
///
/// This function takes an instance of `CliArgs` and returns a string representing an address.
/// It supports both client and server arguments. The address format is "hostname:port".
///
/// ## Parameters
/// - `cli_args`: An instance of `CliArgs`, which can be either `CliClientArgs` or `CliServerArgs`.
///
/// ## Returns
/// A `String` that represents the address in the format "hostname:port".
///
/// ## Examples
/// Assuming `CliArgs::Client` variant is used with hostname "192.168.1.0" and port 8787:
/// ```rust
/// use cli::{parser::{CliClientArgs, Parser, CliArgs}, operation::parse_to_address};
///
/// let client_args = CliArgs::Client(CliClientArgs {
///     hostname: "192.168.1.0".to_string(),
///     port: 8787,
///     worker_name: "worker".to_string(),
/// });
///
/// let address = parse_to_address(client_args);
/// assert_eq!(address, "192.168.1.0:8787");
/// ```
/// Similarly, you can create an instance of `CliArgs::Server` and pass it to this function.
pub fn parse_to_address(cli_args: CliArgs) -> String {
    match cli_args {
        CliArgs::Client(args) => format!("{}:{}", args.hostname, args.port),
        CliArgs::Server(args) => format!("{}:{}", args.hostname, args.port),
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;
    use crate::parser::{CliArgs, CliClientArgs, CliServerArgs};

    pub fn initialize() -> CliServerArgs {
        CliServerArgs {
            hostname: "127.0.0.1".to_string(),
            port: 8787,
        }
    }

    #[test]
    fn test_parse_client_to_address() {
        let args = initialize();
        let client_args = CliArgs::Client(CliClientArgs {
            hostname: args.hostname,
            port: args.port,
            worker_name: "worker".to_string(),
        });

        let address = parse_to_address(client_args);
        assert_eq!(address, "127.0.0.1:8787");
    }

    #[test]
    fn test_parse_server_to_address() {
        let args = initialize();
        let server_args = CliArgs::Server(args);

        let address = parse_to_address(server_args);
        assert_eq!(address, "127.0.0.1:8787");
    }
}
