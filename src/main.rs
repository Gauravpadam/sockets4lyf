use axum::routing::get;
use serde_json::Value;
use socketioxide::{extract::{Data, SocketRef}, SocketIo};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);

    socket.on("message", |socket: SocketRef, Data::<Value>(data)| async move {
        info!("Received from client: {:?}", data);
        
        // Optionally send a message back to the client for confirmation
        socket.emit("server_message", &data).unwrap();
    });
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello World!"}))
        .layer(
            ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer),
        );
        
    info!("Starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
