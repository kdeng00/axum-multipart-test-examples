# Axum Multipart Request Testing Examples

This repository summarizes testing techniques for multipart requests in the Axum framework, offering three different approaches and describing the benefits and usage scenarios for each.

## Overview

Testing multipart requests effectively in Axum requires a balance between speed, isolation, and realism. This repository offers the following approaches:

- **Oneshot:** A lightweight method that tests service logic directly without starting an HTTP server.
- **Axum Test:** A strategy that takes advantage of Axum-specific testing tools for an intuitive and efficient testing experience.
- **HTTP Server with Reqwest:** A full integration test that runs an actual server, simulating real-world client-server interactions.

Each approach provides its own set of advantages, allowing you to tailor your testing strategy to your project's needs.


## Examples

### 1. Oneshot Testing (`examples/oneshot.rs`)

A test method using the `oneshot` method in `tower::ServiceExt`.

- Tests can be done without actually starting the HTTP server
- Use the `common-multipart-rfc7578` crate to build multipart Forms
    - Flexibly read from files to identify mime

### 2. Axum Test (`examples/axum_test_example.rs`)

Testing methodology using `axum-test` crates specialized for `axum` testing.

- Tests can be written with an intuitive API
- No need to start a server by using `mock_trasport`.
- Full set of response assertions

### 3. HTTP Server with Reqwest (`examples/http_server_with_reqwest.rs`)

A test method that starts an actual HTTP server and uses a `reqwest` client.

- Test communication with the actual HTTP server
- End-to-end testing scenarios are possible
- Tests can be performed in a manner close to the actual client code

## Test examples

To execute the tests for each approach, simply run:

```bash
# Oneshot
cargo test --example oneshot

# HTTP Server
cargo test --example http_server_with_reqwest

# Axum Test
cargo test --example axum_test_example
```

