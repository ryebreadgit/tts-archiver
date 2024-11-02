use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    encode::pattern::PatternEncoder,
    config::{Appender, Config, Logger, Root},
};

pub fn init_logging(
    root_level: LevelFilter,
    app_level: LevelFilter,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%dT%H:%M:%S%.3f%:z)(utc)}] [{({l}):5.5}] (({f}:{L})) {m}{n}"
        )))
        .build();

    // Build config with explicit module loggers
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        // Configure each module explicitly
        .logger(Logger::builder()
            .appender("stdout")
            .additive(false)
            .build("extract", app_level))
        .logger(Logger::builder()
            .appender("stdout")
            .additive(false)
            .build("process", app_level))
        .logger(Logger::builder()
            .appender("stdout")
            .additive(false)
            .build("store", app_level))
        // Root logger for everything else
        .build(Root::builder()
            .appender("stdout")
            .build(root_level))?;

    log4rs::init_config(config)?;
    Ok(())
}