executable: rust-web
size: 30MB
Instructions:
1. Install Rust: https://www.rust-lang.org/tools/install
2. cd into `rust-web` directory
3. run `cargo build`
4. run `./target/debug/rust-web`

Request:
curl -X POST \
localhost:8085/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.007769:0.014031:0.014086
0.007411:0.010742:0.010796
0.007513:0.010833:0.010886

Dependencies
- axum
    - https://actix.rs/
    - Web server framework
- reqwest
    - https://docs.rs/reqwest/0.9.18/reqwest/
    - Library for making HTTP calls
- serde
    - https://serde.rs/
    - JSON serialization/deserialization

Pros
- Familiar language to team
- First-party support for JSON
- Native package manager & build system
- No SPOF (vs Rocket) since it's maintained by the Tokio

Cons
- largest executable
- Seem to need to have things be pub so routes can be added in main.rs
    - Couldn't figure out how to configure this in lib.rs
- Doesn't handle as much as Rocket