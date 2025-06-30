#!/bin/bash

# Test script to demonstrate the REST API usage
BASE_URL="http://localhost:3000"

echo "🚀 Testing Rust NLP Server API"
echo "================================"

# Start server in background (uncomment if needed)
# cargo run &
# SERVER_PID=$!
# sleep 3  # Wait for server to start

# Test 1: Health Check
echo "1. Testing Health Check..."
curl -s "$BASE_URL/health" | jq '.'
echo -e "\n"

# Test 2: List Available Models
echo "2. Testing Available Models..."
curl -s "$BASE_URL/models" | jq '.'
echo -e "\n"

# Test 3: Sentiment Analysis (Positive)
echo "3. Testing Sentiment Analysis (Positive)..."
curl -s -X POST "$BASE_URL/process/sentiment" \
  -H "Content-Type: application/json" \
  -d '{"text": "I absolutely love this amazing wonderful product! It is fantastic and makes me so happy!"}' | jq '.'
echo -e "\n"

# Test 4: Sentiment Analysis (Negative)
echo "4. Testing Sentiment Analysis (Negative)..."
curl -s -X POST "$BASE_URL/process/sentiment" \
  -H "Content-Type: application/json" \
  -d '{"text": "This is terrible, awful, and horrible. I hate it completely."}' | jq '.'
echo -e "\n"

# Test 5: Text Classification
echo "5. Testing Text Classification..."
curl -s -X POST "$BASE_URL/process/classify" \
  -H "Content-Type: application/json" \
  -d '{"text": "I love programming in Rust and working with machine learning and artificial intelligence algorithms."}' | jq '.'
echo -e "\n"

# Test 6: Text Summarization
echo "6. Testing Text Summarization..."
curl -s -X POST "$BASE_URL/process/summarize" \
  -H "Content-Type: application/json" \
  -d '{"text": "This is the first sentence of a long article. The middle part contains many details about various topics. There are multiple paragraphs with complex information. The conclusion provides important insights. This is the final sentence with key takeaways."}' | jq '.'
echo -e "\n"

# Test 7: Keyword Extraction
echo "7. Testing Keyword Extraction..."
curl -s -X POST "$BASE_URL/process/extract_keywords" \
  -H "Content-Type: application/json" \
  -d '{"text": "machine learning artificial intelligence technology programming software development computer science algorithms data structures programming programming programming"}' | jq '.'
echo -e "\n"

# Test 8: Question Answering
echo "8. Testing Question Answering..."
curl -s -X POST "$BASE_URL/process/question_answer" \
  -H "Content-Type: application/json" \
  -d '{"text": "What is machine learning and how does it work?"}' | jq '.'
echo -e "\n"

# Test 9: Translation (Mock)
echo "9. Testing Translation..."
curl -s -X POST "$BASE_URL/process/translate" \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello world, this is a test message."}' | jq '.'
echo -e "\n"

# Test 10: General Process Endpoint
echo "10. Testing General Process Endpoint..."
curl -s -X POST "$BASE_URL/process" \
  -H "Content-Type: application/json" \
  -d '{"text": "This is great!", "task": "sentiment"}' | jq '.'
echo -e "\n"

# Test 11: Error Handling - Invalid Task
echo "11. Testing Error Handling (Invalid Task)..."
curl -s -X POST "$BASE_URL/process" \
  -H "Content-Type: application/json" \
  -d '{"text": "Test text", "task": "invalid_task"}' | jq '.'
echo -e "\n"

# Test 12: Error Handling - Empty Text
echo "12. Testing Error Handling (Empty Text)..."
curl -s -X POST "$BASE_URL/process/sentiment" \
  -H "Content-Type: application/json" \
  -d '{"text": ""}' | jq '.'
echo -e "\n"

echo "✅ API Testing Complete!"

# Kill server if started in background
# if [ ! -z "$SERVER_PID" ]; then
#   kill $SERVER_PID
# fi
