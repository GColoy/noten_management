mod server;

use axum::{
    routing::get,
    Router,
};

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

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let server = Server::new(
        ADDRESS,
        MODE,
    );
    server.serve(app).await;
}