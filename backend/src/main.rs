mod config;
mod handlers;
mod models;
mod repository;
mod routes;

use std::net::SocketAddr;

use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let port = 8081;
    let dir = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(dir)
        .await
        .unwrap_or_else(|e| {
            panic!("fallo la conetsion de la dir {}:{}", dir, e);
        });

    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app = routes::root::router().with_state(pool);

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        //esto levanta el servidor
        panic!("fallo la conetsion con el servidor {}", e);
    });
}
