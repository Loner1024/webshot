use axum::{extract::Query, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Resp {
    code: u8,
    message: String,
}

#[derive(Deserialize)]
struct Req {
    name: String,
}

async fn hello(params: Query<Req>) -> Json<Resp> {
    let name = params.0.name;
    let r = Resp {
        code: 0,
        message: format!("Hello, {}", name),
    };
    Json(r)
}
