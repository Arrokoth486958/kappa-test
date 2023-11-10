use std::{fs::File, io::Write, path::Path};

use chrono::{DateTime, Local};
use env_logger::{fmt::Timestamp, Builder};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn init() {
    let log_path = Path::new("logs/latest.log");
    if log_path.exists() {
        let time: DateTime<Local> = log_path
            .metadata()
            .expect("Could not read metatata of last log file!")
            .modified()
            .unwrap()
            .into();
        let time = time.format("%Y-%m-%dT%H:%M:%S");

        let mut i = 0;
        let mut last_file = File::create(format!("logs/{}.log", time));
        while last_file.is_err() {
            i += 1;
            last_file = File::create(format!("logs/{}-{}.log", time, i));
        }
        
        let mut last_file = last_file.unwrap();
        last_file
            .write(&std::fs::read(log_path).unwrap())
            .expect("Could not copy to log file! ");
        std::fs::remove_file(Path::new("logs/latest.log")).expect("Could not remove old log file!");
    }

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%dT%H:%M:%S)}] [{l}] [{t}]: {m}{n}",
        )))
        .build("logs/latest.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(log::LevelFilter::Info),
        )
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
