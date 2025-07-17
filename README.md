# Arabic PDF to Text Converter

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/arabic_pdf_to_text.svg)](https://crates.io/crates/arabic_pdf_to_text)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![CI](https://github.com/RustSandbox/arabic_pdf_to_text/actions/workflows/ci.yml/badge.svg)](https://github.com/RustSandbox/arabic_pdf_to_text/actions/workflows/ci.yml)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

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

## Installation

### From crates.io

```bash
cargo install arabic_pdf_to_text
```

### From Source

```bash
# Clone the repository
git clone https://github.com/RustSandbox/arabic_pdf_to_text.git
cd arabic_pdf_to_text

# Build the project
cargo build --release

# The binary will be at target/release/arabic_pdf_to_text
```

### Prerequisites

- Rust 1.70 or later
- Google Gemini API key ([Get one here](https://makersuite.google.com/app/apikey))

## Configuration

1. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```

2. Add your Gemini API key:
   ```bash
   export GEMINI_API_KEY="your-api-key"
   ```

## Usage

```bash
# Process a PDF
./arabic_pdf_to_text "path/to/arabic.pdf" -o output.txt

# With custom chunk size
./arabic_pdf_to_text "path/to/arabic.pdf" --chunk-size 524288 -o output.txt

# See all options
./arabic_pdf_to_text --help
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

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Attribution for Commercial Use

While this software is free for any use including commercial, if you use it in a commercial product or service, we kindly request (but do not require) that you include the following attribution:

```
This product includes software developed by the arabic_pdf_to_text project
(https://github.com/RustSandbox/arabic_pdf_to_text)
```

## Acknowledgments

- Google Gemini API for providing the PDF processing capabilities
- The Rust community for excellent libraries and tools