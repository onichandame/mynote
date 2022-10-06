#[tokio::main]
async fn main() {
    notebook::start_server().await.unwrap();
}
