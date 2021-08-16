use headcrab_dap::*;
use log::{error, info};

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
    use std::io;

    let stdin = io::stdin();
    let mut input = stdin.lock();

    init_logger();

    loop {
        match Message::read_from(&mut input) {
            Ok(Message::Request(request)) => match request {
                Request::Initialize(_) => info!("init"),
                Request::Disconnect(_) => info!("disconnect"),
                Request::Generic(request) => {
                    info!("command={}", request.command());
                    if let Some(args) = request.arguments() {
                        info!("args={:#}", args)
                    }
                }
            },
            Ok(Message::Generic(message)) => info!("type={}", message.message_type()),
            Err(error) => {
                error!("error: {}", error);
                break;
            }
        }
    }
}
