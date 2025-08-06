use axum::Router;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Http,
    Https,
    HttpsRedirect(u16), // Port for redirecting HTTP to HTTPS
}

#[derive(Clone, Copy)]
pub struct Address {
    pub address: [u8; 4],
    pub port: u16,
}

impl std::string::ToString for Address {
    fn to_string(&self) -> String {
        format!("{}:{}", self.address.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."), self.port)
    }
}

#[derive(Clone, Copy)]
pub struct Server {
    pub address: Address,
    pub mode: Mode,
}

impl Server {
    pub fn new(address: Address, mode: Mode) -> Self {
        Self {
            address,
            mode,
        }
    }

    pub async fn serve(self, app: Router) {
        println!("Starting server at {} in {:?} mode", self.address.to_string(), self.mode);
        match self.mode {
            Mode::Http => self.serve_unsecure(app).await,
            Mode::Https => self.serve_secure(app).await,
            Mode::HttpsRedirect(http_port) => {
                self.redirect_http_to_https(http_port).await;
                self.serve_secure(app).await;
            }
        }
    }

    async fn serve_unsecure(self, app: Router) {
        let listener = tokio::net::TcpListener::bind(self.address.to_string()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    async fn serve_secure(self, _app: Router) {
        unimplemented!()
    }
    
    async fn redirect_http_to_https(self, http_port: u16) {
        unimplemented!()
    }
}