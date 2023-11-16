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

To build and run only the server:

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
