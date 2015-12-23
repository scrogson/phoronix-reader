use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

#[allow(dead_code)]
fn online() -> String {
    let client = Client::new();
    let mut response = client.get("https://www.phoronix.com/")
                             .header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    return body;
}

#[allow(dead_code)]
pub fn offline() -> &'static str {
    include_str!("phoronix.html")
}
