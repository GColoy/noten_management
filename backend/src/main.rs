mod server;
mod app;

use std::path::Path;

use server::*;

const ADDRESS: Address = Address {
    address: [127, 0, 0, 1],
    port: 3000,
};
const MODE: Mode = if cfg!(debug_assertions) {
                        Mode::Http
                    } else {
                        Mode::Https
                    };
const STATIC_DIR: &str = "../frontend/dist";

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } 

    let path = Path::new(STATIC_DIR);
    let app = app::create_app(path);

    let server = Server::new(
        ADDRESS,
        MODE,
    );
    server.serve(app).await;
}