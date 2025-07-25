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

pub fn init_logger() -> Result<()> {
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = if cfg!(target_os = "windows") {
        FixedWindowRoller::builder()
            .base(0)
            .build(
                get_switch_hosts_rs_dir()
                    .clone()
                    .map(|buf| buf.join("archive/switch-hosts-rs.{}.log"))
                    .unwrap()
                    .to_str()
                    .unwrap(),
                LOG_FILE_COUNT,
            )
            .unwrap()
    } else {
        FixedWindowRoller::builder()
            .base(0)
            .build("/tmp/archive/switch-hosts-rs.{}.log", LOG_FILE_COUNT)
            .unwrap()
    };
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
