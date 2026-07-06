mod app;
mod handlers;

#[tokio::main]
async fn main() {
    let app = app::build_app();
    let listener = tokio::net::TcpListener::bind(get_addr()).await.unwrap();

    println!("Working at localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn get_addr() -> String {
    let ip = "0.0.0.0";
    let port = "3000";
    format!("{}:{}", ip, port)
}
