executable: {executable name}
size: {executable size}
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
{steps to build and run the executable}

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
- {dep name}
    - {url to documentation}
    - {brief description}

Pros
- {}

Cons
- {}