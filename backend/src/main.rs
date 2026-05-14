//pense el main para levantar el servidor nomas, las otras responsabilidades las delego a otros archivos
mod app;
mod repository;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let puerto = 8081; //elegido para no tener problemas
    let dir = SocketAddr::from(([0, 0, 0, 0], puerto));
    let listener = tokio::net::TcpListener::bind(dir)
        .await
        .unwrap_or_else(|e| {
            panic!("fallo la conetsion de la dir {}:{}", dir, e);
        });
    let router = routes::root::router().merge(routes::root::router());

    axum::serve(listener, router).await.unwrap_or_else(|e| {
        //esto levanta el servidor
        panic!("fallo la conetsion con el servidor {}", e);
    });
}
