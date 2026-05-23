use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::broadcaster::TranslationEvent;
use futures_util::{sink::SinkExt, stream::StreamExt};

pub struct AppState {
    pub tx: broadcast::Sender<TranslationEvent>,
}

pub async fn start_server(listener: tokio::net::TcpListener, tx: broadcast::Sender<TranslationEvent>, shutdown_rx: tokio::sync::oneshot::Receiver<()>) {
    let state = Arc::new(AppState { tx });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Resolve web-ui path: try CWD/web-ui first, then relative to executable
    let web_ui_path = if std::path::Path::new("web-ui").exists() {
        "web-ui".to_string()
    } else if let Ok(exe) = std::env::current_exe() {
        // Binary is in src-tauri/target/debug/, web-ui is at project root
        let project_root = exe.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent());
        if let Some(root) = project_root {
            let p = root.join("web-ui");
            if p.exists() { p.to_string_lossy().to_string() } else { "web-ui".to_string() }
        } else {
            "web-ui".to_string()
        }
    } else {
        "web-ui".to_string()
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new(&web_ui_path))
        .layer(cors)
        .with_state(state);

    log::debug!("Web server listening on http://{}", listener.local_addr().unwrap());
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await 
    {
        log::error!("Web server fatal error: {}", e);
    }
    log::info!("Web server shut down gracefully");
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    log::debug!("WebSocket client connected");
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    log::debug!("WebSocket send failed, client likely disconnected");
                    break;
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_msg)) = receiver.next().await {
            // Ignoring incoming client messages
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    log::debug!("WebSocket client disconnected");
}
