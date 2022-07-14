executable: d-web
size: 24MB
Instructions:
1. Install D: https://dlang.org/install.html
2. cd into `d-web`
3. Run `source ~/dlang/dmd-2.100.0/activate`
4. Run `dub build`
5. Run `./d-web`

Request:
curl -X POST \
localhost:8082/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.007204:0.018770:0.018828
0.007111:0.009244:0.009299
0.007416:0.009627:0.009698

Dependencies
- vibe.d
    - https://code.dlang.org/packages/vibe-d 
    - web server; part of core language
- asdf
    - https://code.dlang.org/packages/asdf 
    - json serlialization/deserialization

Pros
- Pretty simple
- Nice support for json
- Web server library seems to be part of core language
- Familar syntax
- Native package manager & build system

Cons
- Unfamiliar language
- Not the best error messages
- Requires package for JSON support
- Larger executable
- Slowest first request time