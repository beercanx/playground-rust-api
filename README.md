# Playground Rust API

[![Rust](https://github.com/beercanx/playground-rust-api/actions/workflows/rust.yml/badge.svg)](https://github.com/beercanx/playground-rust-api/actions/workflows/rust.yml)
[![CodeQL](https://github.com/beercanx/playground-rust-api/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/beercanx/playground-rust-api/actions/workflows/github-code-scanning/codeql)
[![Dependabot Updates](https://github.com/beercanx/playground-rust-api/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/beercanx/playground-rust-api/actions/workflows/dependabot/dependabot-updates)

Playing around with Rust and HTTP to provide an API using [Axum](https://github.com/tokio-rs/axum).

## Requirements
* Rust 1.92
* _**(Windows only)**_ Build Tools for Visual Studio with these individual components:
  * Build Tools for Visual Studio can be found via https://visualstudio.microsoft.com/downloads
  * MSVC Build Tools for x86/x64 (latest)
  * Windows 11 SDK (10.0.22621.0)

## Doing things
Its just standard cargo stuff for now
### Build
```bash
cargo build
```
### Test
```bash
cargo test
```
### Run
```bash
cargo run
```

## Endpoints
```
curl -v http://127.0.0.1:8080

> GET / HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.13.0
> Accept: */*
> 
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< content-length: 13
< date: Tue, 10 Feb 2026 13:30:56 GMT
< 
Hello, World!
```
```
curl -v http://127.0.0.1:8080/json

> GET /json HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.13.0
> Accept: */*
> 
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 27
< date: Tue, 10 Feb 2026 13:31:07 GMT
< 
{"message":"Hello, World!"}
```
```
curl -v http://127.0.0.1:8080/html

> GET /html HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.13.0
> Accept: */*
> 
< HTTP/1.1 200 OK
< content-type: text/html; charset=utf-8
< content-length: 22
< date: Tue, 10 Feb 2026 13:31:15 GMT
< 
<h1>Hello, World!</h1>
```

## Reading List
* https://rust-lang.github.io/rustup/installation/windows-msvc.html
* https://medium.com/gitconnected/rust-http-server-frameworks-making-the-right-choice-513a61afa674
* https://medium.com/solo-devs/the-async-runtime-wars-and-why-i-just-want-to-process-some-http-requests-bb445cbdb17f
* https://medium.com/@AlexanderObregon/building-restful-apis-with-rust-and-warp-70a6159fd804
* https://dystroy.org/bacon/
* https://docs.rs/axum/latest/axum/index.html
* https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
* https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
* https://github.com/tokio-rs/axum/blob/main/examples/oauth/src/main.rs
* https://doc.rust-lang.org/stable/book/index.html
