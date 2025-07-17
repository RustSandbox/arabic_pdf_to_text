#!/bin/bash

# Demo script to showcase the verbose terminal UI

echo "🚀 Arabic PDF to Text Converter - Verbose UI Demo"
echo "================================================"
echo ""
echo "This demo will show the highly verbose and expressive terminal UI"
echo "that provides detailed feedback for every tiny operation!"
echo ""
echo "Press Enter to continue..."
read

# Clear the screen for a clean demo
clear

# Run with a small test PDF
if [ -f "test_small.pdf" ]; then
    echo "Using test_small.pdf for demo..."
    source ~/.zshrc && ./target/release/arabic_pdf_to_text test_small.pdf -o demo_output.txt
else
    echo "Creating a demo PDF first..."
    # Run with the first available PDF
    PDF_FILE=$(find . -name "*.pdf" -type f | head -1)
    if [ -n "$PDF_FILE" ]; then
        echo "Using $PDF_FILE for demo..."
        source ~/.zshrc && ./target/release/arabic_pdf_to_text "$PDF_FILE" -o demo_output.txt
    else
        echo "No PDF files found for demo!"
        echo "Please provide a PDF file to test the verbose UI."
    fi
fi

echo ""
echo "✨ Demo completed! Check demo_output.txt for the extracted text."