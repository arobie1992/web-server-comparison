executable: {executable name}
size: {executable size}

Request:
curl -X POST \
localhost:{port}/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w %{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
{3}
