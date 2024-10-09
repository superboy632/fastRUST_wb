mod router;

use serde::{Serialize, Deserialize};
use std::net::{IpAddr, Ipv4Addr};
use router::create_main_router;

// подключаем библиотеку axum. В данном сервисе используем 3 http запроса, router, чтобы связать функции с ними, и Json для передачи информации
use axum::{
    routing::{post, get, delete},
    Router,
    Json,
    extract::State,
};


// модель данных
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: u32,
    user_id: u32,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Like {
    id: u32,
    post_id: u32,
    user_id: String,
}



struct App {
    address: IpAddr,
    port: u16,
}

impl App {
    async fn new(port: u16) -> App {
        let address: IpAddr = IpAddr::from([127, 0, 0, 1]);

        Self { address, port }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let router = create_main_router();
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;

        axum::serve(listener, router).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .expect("PORT environment variable not set")
        .parse::<u16>()
        .expect("PORT environment variable is not a number");

    let app = App::new(port).await;

    app.run().await.expect("Error running server");
}

