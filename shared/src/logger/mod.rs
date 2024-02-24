use crate::types::error::FractalError;
use env_logger::{Builder, Env};
use log::LevelFilter;

pub fn init_logger(verbose: u8, debug: u8, trace: u8) -> Result<(), FractalError> {
    let log_level = match (verbose, debug, trace) {
        (0, 0, 0) => LevelFilter::Error, // default
        (_, 1, 0) => LevelFilter::Debug, // -d
        (1, _, _) => LevelFilter::Info,  // -v
        (_, _, 1) => LevelFilter::Trace, // -t
        _ => LevelFilter::Warn,          // multiple flags
    };

    let mut builder = Builder::from_env(Env::default());

    builder.format_timestamp(Some(env_logger::TimestampPrecision::Millis));
    builder.filter_level(log_level);
    builder.init();

    Ok(())
}
