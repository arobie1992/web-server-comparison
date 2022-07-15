executable: go-web-gin
size: 10MB
Runtime Info:
    - Idle:
        - CPU time: 0.01
        - Memory: 4.4MB
        - Threads: 6
        - Ports: 16
    - After 3 requests:
        - CPU time: 0.02
        - Memory: 5.7MB
        - Threads: 9
        - Ports: 26

Instructions:
1. install Go: https://go.dev/doc/install
2. cd into `go-web-gin` directory
3. run `go build`
4. run `./go-web-gin`

Request:
curl -X POST \
localhost:8086/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.008121:0.012240:0.012301
0.009022:0.012012:0.012074
0.007172:0.009067:0.009123

Dependencies
- Gin
    - https://github.com/gin-gonic/gin
    - Web server framework

Pros
- Much less DIY (vs Go net/http)
- Small executable
- Popular language
- Native package manager & build system
- Gin has great documentation

Cons
- Requires exported fields for JSON
- Requires annotating all JSON fields
- Comparatively slow