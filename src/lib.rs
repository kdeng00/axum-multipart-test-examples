use axum::{extract::Multipart, routing::post, Router};

pub async fn handle_multipart(mut multipart: Multipart) -> Result<String, String> {
    while let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        let name = field.name().ok_or("Field name is required")?.to_string();

        match field.content_type() {
            Some("text/plain") => {
                let text = field.text().await.unwrap();
                println!("name: {:?}, text: {:?}", name, text)
            }
            Some("text/csv") => {
                let data = field.bytes().await.unwrap();
                let mut reader = csv::Reader::from_reader(data.iter().as_slice());
                let header = reader.headers().unwrap();
                println!("name: {:?}, csv header: {:?}", name, header);
            }
            _ => {}
        }
    }

    Ok("Files uploaded successfully".to_string())
}

pub fn create_app() -> Router {
    Router::new().route("/upload", post(handle_multipart))
}
