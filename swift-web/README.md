executable: ? - static linking that gets bundled in .build folder
size: ? - doesn't produce single executable
Instructions:
{steps to build and run the executable}

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
{Any libraries oustide the language's standard features}
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
- Requires WSL for Windows development