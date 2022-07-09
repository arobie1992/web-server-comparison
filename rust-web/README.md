executable: rust-web
size: 27M

Request:
curl -X POST \
localhost:8080/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w %{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.006974:0.008106:0.008320
0.006972:0.008075:0.008189
0.007108:0.008123:0.008223

Dependencies:
https://rocket.rs/v0.5-rc/overview/
https://docs.rs/reqwest/0.9.18/reqwest/
https://serde.rs/