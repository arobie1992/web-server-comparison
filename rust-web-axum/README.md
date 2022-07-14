executable: rust-web
size: 19MB
Instructions:
1. Install Rust: https://www.rust-lang.org/tools/install
2. cd into `rust-web` directory
3. run `cargo build`
4. run `./target/debug/rust-web`

Request:
curl -X POST \
localhost:3000/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.008179:0.015017:0.015084
0.007344:0.010250:0.010321
0.009235:0.012959:0.013031

Dependencies
- axum
    - https://github.com/tokio-rs/axum
    - Web server framework
- reqwest
    - https://docs.rs/reqwest/0.9.18/reqwest/
    - Library for making HTTP calls
- serde
    - https://serde.rs/
    - JSON serialization/deserialization

Pros (vs Rocket)
- Familiar language to team
- First-party support for JSON
- Native package manager & build system
- No SPOF (vs Rocket) since it's maintained by the Tokio

Cons
- Slowest
- Doesn't handle as much as Rocket