executable: c-web
size: ? 
    - also dynamically linked
Runtime Info:
    - Idle:
        - CPU time: 
        - Memory: 
        - Threads: 
        - Ports: 
    - After 3 requests:
        - CPU time: 
        - Memory: 
        - Threads: 
        - Ports: 

Instructions:
1. Ensure you have a C compiler installed. Both gcc and clang should work.
2. cd into `c-web`
3. Run `make`
4. Run `./c-web` -- note this doesn't actually work yet

Request:
curl -X POST \
localhost:{port}/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
{3}

Dependencies
{Any libraries oustide the language's standard features}
- Mongoose
    - https://github.com/cesanta/mongoose
    - Web server library

Pros
- {}

Cons
- {}