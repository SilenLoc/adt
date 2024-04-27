mod db;

use crate::db::{InMemoryDatabase, ShoppingItem};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, post};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

type Database = Arc<RwLock<InMemoryDatabase>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let db = Database::default();
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/healthz", get(healthz))
        .route("/workshop", post(workshop_echo))
        .route("/items", get(get_items).post(add_item))
        .route("/items/:uuid", delete(delete_item))
        .with_state(db);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3099")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// basic handler that responds with a static string
async fn healthz() -> &'static str {
    "Hello, World!"
}

async fn workshop_echo(Json(workshop): Json<Workshop>) -> impl IntoResponse {
    Json(workshop)
}

#[derive(Serialize, Deserialize)]
struct Workshop {
    attendees_count: i32,
    people_like_it: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingListItem {
    pub title: String,
    pub posted_by: String,
    pub uuid: String,
}

pub async fn get_items(State(state): State<Database>) -> impl IntoResponse {
    let items: Vec<ShoppingListItem> = state
        .read()
        .unwrap()
        .as_vec()
        .iter()
        .cloned()
        .map(|(uuid, item)| ShoppingListItem {
            title: item.title,
            posted_by: item.creator,
            uuid,
        })
        .collect();

    Json(items)
}

#[derive(Serialize, Deserialize)]
pub struct PostShopItem {
    pub title: String,
    pub posted_by: String,
}

pub async fn add_item(
    State(state): State<Database>,
    Json(post_request): Json<PostShopItem>,
) -> impl IntoResponse {
    let item = ShoppingItem {
        title: post_request.title.clone(),
        creator: post_request.posted_by.clone(),
    };
    let uuid = Uuid::new_v4().to_string();

    let Ok(mut db) = state.write() else {
        return (StatusCode::SERVICE_UNAVAILABLE).into_response();
    };

    db.insert_item(&uuid, item);

    (
        StatusCode::OK,
        Json(ShoppingListItem {
            title: post_request.title,
            posted_by: post_request.posted_by,
            uuid,
        }),
    )
        .into_response()
}

use axum::extract::Path;

pub async fn delete_item(
    State(state): State<Database>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let Ok(mut db) = state.write() else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };

    db.delete_item(&uuid.to_string());

    StatusCode::OK
}
