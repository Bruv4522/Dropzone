mod res;

use axum::{Router, routing::get, Json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {
            Json(res::Res {
                message: "Dropzone: use /new to create a drop".to_string()
            })
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
