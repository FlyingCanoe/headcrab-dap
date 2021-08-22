use log::{error, info};

use headcrab_dap::adapter::Adapter;

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
    let adapter = Adapter::single_session_mode();

    init_logger();

    for msg in adapter {
        match msg {
            Ok(msg) => match msg {
                headcrab_dap::dap_type::Message::Request(request) => {
                    info!("request");
                    info!("raw={:#?}", request)
                }
                headcrab_dap::dap_type::Message::Event(_) => todo!(),
                headcrab_dap::dap_type::Message::Response(_) => todo!(),
            },
            Err(error) => {
                error!("error: {}", error);
                break;
            }
        }
    }
}
