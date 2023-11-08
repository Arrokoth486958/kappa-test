use std::io::Write;

use env_logger::{fmt::Timestamp, Builder};
use log4rs::{append::{file::FileAppender, console::ConsoleAppender}, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};

pub fn init() {
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%dT%H:%M:%S)}] [{l}] [{t}]: {m}{n}")))
        .build("logs/latest.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(Root::builder()
            .appender("logfile")
            .build(log::LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    // Builder::new()
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "[{}][{}]: {}",
    //             chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //             record.level(),
    //             record.args(),
    //         )
    //     })
    //     .filter(None, log::LevelFilter::Info)
    //     .format_timestamp(None)
    //     .init();
}
