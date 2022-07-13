executable: main.exe
size: 9.6MB
Instructions:
1. Install OCaml: https://ocaml.org/docs/up-and-running
2. cd into ocamlweb
3. Run `eval $(opam env)`
4. Run `opam install dream cohttp cohttp-lwt-unix`
5. Run `dune build`
6. Run `_build/default/bin/main.exe`

Request:
curl -X POST \
localhost:8084/call \
-H "Content-Type: application/json" \
-d '{"url": "http://localhost:8080/resp"}' \
-v -w \\n%{time_connect}:%{time_starttransfer}:%{time_total}\\n

Time
connect:transfer:total
0.007229:0.010243:0.010309
0.007195:0.010507:0.010579
0.007542:0.009699:0.009751

Dependencies
- Dream
    - https://github.com/aantron/dream
    - Web server framework
- Cohttp
    - https://github.com/mirage/ocaml-cohttp
    - Library for making HTTP requests

Pros
- Small executable
- Dream is very feature rich and has excellent documentation
- Yojson is very flexible and convenient
- First-party build system and package manager in Dune and Opam

Cons
- Very unfamiliar syntax
- Requires WSL2 for Windows development
- Functional programming is a big conceptual shift from procedural
- Non-Dream documentation can be tricky to find