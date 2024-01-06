
![GitHub Release](https://img.shields.io/github/v/release/jabibamman/frakt)
[![wakatime](https://wakatime.com/badge/github/jabibamman/frakt.svg)](https://wakatime.com/badge/github/jabibamman/frakt)
[![codecov](https://codecov.io/gh/jabibamman/frakt/branch/master/graph/badge.svg)](https://codecov.io/gh/jabibamman/frakt)
![GitHub issues](https://img.shields.io/github/issues/jabibamman/frakt.svg)

# Frakt - Fractal calculation

## Description

Frakt is a Rust-based project focused on the computation and visualization of fractals. This workspace includes several components, such as clients, servers, and shared libraries.

## Installation

### Prerequisites

Ensure you have Rust and Cargo installed on your system.

### Cloning the Repository

To clone the repository, run the following command:

```bash
git clone https://jabibamman/frakt.git
cd frakt
```

## Building and Running

### Development Build

To build all packages in development mode:

```bash
cargo build
```

### Release Build

To build all packages in release mode:

```bash
cargo build --release
```

To build a specific package in release mode:

```bash
cargo build -p <package_name> --release
```

### Running Specific Packages

To run a specific package, use `-p` followed by the package name. For example, to run the client package:

```bash
cargo run -p client
```

## Individual Packages

### Client

To build and run only the client:

```bash
cargo build -p client
cargo run -p client
```

### Server

To build the server library (used by other components):

```bash
cargo build -p server
cargo run -p server
```

### Shared

To build the shared library (used by other components):

```bash
cargo build -p shared
```

### Complex

To build and the complex library (used by other components):

```bash
cargo build -p complex
```

## Running the worker with CLI

To run the worker with CLI, use the following command:

You can read the CLI rustdoc documentation for more information on the CLI arguments.

```bash
cargo run -p client -- -h
```

## Running the server with CLI

To run the server with CLI, use the following command:

```bash
cargo run -p server -- -h
```

## Using verbose log level

To use the verbose log level, use the following command:

```bash
cargo run -p client -- -v
cargo run -p server -- -v
```

## Using debug log level

To use the debug log level, use the following command:

```bash
cargo run -p client -- -d
cargo run -p server -- -d
```

## Documentation

To generate documentation for all packages without including dependencies (recommended):

```bash
cargo doc --no-deps --open
```

To generate documentation for all packages including dependencies:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome. Please follow standard contribution guidelines for pull requests.
