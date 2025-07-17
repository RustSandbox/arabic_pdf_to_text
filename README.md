# Arabic PDF to Text Converter

A Rust CLI tool that converts Arabic PDFs to text using Google's Gemini API.

## Features

- ✅ Uploads full PDF once using resumable upload protocol
- ✅ Processes PDFs in page ranges with MapReduce pattern
- ✅ Concurrent processing with rate limit handling
- ✅ Preserves Arabic text formatting exactly
- ✅ Progress tracking for each page range
- ✅ Automatic retry on rate limit errors

## How It Works

1. **Single Upload**: The entire PDF is uploaded once to Gemini API
2. **Page Range Processing**: The tool requests text extraction for specific page ranges (e.g., pages 1-5, 6-10, etc.)
3. **MapReduce Pattern**: Multiple page ranges are processed concurrently
4. **Rate Limiting**: Automatic delays and retries handle API rate limits
5. **Result Aggregation**: All extracted text is combined in the correct order

## Current Implementation

The tool successfully:
- Uploads PDFs using Gemini's resumable upload API
- Processes page ranges concurrently (default: 5 pages per chunk)
- Handles rate limiting with 6-second delays between requests
- Retries failed requests up to 3 times with 30-second delays
- Combines results maintaining page order

## Usage

```bash
# Set your API key
export GEMINI_API_KEY="your-api-key"

# Process a PDF
./arabic_pdf_to_text "path/to/arabic.pdf" -o output.txt

# With custom chunk size
./arabic_pdf_to_text "path/to/arabic.pdf" --chunk-size 524288 -o output.txt
```

## Recommendations

For production use, consider:
1. Using a PDF library to split PDFs at page boundaries instead of byte boundaries
2. Implementing a queue system to respect rate limits
3. Using a paid API tier for higher quotas
4. Caching processed chunks to avoid reprocessing

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```