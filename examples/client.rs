extern crate dbus;

use dbus::{Connection, BusType, Message, MessageItem};
use std::io::Read;
use std::fs::File;

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call("com.example.test", "/hello", "com.example.test", "Hello").unwrap();
    let mut r = c.send_with_reply_and_block(m, 2000).unwrap();
    let reply = r.get_items();
    println!("{:?}", reply);
    match reply[1] {
        MessageItem::FileDescriptor(fd) => {
            let path = format!("/proc/self/fd/{}", fd);
            let mut opened = File::open(&path).unwrap();
            let mut buf = String::new();
            opened.read_to_string(&mut buf);

            println!("{}", buf);
        },
        _ => println!("nothing cool")
    }
}

