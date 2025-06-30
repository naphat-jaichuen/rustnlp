#!/bin/bash

echo "Building all callback handlers..."

# Array of all binary names
binaries=(
    "install-handler"
    "find-file-handler"
    "find-content-handler"
    "get-file-from-handler"
    "show-tools-handler"
    "open-app-handler"
    "open-file-handler"
    "checkout-handler"
    "diff-handler"
    "google-search-handler"
    "ask-ai-handler"
    "sentiment-handler"
    "summarize-handler"
    "classify-handler"
    "extract-keywords-handler"
    "translate-handler"
    "question-answer-handler"
    "natural-language-handler"
)

# Build each binary
for binary in "${binaries[@]}"; do
    echo "Building $binary..."
    if cargo build --release --bin "$binary"; then
        echo "✓ Successfully built $binary"
    else
        echo "✗ Failed to build $binary"
        exit 1
    fi
done

echo ""
echo "All handlers built successfully!"
echo "Executables are available in target/release/"
echo ""
echo "Usage examples:"
echo "  ./target/release/install-handler rust"
echo "  echo 'Hello world' | ./target/release/sentiment-handler"
echo "  PARSED_RESULT='{\"custom\":\"data\"}' ./target/release/find-file-handler myfile.txt"
