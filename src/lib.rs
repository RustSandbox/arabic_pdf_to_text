pub mod chunker;
pub mod gemini_client;
pub mod pdf_reader;
pub mod ui;

use crate::ui::VerboseUI;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Config {
    pub api_key: String,
    pub chunk_size: usize,
}

pub async fn process_pdf(path: &str, config: &Config) -> Result<String> {
    let ui = VerboseUI::new();
    ui.print_banner();

    // Read PDF with detailed feedback
    ui.print_file_info(path, 0); // We'll get size after reading
    let pdf_data = pdf_reader::read_pdf(path)?;
    let total_size = pdf_data.len();
    ui.print_file_info(path, total_size);

    // Upload the full PDF once with progress tracking
    ui.print_upload_start();
    let client = gemini_client::GeminiClient::new(&config.api_key);

    // Create upload progress bar
    let upload_pb = ui.create_upload_progress(total_size as u64);

    // Simulate upload progress (in real implementation, you'd update this during actual upload)
    tokio::spawn(async move {
        for i in 0..=100 {
            upload_pb.set_position((total_size as u64 * i / 100) as u64);
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }
        upload_pb.finish();
    });

    let file_uri = client.upload_full_pdf(&pdf_data, "arabic_pdf").await?;
    ui.print_upload_complete(&file_uri);

    // Process in page ranges
    let pages_per_chunk = 5; // Process 5 pages at a time
    let total_pages = 30; // Start with first 30 pages for testing
    let num_chunks = (total_pages + pages_per_chunk - 1) / pages_per_chunk;

    ui.print_processing_start(num_chunks, pages_per_chunk);

    // Create shared resources
    let client = Arc::new(client);
    let file_uri = Arc::new(file_uri);
    let ui = Arc::new(ui);

    // Create progress bars for each chunk
    let progress_bars: Vec<_> = (0..num_chunks)
        .map(|i| {
            let start = i * pages_per_chunk + 1;
            let end = ((i + 1) * pages_per_chunk).min(total_pages);
            Arc::new(Mutex::new(ui.create_chunk_progress(start, end)))
        })
        .collect();

    // Process page ranges concurrently (MapReduce pattern)
    // Limit to 2 concurrent requests to avoid rate limits
    let semaphore = Arc::new(tokio::sync::Semaphore::new(2));

    let tasks: Vec<_> = (0..num_chunks)
        .map(|chunk_idx| {
            let client = client.clone();
            let sem = semaphore.clone();
            let file_uri = file_uri.clone();
            let start_page = chunk_idx * pages_per_chunk + 1;
            let end_page = ((chunk_idx + 1) * pages_per_chunk).min(total_pages);

            let pb = progress_bars[chunk_idx].clone();
            let ui = ui.clone();

            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();

                // Update progress to processing
                {
                    let pb_lock = pb.lock().await;
                    ui.update_chunk_progress(&pb_lock, "processing", 20);
                }

                // Add delay to respect rate limits
                if chunk_idx > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
                }

                // Simulate some processing time
                {
                    let pb_lock = pb.lock().await;
                    ui.update_chunk_progress(&pb_lock, "processing", 50);
                }

                let mut retries = 3;
                let mut result = client
                    .extract_page_range(&file_uri, start_page, end_page)
                    .await;

                // Retry on rate limit errors
                while retries > 0 && result.is_err() {
                    if let Err(e) = &result {
                        if e.to_string().contains("RESOURCE_EXHAUSTED") {
                            {
                                let pb_lock = pb.lock().await;
                                ui.update_chunk_progress(&pb_lock, "rate_limited", 70);
                            }
                            ui.print_rate_limit_warning(chunk_idx + 1, 30);
                            retries -= 1;
                            result = client
                                .extract_page_range(&file_uri, start_page, end_page)
                                .await;
                        } else {
                            break;
                        }
                    }
                }

                match &result {
                    Ok(text) => {
                        let pb_lock = pb.lock().await;
                        if text.trim().is_empty() {
                            ui.update_chunk_progress(&pb_lock, "completed", 100);
                        } else {
                            ui.update_chunk_progress(&pb_lock, "completed", 100);
                            ui.print_extraction_stats(chunk_idx + 1, text.len());
                        }
                    }
                    Err(e) => {
                        let pb_lock = pb.lock().await;
                        ui.update_chunk_progress(&pb_lock, "failed", 100);
                        ui.print_error(&format!("Failed pages {}-{}: {}", start_page, end_page, e));
                    }
                }

                (chunk_idx, result)
            })
        })
        .collect();

    // Collect results in order
    let mut results = vec![String::new(); tasks.len()];
    let mut failed_ranges = Vec::new();
    let mut last_non_empty_chunk = 0;

    for task in tasks {
        let (index, result) = task.await?;
        match result {
            Ok(text) => {
                if !text.trim().is_empty() {
                    results[index] = text;
                    last_non_empty_chunk = index;
                }
            }
            Err(e) => {
                let start = index * pages_per_chunk + 1;
                let end = ((index + 1) * pages_per_chunk).min(total_pages);
                failed_ranges.push((start, end));
                eprintln!("Pages {}-{} failed: {}", start, end, e);
                results[index] = format!("[Pages {}-{} failed to process]", start, end);
            }
        }
    }

    // Trim results to last non-empty chunk
    results.truncate(last_non_empty_chunk + 1);

    // Calculate statistics
    let success_count = results
        .iter()
        .filter(|s| !s.is_empty() && !s.starts_with("[Pages"))
        .count();
    let total_chars: usize = results.iter().map(|s| s.len()).sum();

    ui.print_final_summary(num_chunks, success_count, failed_ranges.len(), total_chars);

    // Combine all results
    let final_text = results
        .into_iter()
        .filter(|s| !s.is_empty() && !s.starts_with("[Pages"))
        .collect::<Vec<_>>()
        .join("\n\n--- Page Break ---\n\n");

    Ok(final_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_size_calculation() {
        let data = vec![0u8; 1000];
        let chunks = chunker::chunk_pdf(&data, 100);
        assert_eq!(chunks.len(), 10);
    }

    #[tokio::test]
    async fn test_gemini_client_mock() {
        let _server = mockito::Server::new_async().await;
    }
}
