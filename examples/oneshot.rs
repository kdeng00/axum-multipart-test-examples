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
    use axum::http::{CONTENT_TYPE, method::POST, Request};
    use axum::body::Body as AxumBody;
    use anyhow::Result;
    use tower::ServiceExt;
    use http_body_util::BodyExt;
    use common_multipart_rfc7578::client::multipart::{Form as MultipartForm, Body as MultipartBody};

    #[tokio::test]
    async fn test_with_oneshot() -> Result<()> {
        // Prepare
        let app = create_app();

        // Create multipart form
        let mut form = MultipartForm::default();
        form.add_text("file", "hoge-able");
        form.add_file("csv", "./dummy/test_upload.csv")?;

        // Create request
        let content_type = form.content_type();
        let body = MultipartBody::from(form);
        let req = Request::builder()
            .method(POST)
            .uri("/upload")
            .header(CONTENT_TYPE, content_type)
            .body(AxumBody::from_stream(body))?;

        // Send request
        let response = app.oneshot(req).await?;

        // Assert response
        assert_eq!(response.status(), 200);
        let response_body = response
            .into_body().collect().await?.to_bytes().to_vec();
        assert_eq!(
            String::from_utf8(response_body)?,
            "Files uploaded successfully".to_string()
        );

        Ok(())
    }
}
