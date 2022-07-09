executable: web-server
size: 6.8M

Request:
curl -X POST \
localhost:8081/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w %{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.006864:0.009942:0.009996
0.007616:0.009372:0.009416
0.007221:0.009173:0.009228
