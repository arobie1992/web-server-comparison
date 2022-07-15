executable: ? - static linking that gets bundled in .build folder
size: ? - doesn't produce single executable
Runtime Info:
    - Idle:
        - CPU time: 1.55
        - Memory: 2.3MB
        - Threads: 77
        - Ports: 103
    - After 3 requests:
        - CPU time: 1.62
        - Memory: 3.9MB
        - Threads: 83
        - Ports: 133

Instructions:
1. Install Swift
    - Linux & Mac: https://www.swift.org/download/
    - Windows: https://swiftforwindows.github.io/
2. cd into `swift-web`
3. run `swift run`

Request:
curl -X POST \
localhost:8083/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.010203:0.074613:0.074676
0.007371:0.010778:0.010870
0.007674:0.009056:0.009119

Dependencies
- Vapor
    - https://vapor.codes/
    - Web server framework

Pros
- Some level of null safety
- First party JSON support

Cons
- Vapor is incredibly heavy
- Swift doesn't compile to a single executable
    - This is due to mac: according to here it should be fine for Linux which is what we want: https://stackoverflow.com/questions/66036826/can-the-swift-compiler-create-a-single-executable-for-my-application
- Unfamilar language and syntax
- HTTP requests use async callbacks which may be unfamilar and requires wait group for sync operations
- Windows develeopment support doesn't appear to be the greatest (4.x.x on Windows vs 5.x.x on Linux & Mac)