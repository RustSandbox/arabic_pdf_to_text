# Arabic PDF to Text CLI

A command-line tool to convert Arabic PDFs to text using Google's Gemini API.

## Usage

```bash
arabic_pdf_to_text [OPTIONS] <PDF_PATH>
```

## Arguments

- `<PDF_PATH>`: Path to the Arabic PDF file to convert

## Options

- `-a, --api-key <API_KEY>`: Gemini API key (can also be set via GEMINI_API_KEY env var)
- `-c, --chunk-size <CHUNK_SIZE>`: Chunk size in bytes (default: 1048576 = 1MB)
- `-o, --output <OUTPUT>`: Output file path (default: stdout)
- `-h, --help`: Print help
- `-V, --version`: Print version

## Examples

### Basic usage (output to stdout)
```bash
export GEMINI_API_KEY="your-api-key"
./arabic_pdf_to_text "path/to/arabic.pdf"
```

### Save output to file
```bash
./arabic_pdf_to_text "path/to/arabic.pdf" -o output.txt
```

### Use smaller chunks for large PDFs
```bash
./arabic_pdf_to_text "path/to/arabic.pdf" --chunk-size 524288 -o output.txt
```

### Provide API key directly
```bash
./arabic_pdf_to_text "path/to/arabic.pdf" --api-key "your-api-key"
```

## How it works

1. The PDF is read and split into chunks based on the specified chunk size
2. Each chunk is processed asynchronously using the Gemini API
3. Results are aggregated in the correct order and output as text

## Notes

- The tool processes chunks in parallel for better performance
- Chunk size affects API usage - smaller chunks mean more API calls
- Default chunk size of 1MB works well for most PDFs