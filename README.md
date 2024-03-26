# Banking

## Description

This is a learning project aimed at understanding Rust web applications by creating a dummy banking system.

The project uses Rocket, a web framework for Rust, to handle HTTP requests and responses.

## Continuous Integration

This project uses GitHub Actions for continuous integration. The workflow is defined in `.github/workflows/ci.yml`.

The workflow is triggered on every `push` event and does the following:

1. Checks out the repository code.
2. Sets up the Rust toolchain on the runner.
3. Builds the project using `cargo build`.
4. Runs the tests using `cargo test`.

## How to Run

1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the server.

## How to Test

1. Navigate to the project directory.
2. Run `cargo test` to run the tests.
