use crate::util::Result;
use log::LevelFilter;
use log4rs::{
    append::rolling_file::policy::compound::{
        roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::env;

const TRIGGER_FILE_SIZE: u64 = 2 * 1024 * 1024;

const LOG_FILE_COUNT: u32 = 10;

pub fn init_logger() -> Result<()> {
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let archive_path = env::current_dir()
        .unwrap()
        .join("debug.{}.log")
        .to_string_lossy()
        .into_owned();
    let roller = FixedWindowRoller::builder()
        .base(0)
        .build(&archive_path, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let log_file_path = env::current_dir()
        .unwrap()
        .join("debug.log")
        .to_string_lossy()
        .into_owned();
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m} {d}\n")))
        .build(&log_file_path, Box::new(policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use log::{debug, error};

    use super::init_logger;

    #[test]
    fn test_debug_log() {
        init_logger();
        debug!("test");

        error!("error");
    }
}
