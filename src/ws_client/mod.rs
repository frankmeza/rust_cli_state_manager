extern crate ws;

use ws::{connect, CloseCode, Handler, Handshake, Message, Sender, Result};

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("ws is hella lit")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("I got it: {}", msg);
        self.out.send(msg).unwrap();

        self.out.close(CloseCode::Normal)
    }
}

pub fn connect_ws(mtype: &str, mvalue: &str) {
    // let out_msg = format!("{}, {}", mtype, mvalue);
    println!("{} : {}", mtype, mvalue);
    connect("ws://127.0.0.1:3012", |out| {
        println!("OUT : {:?}", out);

        Client { out }
    }).unwrap()
}
