#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = restfullapi::routes::routes().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("[+] listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
