use chrono::Local;
use cron_job::{CronJob, Job};
use log::LevelFilter;

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config, Handle,
};
use tokio::spawn;

const PATTERN_ENCODER: &str = "[EAST] {d(%Y-%m-%d %H:%M:%S)} - {l} -{t} - {m}{n}";
const DATETIME_FORMAT: &str = "%Y%m%d";
// const DATETIME_FORMAT: &str = "%Y%m%d%H%M%S";
const CRON: &str = "0 0 0 * * ?";
// const CRON: &str = "0 0/1 * * * ?";

fn get_log_config() -> Config {
    let now = Local::now().format(DATETIME_FORMAT);
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN_ENCODER)))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN_ENCODER)))
        .build(format!("log/{}.log", now.to_string()))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(
            Logger::builder()
                .appender("file")
                .additive(true)
                .build("EAST", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )
        .unwrap();
    config
}

fn init_config() -> Handle {
    let config = get_log_config();
    log4rs::init_config(config).unwrap()
}

fn job(handle: Handle) {
    let mut cron = CronJob::new();
    let log_job = LogJob { handle: handle };
    cron.new_job(CRON, log_job);
    cron.start();
}

pub fn init() {
    let handle = init_config();
    spawn(async move {
        job(handle);
    });
}

struct LogJob {
    handle: Handle,
}

impl Job for LogJob {
    fn run(&mut self) {
        self.handle.set_config(get_log_config());
    }
}
