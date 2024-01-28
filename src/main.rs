mod database_connection;
use database_connection::*;

mod routes;
use routes::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database()
        .await
        .expect("Could not connect to database");
    println!("Connected to the database without any error");

    let routes = routes(pool);

    let tcp_listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not get the port 3000");
    println!("Server running on http://localhost:3000");
    let _ = axum::serve(tcp_listener, routes.into_make_service()).await;

    println!("Connected to database");
}
