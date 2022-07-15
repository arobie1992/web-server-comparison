executable: web-server
size: 6.8M
Runtime Info:
    - Idle:
        - CPU time: 0.01
        - Memory: 1.3MB
        - Threads: 6
        - Ports: 16
    - After 3 requests:
        - CPU time: 0.01
        - Memory: 1.8MB
        - Threads: 6
        - Ports: 23

Instructions:
1. install Go: https://go.dev/doc/install
2. cd into `go-web` directory
3. run `go build main.go`
4. run `./web-server`

Request:
curl -X POST \
localhost:8081/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Dependencies
N/A

Time
connect:transfer:total
0.009853:0.013956:0.014011
0.008616:0.010872:0.010929
0.007296:0.010115:0.010179

Pros
- Very small executable
- No external dependencies
- Popular language
- Native package manager & build system

Cons
- Very DIY, especially WRT HTTP handling
- Requires exported fields for JSON
- Requires annotating all JSON fields