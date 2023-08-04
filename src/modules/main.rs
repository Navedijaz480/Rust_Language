use server::Server;
mod server;
fn main() {
    // let server = server::Server::new("127.0.0.1".to_string());
    let server = Server::new("127.0.0.1".to_string());
    server.run();
}

