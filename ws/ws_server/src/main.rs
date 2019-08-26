extern crate ws;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};

#[derive(Debug)]
struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        println!("received {}", msg);
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("{:?}", self),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
  listen("127.0.0.1:3012", |out| Server { out: out } ).unwrap()
}
