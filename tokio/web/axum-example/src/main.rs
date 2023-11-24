use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {
    // init route
    let app = Router::new()
        // "/" path
        .route("/", get(root))
        // /users path
        .route("/users", post(create_user));

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hello world"
}

// curl -XPOST -H 'Content-Type: application/json' -d '{"username":"halo"}' http://127.0.0.1:8080/users
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    Json(user)
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, Serialize)]
struct User {
    id: u64,
    username: String,
}
