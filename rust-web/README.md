executable: rust-web
size: 27M
Instructions:
1. Install Rust: https://www.rust-lang.org/tools/install
2. cd into `rust-web` directory
3. run `cargo build`
4. run `./target/debug/rust-web`

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
- rocket
    - https://rocket.rs/v0.5-rc/overview/
    - HTTP web framework
- reqwest
    - https://docs.rs/reqwest/0.9.18/reqwest/
    - Library for making HTTP calls
- serde
    - https://serde.rs/
    - JSON serialization/deserialization

Pros
- Fastest
- Familiar language to team
- First-party support for JSON
- Handles a lot for user

Cons
- Rocket has SPOF in maintainer. Has been somewhat absent recently
- Rocket is only release candidate for 0.5
- Rocket 0.4 requires Rust Nightly
- Largets executable
