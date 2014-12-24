extern crate "dbus-rs" as dbus;

use dbus::{Connection, BusType, NameFlag, ConnectionItem, Message};

static DBUS_ERROR_FAILED: &'static str = "org.freedesktop.DBus.Error.Failed";

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("com.example.test", NameFlag::ReplaceExisting as u32).unwrap();
    c.register_object_path("/hello").unwrap();
    for n in c.iter(1000) {
        if let ConnectionItem::MethodCall(method_call) = n {
            if let (_, Some(path), Some(_), Some(member)) = method_call.headers() {
                match (&*path, &*member) {
                    ("/hello", "SayHello") => {
                        let mut response = Message::new_method_return(&method_call).unwrap();
                        response.append_item("Hello, world!");
                        c.send(response).ok().expect("Failed to send reply");
                    },
                    _ => {
                        c.send(Message::new_error(&method_call, DBUS_ERROR_FAILED, "Method not found").unwrap())
                            .ok().expect("Failed to send reply");
                    }
                }
            }
        }
    }
}
