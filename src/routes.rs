use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use prkorm::*;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Table, Debug, sqlx::FromRow, Serialize, Deserialize)]
#[table_name("users")]
struct User {
    id: Option<i32>,
    username: String,
    email: String,
}

pub fn routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/users/all", get(get_user))
        .route("/user/create", post(create_new_user))
        .with_state(pool)
}

// function that gets all the users from database. Add this method as a route in routes method.
async fn get_user(State(db): State<MySqlPool>) -> impl IntoResponse {
    let query = User::select().build();
    let users: Result<Vec<User>, _> = sqlx::query_as(&query).fetch_all(&db).await;
    match users {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// Function to create a new user
async fn create_new_user(
    State(db): State<MySqlPool>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    // Let us define our insert query
    let query = User::insert()
        .insert_to_username(payload.username)
        .insert_to_email(payload.email)
        .build();

    let response = sqlx::query(&query).execute(&db).await;

    match response {
        Ok(res) => (StatusCode::CREATED, Json(res.last_insert_id())).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, _e.to_string()).into_response(),
    }
}
