use log::LevelFilter;

use log4rs::{
    append::rolling_file::policy::compound::{
        roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

use crate::data::get_switch_hosts_rs_dir;

use crate::util::Result;

const TRIGGER_FILE_SIZE: u64 = 200 * 1024;

const LOG_FILE_COUNT: u32 = 10;

const ARCHIVE_PATTERN: &str = "/tmp/archive/switch-hosts-rs.{}.log";

pub fn init_logger() -> Result<()> {
    let level = log::LevelFilter::Info;
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .base(0)
        .build(ARCHIVE_PATTERN, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let log_file_path = get_switch_hosts_rs_dir()
        .map(|buf| buf.join("error.log"))
        .unwrap();
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m} {d}\n")))
        .build(&log_file_path, Box::new(policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    Ok(())
}
