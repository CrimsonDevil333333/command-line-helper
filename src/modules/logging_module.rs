use log::LevelFilter;
use log4rs;

use crate::print_error_message;

pub fn setup_logging(verbose: bool, out: bool) {
    // Initialize logging using log4rs programmatically
    let log_format = "[{d(%Y-%m-%dT%H:%M:%S%.f%:z)}] : {l} : {m}{n}";

    let config = if out {
        log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder().build(
                    "console",
                    Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                                log_format,
                            )))
                            .build(),
                    ),
                ),
            )
            .appender(
                log4rs::config::Appender::builder().build(
                    "file_verbose",
                    Box::new(
                        log4rs::append::file::FileAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                                log_format,
                            )))
                            .build("logs.log")
                            .unwrap(),
                    ),
                ),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("console")
                    .appender("file_verbose")
                    .build(LevelFilter::Trace),
            )
            .unwrap()
    } else if verbose {
        log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder().build(
                    "console",
                    Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                                log_format,
                            )))
                            .build(),
                    ),
                ),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("console")
                    .build(LevelFilter::Trace),
            )
            .unwrap()
    } else {
        // Adding default error logs
        log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder().build(
                    "console",
                    Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                                log_format,
                            )))
                            .build(),
                    ),
                ),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("console")
                    .build(LevelFilter::Error),
            )
            .unwrap()
    };

    if let Err(e) = log4rs::init_config(config) {
        print_error_message(&format!("Error initializing logging: {}\n", e));
    }
}
