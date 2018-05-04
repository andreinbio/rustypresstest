extern crate rustypress;

fn main() {
    let socket = "3000";
    let ip = "127.0.0.1";
    let address = format!("{}:{}", ip, socket);

    println!("server running at localhost:{0}", socket);
    rustypress::RustyServer::new().http(&address);
}