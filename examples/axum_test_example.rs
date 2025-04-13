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
    use anyhow::Result;
    use axum_test::multipart::{MultipartForm, Part};
    use axum_test::TestServer;
    use std::fs;

    #[tokio::test]
    async fn test_with_axum_test() -> Result<()> {
        // Prepare
        let app = create_app();
        let server = TestServer::builder().mock_transport().build(app)?;

        // Create multipart form
        let file = fs::read("./dummy/test_upload.csv")?;
        let part = Part::bytes(file)
            .file_name("test_upload.csv")
            .mime_type("text/csv");
        let form = MultipartForm::new()
            .add_text("file", "hoge-able")
            .add_part("csv", part);

        // Send request
        let response = server.post("/upload").multipart(form).await;

        // Assert response
        response.assert_status_ok();
        response.assert_text("Files uploaded successfully");

        Ok(())
    }
}
