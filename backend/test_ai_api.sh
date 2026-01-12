#!/bin/bash

# 設定 API 基礎 URL（根據你的設定調整）
BASE_URL="${BASE_URL:-http://localhost:3030}"

# 顏色輸出
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== AI Editor API 測試 ==="
echo "Base URL: $BASE_URL"
echo ""

# 測試函數
test_api() {
    local endpoint=$1
    local name=$2
    local payload=$3
    
    echo -e "${YELLOW}測試: $name${NC}"
    echo "Endpoint: POST $BASE_URL$endpoint"
    echo "Payload: $payload"
    
    response=$(curl -s -w "\nHTTP_STATUS:%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "$BASE_URL$endpoint")
    
    http_status=$(echo "$response" | grep "HTTP_STATUS" | cut -d: -f2)
    body=$(echo "$response" | sed '/HTTP_STATUS/d')
    
    if [ "$http_status" -eq 200 ]; then
        echo -e "${GREEN}✓ 成功 (HTTP $http_status)${NC}"
        echo "Response: $body" | jq '.' 2>/dev/null || echo "$body"
    else
        echo -e "${RED}✗ 失敗 (HTTP $http_status)${NC}"
        echo "Response: $body"
    fi
    echo ""
}

# 測試案例 1: Improve - 正常情況
test_api "/improve" "Improve Text - 正常情況" \
    '{"text": "This is a test sentence. It has some grammar issues."}'

# 測試案例 2: Improve - 短文字
test_api "/improve" "Improve Text - 短文字" \
    '{"text": "Hello world"}'

# 測試案例 3: Fix - 正常情況
test_api "/fix" "Fix Text - 正常情況" \
    '{"text": "I has a cat. The cat is cute."}'

# 測試案例 4: Fix - 拼寫錯誤
test_api "/fix" "Fix Text - 拼寫錯誤" \
    '{"text": "Ths is a tst sentnce with speling erors."}'

# 測試案例 5: Longer - 正常情況
test_api "/longer" "Longer Text - 正常情況" \
    '{"text": "The weather is nice."}'

# 測試案例 6: Longer - 已經是長文字
test_api "/longer" "Longer Text - 長文字" \
    '{"text": "This is already a very long sentence that contains many words and should be expanded even further to test the API functionality."}'

# 測試案例 7: Shorter - 正常情況
test_api "/shorter" "Shorter Text - 正常情況" \
    '{"text": "This is a very long sentence that needs to be shortened to make it more concise and easier to read."}'

# 測試案例 8: Shorter - 已經是短文字
test_api "/shorter" "Shorter Text - 短文字" \
    '{"text": "Short."}'

# 測試案例 9: 空字串（邊界情況）
test_api "/improve" "Improve Text - 空字串" \
    '{"text": ""}'

# 測試案例 10: 特殊字元
test_api "/fix" "Fix Text - 特殊字元" \
    "{\"text\": \"Hello! How are you? I'm fine, thanks.\"}"

# 測試案例 11: 多行文字
test_api "/longer" "Longer Text - 多行文字" \
    '{"text": "Line 1.\nLine 2.\nLine 3."}'

# 測試案例 12: 非常長文字（壓力測試）
long_text="This is a very long text. " 
long_text=$(printf "%s" "$long_text"{1..50})
test_api "/shorter" "Shorter Text - 非常長文字" \
    "{\"text\": \"$long_text\"}"

# 測試案例 13: 無效 JSON（錯誤處理）
echo -e "${YELLOW}測試: 無效 JSON${NC}"
echo "Endpoint: POST $BASE_URL/improve"
echo "Payload: {invalid json}"
response=$(curl -s -w "\nHTTP_STATUS:%{http_code}" \
    -X POST \
    -H "Content-Type: application/json" \
    -d "{invalid json}" \
    "$BASE_URL/improve")
http_status=$(echo "$response" | grep "HTTP_STATUS" | cut -d: -f2)
if [ "$http_status" -ne 200 ]; then
    echo -e "${GREEN}✓ 正確拒絕無效 JSON [HTTP ${http_status}]${NC}"
else
    echo -e "${RED}✗ 應該拒絕無效 JSON${NC}"
fi
echo ""

# 測試案例 14: 缺少必要欄位
test_api "/improve" "Improve Text - 缺少 text 欄位" \
    '{}'

echo "=== 測試完成 ==="