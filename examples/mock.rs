use std::io;

use log::{error, info};

use headcrab_dap::header::Header;

fn init_logger() {
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    init_logger();

    loop {
        match Header::read_from(&mut input) {
            Ok(header) => {
                info!("content-length={}", header.len);
                info!("field={:?}", header.fields);
            }
            Err(error) => {
                error!("error: {}", error);
                break;
            }
        }
    }
}
