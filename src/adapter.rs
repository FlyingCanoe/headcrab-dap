use std::io;
use std::io::BufRead;
use std::sync::mpsc;
use std::thread;

use serde_json as json;

use crate::dap_type::Message;
use crate::header::Header;
use crate::Error;

pub struct Adapter {
    receiver: mpsc::Receiver<Result<Message, Error>>,
}

impl Adapter {
    /// Start a debug adapter in single session mode.
    /// That is a adapter which use stdin and stdout to communicate with the client.
    /// This mean that you should not have printed anything to stdout before you call this function.
    pub fn single_session_mode() -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            let lock = stdin.lock();
            let listener = Listener::new(sender, lock);
            listener.start();
        });
        Adapter { receiver }
    }
}

impl Iterator for Adapter {
    type Item = Result<Message, Error>;

    fn next(&mut self) -> Option<Result<Message, Error>> {
        self.receiver.recv().ok()
    }
}

struct Listener<R: BufRead> {
    input: R,
    sender: mpsc::Sender<Result<Message, Error>>,
}

impl<R: BufRead> Listener<R> {
    fn new(sender: mpsc::Sender<Result<Message, Error>>, input: R) -> Listener<R> {
        Listener { input, sender }
    }

    fn start(mut self) -> ! {
        loop {
            let msg = self.next_msg();
            self.sender.send(msg).unwrap()
        }
    }

    fn next_msg(&mut self) -> Result<Message, Error> {
        let header = Header::read_from(&mut self.input)?;

        let mut buffer = vec![0; header.len];
        self.input.read_exact(buffer.as_mut_slice())?;

        let msg = json::from_slice(&buffer)?;
        Ok(msg)
    }
}
