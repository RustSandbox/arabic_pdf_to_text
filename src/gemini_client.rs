use anyhow::{Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Debug, Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Part {
    Text {
        text: String,
    },
    FileData {
        #[serde(rename = "file_data")]
        file_data: FileData,
    },
}

#[derive(Debug, Serialize)]
struct FileData {
    #[serde(rename = "mime_type")]
    mime_type: String,
    #[serde(rename = "file_uri")]
    file_uri: String,
}

#[derive(Debug, Serialize)]
struct FileUploadRequest {
    file: FileMetadata,
}

#[derive(Debug, Serialize)]
struct FileMetadata {
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct FileUploadResponse {
    file: FileInfo,
}

#[derive(Debug, Deserialize)]
struct FileInfo {
    uri: String,
    #[allow(dead_code)]
    name: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

pub struct GeminiClient {
    api_key: String,
    client: Client,
    base_url: String,
}

impl GeminiClient {
    pub fn new(api_key: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minute timeout
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            client,
            base_url: "https://generativelanguage.googleapis.com".to_string(),
        }
    }

    #[cfg(test)]
    fn with_base_url(api_key: &str, base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            client,
            base_url: base_url.to_string(),
        }
    }

    async fn upload_file(&self, pdf_data: &[u8], display_name: &str) -> Result<String> {
        let num_bytes = pdf_data.len();

        use colored::*;
        println!(
            "\n{} {}",
            "🔐".cyan(),
            format!("Initiating secure file transfer for: {}", display_name)
                .cyan()
                .bold()
        );
        println!(
            "  {} File size: {} bytes",
            "📊".yellow(),
            num_bytes.to_string().green()
        );

        // Step 1: Initial resumable request
        let upload_init_url = format!("{}/upload/v1beta/files?key={}", self.base_url, self.api_key);

        let file_metadata = FileUploadRequest {
            file: FileMetadata {
                display_name: display_name.to_string(),
            },
        };

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Goog-Upload-Protocol",
            HeaderValue::from_static("resumable"),
        );
        headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("start"));
        headers.insert(
            "X-Goog-Upload-Header-Content-Length",
            HeaderValue::from_str(&num_bytes.to_string())?,
        );
        headers.insert(
            "X-Goog-Upload-Header-Content-Type",
            HeaderValue::from_static("application/pdf"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let init_response = self
            .client
            .post(&upload_init_url)
            .headers(headers)
            .json(&file_metadata)
            .send()
            .await
            .context("Failed to initiate file upload")?;

        if !init_response.status().is_success() {
            let error_text = init_response.text().await?;
            anyhow::bail!("Failed to initiate upload: {}", error_text);
        }

        // Get upload URL from response headers
        let upload_url = init_response
            .headers()
            .get("x-goog-upload-url")
            .ok_or_else(|| anyhow::anyhow!("No upload URL in response"))?
            .to_str()?
            .to_string();

        println!(
            "\n{} {}",
            "🚀".green(),
            "Upload session initialized successfully!".green().bold()
        );
        println!("  {} Session URL acquired", "✓".green());
        println!("  {} Beginning data transmission...", "📡".cyan());

        // Step 2: Upload the actual bytes
        let mut upload_headers = HeaderMap::new();
        upload_headers.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&num_bytes.to_string())?,
        );
        upload_headers.insert("X-Goog-Upload-Offset", HeaderValue::from_static("0"));
        upload_headers.insert(
            "X-Goog-Upload-Command",
            HeaderValue::from_static("upload, finalize"),
        );

        let upload_response = self
            .client
            .post(&upload_url)
            .headers(upload_headers)
            .body(pdf_data.to_vec())
            .send()
            .await
            .context("Failed to upload file data")?;

        if !upload_response.status().is_success() {
            let error_text = upload_response.text().await?;
            anyhow::bail!("Failed to upload file: {}", error_text);
        }

        let file_info: FileUploadResponse = upload_response
            .json()
            .await
            .context("Failed to parse upload response")?;

        println!(
            "\n{} {}",
            "🎉".green(),
            format!("File uploaded successfully!").green().bold()
        );
        println!(
            "  {} Upload ID: {}",
            "🆔".cyan(),
            file_info.file.name.yellow()
        );
        println!(
            "  {} Cloud URI: {}",
            "☁️".cyan(),
            file_info.file.uri.blue().underline()
        );
        println!(
            "  {} Status: {}",
            "✅".green(),
            "Ready for AI processing".green()
        );

        Ok(file_info.file.uri)
    }

    pub async fn upload_full_pdf(&self, pdf_data: &[u8], display_name: &str) -> Result<String> {
        self.upload_file(pdf_data, display_name).await
    }

    pub async fn extract_page_range(
        &self,
        file_uri: &str,
        start_page: usize,
        end_page: usize,
    ) -> Result<String> {
        let request = GeminiRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![
                    Part::Text {
                        text: format!(
                            "Extract all text from pages {} to {} of this PDF document. \
                                Return ONLY the text content from those specific pages, \
                                preserving all Arabic text exactly as it appears. \
                                If these pages don't exist, return an empty response.",
                            start_page, end_page
                        ),
                    },
                    Part::FileData {
                        file_data: FileData {
                            mime_type: "application/pdf".to_string(),
                            file_uri: file_uri.to_string(),
                        },
                    },
                ],
            }],
            generation_config: GenerationConfig {
                response_mime_type: "text/plain".to_string(),
            },
        };

        let url = format!(
            "{}/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            self.base_url, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .context(format!(
                "Failed to process pages {}-{}",
                start_page, end_page
            ))?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!(
                "Gemini API error for pages {}-{}: {}",
                start_page,
                end_page,
                error_text
            );
        }

        let gemini_response: GeminiResponse = response.json().await.context(format!(
            "Failed to parse response for pages {}-{}",
            start_page, end_page
        ))?;

        gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No text found in response for pages {}-{}",
                    start_page,
                    end_page
                )
            })
    }

    pub async fn convert_to_text(&self, pdf_data: &[u8]) -> Result<String> {
        // Upload the file first
        let file_uri = self.upload_file(pdf_data, "arabic_pdf").await?;

        // Generate content using the uploaded file
        let request = GeminiRequest {
            contents: vec![
                Content {
                    role: "user".to_string(),
                    parts: vec![
                        Part::Text {
                            text: "Extract all text from this PDF document. Return the text exactly as it appears, preserving Arabic text.".to_string()
                        },
                        Part::FileData {
                            file_data: FileData {
                                mime_type: "application/pdf".to_string(),
                                file_uri,
                            }
                        }
                    ],
                },
            ],
            generation_config: GenerationConfig {
                response_mime_type: "text/plain".to_string(),
            },
        };

        let url = format!(
            "{}/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            self.base_url, self.api_key
        );

        println!("Sending request to generate content...");
        println!("This may take a while for large PDFs...");

        let response = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(600)) // 10 minute timeout for generation
            .send()
            .await
            .context("Failed to send request to Gemini API (timeout or network error)")?;

        println!("Response status: {}", response.status());

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No text found in Gemini response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_gemini_client_success() {
        let mut server = Server::new_async().await;

        // Mock the file upload initiation
        let upload_init_mock = server
            .mock("POST", "/upload/v1beta/files")
            .match_query(mockito::Matcher::Regex("key=test_key".to_string()))
            .match_header("x-goog-upload-protocol", "resumable")
            .with_status(200)
            .with_header("x-goog-upload-url", &format!("{}/upload/123", server.url()))
            .create_async()
            .await;

        // Mock the actual file upload
        let upload_mock = server
            .mock("POST", "/upload/123")
            .with_status(200)
            .with_body(r#"{"file": {"uri": "https://file-uri/123", "name": "files/123"}}"#)
            .create_async()
            .await;

        // Mock the generate content call
        let generate_mock = server
            .mock("POST", "/v1beta/models/gemini-2.5-flash:generateContent")
            .match_query(mockito::Matcher::Regex("key=test_key".to_string()))
            .with_status(200)
            .with_body(
                r#"{
                "candidates": [{
                    "content": {
                        "parts": [{"text": "مرحبا بالعالم"}]
                    }
                }]
            }"#,
            )
            .create_async()
            .await;

        let client = GeminiClient::with_base_url("test_key", &server.url());

        let test_pdf = b"test pdf data";
        let result = client.convert_to_text(test_pdf).await;

        upload_init_mock.assert_async().await;
        upload_mock.assert_async().await;
        generate_mock.assert_async().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "مرحبا بالعالم");
    }

    #[tokio::test]
    async fn test_gemini_client_upload_error() {
        let mut server = Server::new_async().await;

        // Mock upload initiation failure
        let upload_init_mock = server
            .mock("POST", "/upload/v1beta/files")
            .match_query(mockito::Matcher::Regex("key=test_key".to_string()))
            .with_status(400)
            .with_body("Invalid API key")
            .create_async()
            .await;

        let client = GeminiClient::with_base_url("test_key", &server.url());

        let test_pdf = b"test pdf data";
        let result = client.convert_to_text(test_pdf).await;

        upload_init_mock.assert_async().await;
        assert!(result.is_err());
    }
}
