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
        match Message::try_from_input(&mut input) {
            Ok(message) => {
                info!("seq={}", message.seq());

                if let Some(request) = message.message_kind() {
                    match request.request_kind() {
                        Some(Request::Initialize(init)) => {
                            info!("init={:#?}", init);
                        }
                        _ => {
                            info!("command={}", request.command());
                            if let Some(args) = request.arguments() {
                                info!("args={:#}", args)
                            }
                        }
                    }
                } else {
                    info!("type={}", message.message_type());
                }
            }
            Err(error) => {
                error!("error: {}", error);
                break;
            }
        }
    }
}
