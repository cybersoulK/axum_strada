
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{Router, post};
use axum::Json;

use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;



#[tokio::main]
async fn main() {

    let server_state = ServerState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/register", post(register_account))
        .layer(cors)
        .with_state(server_state)
        .fallback_service(ServeDir::new("website"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



#[derive(Debug)]
struct User {
    unique_id: u64,
    full_name: String,
    email: String,
    password: String
}

#[derive(Clone)]
struct ServerState {
    users: Arc<Mutex<Vec<User>>>
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            users: Default::default()
        }
    }
}


#[derive(Serialize, Deserialize)]
struct RegisterData {
    full_name: String,
    email: String,
    password: String
}

async fn register_account(State(server_state): State<ServerState>, Json(register_data): Json<RegisterData>) -> (StatusCode, Json<u64>) {

    let RegisterData { full_name, email, password } = register_data;

    let mut users = server_state.users.lock().await;
    
    let unique_id = users.len() as u64;

    let user = User {
        unique_id,
        full_name,
        email,
        password
    };

    println!("{user:?}");

    users.push(user);

    (StatusCode::CREATED, unique_id.into())
}