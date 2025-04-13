use axum_multipart_test_examples::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = create_app();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use reqwest::multipart::{Form, Part};
    use reqwest::Client;
    use std::fs;

    #[tokio::test]
    async fn test_with_http_server() -> Result<()> {
        // Prepare
        let app = create_app();
        let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        // Create multipart form
        let file = fs::read("./dummy/test_upload.csv")?;
        let part = Part::bytes(file)
            .file_name("test_upload.csv")
            .mime_str("text/csv")?;
        let form = Form::new()
            .text("file", "hoge-able")
            .part("csv", part);

        // Send request
        let response = Client::new()
            .post(&format!("http://{}/upload", addr))
            .multipart(form)
            .send()
            .await?;

        // Assert response
        assert_eq!(response.status(), 200);
        assert_eq!(
            response.text().await.unwrap(),
            "Files uploaded successfully"
        );

        Ok(())
    }
}
