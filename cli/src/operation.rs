#[cfg(test)]
mod operation_tests {
    use crate::parser::{CliClientArgs, CliServerArgs};

    pub fn initialize() -> CliServerArgs {
        CliServerArgs {
            hostname: "127.0.0.1".to_string(),
            port: 8787,
            verbose: false,
            debug: false,
        }
    }

    #[test]
    fn test_parse_client_to_address() {
        let args = initialize();
        let client_args: CliClientArgs = CliClientArgs {
            hostname: args.hostname.clone(),
            port: args.port.clone(),
            worker_name: "worker".to_string(),
            verbose: args.verbose.clone(),
            debug: args.debug.clone(),
            open: false,
        };

        let address = format!("{}:{}", client_args.hostname, client_args.port);
        assert_eq!(address, "127.0.0.1:8787");
    }

    #[test]
    fn test_parse_server_to_address() {
        let args = initialize();
        let server_args: CliServerArgs = CliServerArgs {
            hostname: args.hostname.clone(),
            port: args.port.clone(),
            verbose: args.verbose.clone(),
            debug: args.debug.clone(),
        };

        let address = format!("{}:{}", server_args.hostname, server_args.port);
        assert_eq!(address, "127.0.0.1:8787");
    }
}
