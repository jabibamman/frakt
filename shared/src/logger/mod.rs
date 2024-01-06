use crate::types::error::FractalError;
use log::LevelFilter;
use env_logger::{Builder, Env};

pub fn init_logger(verbose: bool, debug: bool) -> Result<(), FractalError> {
    let mut builder = Builder::from_env(Env::default());

    builder.format_timestamp(Some(env_logger::TimestampPrecision::Millis));

    let log_level = if debug {
        LevelFilter::Debug
    } else if verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    builder.filter_level(log_level);
    builder.init();

    Ok(())
}