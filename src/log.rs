use std::{fs::File, io::Write, path::Path};

use chrono::{DateTime, Local};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::error::KappaError;

// 配置日志
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // 配置日志系统
    let log_path = Path::new("logs/latest.log");
    // 将上次的日志保存至新文件
    if log_path.exists() {
        // 获取老日志访问时间，用来保存老日志
        let time: DateTime<Local> = log_path
            .metadata()
            .expect("Could not read metatata of last log file!")
            .modified()?
            .into();
        let time = time.format("%Y%m%d-%H%M%S");

        // 重复文件名处理
        let mut i = 0;
        let mut last_file = File::create(format!("logs/{}.log", time));
        while last_file.is_err() && i < 16 {
            i += 1;
            last_file = File::create(format!("logs/{}-{}.log", time, i));
        }

        if i >= 15 {
            return Err(Box::new(KappaError::new("Could not save last log file!")));
        }

        let mut last_file = last_file?;
        last_file
            .write(&std::fs::read(log_path)?)
            .expect("Could not copy to log file! ");
        std::fs::remove_file(Path::new("logs/latest.log")).expect("Could not remove old log file!");
    }

    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%dT%H:%M:%S)}] [{l}] [{t}]: {m}{n}",
        )))
        .build("logs/latest.log")?;

    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%dT%H:%M:%S)}] [{l}] [{t}]: {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appenders(vec![
            Appender::builder().build("console", Box::new(console_appender)),
            Appender::builder().build("logfile", Box::new(file_appender)),
        ])
        .build(
            Root::builder()
                .appenders(vec!["logfile", "console"])
                .build(log::LevelFilter::Warn),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}
