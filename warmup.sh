#!/bin/bash

BASE_URL="https://mevlog.rs"

echo "Waiting 20 seconds for server to start..."
sleep 20

warmup_url() {
    local url="$1"
    local desc="$2"
    echo -n "Warming up $desc... "
    curl -s -o /dev/null -w "%{http_code}\n" "$url"
}

warmup_url "$BASE_URL/" "homepage"
warmup_url "$BASE_URL/explore" "/explore"
warmup_url "$BASE_URL/search" "/search"
warmup_url "$BASE_URL/search" "/search"
warmup_url "$BASE_URL/search" "/search"
warmup_url "$BASE_URL/search" "/search"
warmup_url "$BASE_URL/tx/0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef" "tx page (404 expected)"

echo "Warmup complete."
