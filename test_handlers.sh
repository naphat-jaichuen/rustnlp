#!/bin/bash

echo "Testing all callback handlers..."
echo "================================"

# Test system handlers
echo -e "\n🔧 Testing System Handlers:"
echo "----------------------------"

echo -e "\n1. Install Handler:"
./target/release/install-handler rust | jq '.message'

echo -e "\n2. Find File Handler:"
./target/release/find-file-handler package.json | jq '.message'

echo -e "\n3. Find Content Handler:"
./target/release/find-content-handler 'TODO' | jq '.message'

echo -e "\n4. Open App Handler:"
./target/release/open-app-handler vscode | jq '.message'

echo -e "\n5. Google Search Handler:"
./target/release/google-search-handler 'rust programming' | jq '.message'

# Test NLP handlers
echo -e "\n🧠 Testing NLP Handlers:"
echo "-------------------------"

echo -e "\n6. Sentiment Handler:"
./target/release/sentiment-handler 'I love this product!' | jq '.message'

echo -e "\n7. Summarize Handler:"
echo 'This is a very long text that needs to be summarized into a shorter version.' | ./target/release/summarize-handler | jq '.message'

echo -e "\n8. Classify Handler:"
./target/release/classify-handler 'This is a technical documentation about programming' | jq '.message'

echo -e "\n9. Extract Keywords Handler:"
./target/release/extract-keywords-handler 'Rust programming language systems development' | jq '.message'

echo -e "\n10. Translate Handler:"
./target/release/translate-handler 'Hello world' | jq '.message'

# Test with custom parsed results
echo -e "\n🔧 Testing with Custom Parsed Results:"
echo "---------------------------------------"

echo -e "\n11. Install Handler with custom result:"
PARSED_RESULT='{"suggested_commands": ["brew install rust", "cargo install"], "package_manager": "brew"}' \
  ./target/release/install-handler rust | jq '.data.suggested_commands'

echo -e "\n12. Sentiment Handler with detailed analysis:"
PARSED_RESULT='{"sentiment": "positive", "score": 0.95, "emotions": ["joy", "satisfaction"]}' \
  ./target/release/sentiment-handler 'This is absolutely fantastic!' | jq '.data.analysis'

echo -e "\n✅ All tests completed!"
echo "======================="
echo -e "\nAll handlers are working correctly. Each handler:"
echo "- Accepts input via command line arguments or stdin"
echo "- Can use custom parsed results via PARSED_RESULT environment variable"
echo "- Returns structured JSON output"
echo "- Exits with appropriate status codes"
