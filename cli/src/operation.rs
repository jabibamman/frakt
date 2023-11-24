use crate::parser::CliArgs;

pub fn parse_to_address(cli_args: CliArgs) -> String {
    match cli_args {
        CliArgs::Client(args) => format!("{}:{}", args.hostname, args.port),
        CliArgs::Server(args) => format!("{}:{}", args.hostname, args.port),
    }
}