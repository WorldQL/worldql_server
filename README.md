# WorldQL Server [![Build](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml/badge.svg)](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml) [![Docker Build](https://github.com/WorldQL/worldql_server/actions/workflows/docker.yml/badge.svg)](https://github.com/WorldQL/worldql_server/actions/workflows/docker.yml)
*Rust implementation of the WorldQL server.*

## Running WorldQL
To run WorldQL you will first need the executable binary. The easiest way is by using the pre-built binaries available on this repo's [GitHub Actions](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml) tab.
Click on a workflow run and scroll to the bottom and you will find build artifacts containing the pre-built binary.

If you want to build from source, for example if building for a platform not supported by CI, then follow the setup instructions below. **Make sure to build in release mode**, as Rust will optimise the
final binary and you should get greater performance. To build in release mode use `cargo build --release`, and the resulting binary will end up in the `target/release` directory.

## Setup Instructions
These instructions assume you are using a Debian-based Linux (or WSL) distro, such as Ubuntu. If using WSL, make sure you are using WSL2, as WSL1 has compatibility issues and may not work as expected.

First, install dependency packages for the Rust toolchains.
```sh
$ sudo apt-get install -y curl build-essential cmake
```

Next you will need to install Rust, the easiest way is by using [Rustup](https://rustup.rs/).

### Cloning the Project
**If using WSL, ensure you clone the project inside the WSL filesystem.** If you use the Windows filesystem (any paths starting with `/mnt`) then you may experience slower compile times.

### Building and Running
The project is managed by Rust's package manager and build tool, Cargo, and should come preinstalled with Rust.

To build the project, simply run `cargo build` in the project's root directory.  
To build and run in a single command, you can use `cargo run`. To pass WorldQL CLI flags to `cargo run`, use `cargo run -- <...flags>`

```bash
# Example using cargo run
$ cargo run -- --psql "postgres://username:password@localhost/database_name"

# Example using cargo build
$ cargo build --release
$ ./target/release/worldql_server --psql "postgres://username:password@localhost/database_name"
```

WorldQL is configured either using environment variables or CLI flags. Run with `--help` to list flags and their associated environment variables. Note that CLI flags will always take priority.

### Using VSCode
Ensure you have the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension for excellent Rust language support. If using WSL, you will also need the [Remote - WSL](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) extension to be able to open the project from within the WSL filesystem.

### Using CLion
Simply open the project after installing the [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust/docs) and everything should work correctly; you will only need to change one setting. Under `Languages & Frameworks` > `Rust` > `rustfmt` ensure `Use rustfmt instead of built-in formatter` is checked.

## Viewing Documentation
The best documentation available is our hand-written docs site, available at https://docs.worldql.com/, or view the source at [WorldQL/docs.worldql.com](https://github.com/WorldQL/docs.worldql.com)

You can also view the project's auto-generated Rustdoc documentation by running the `./view_documentation.sh` script, located in the project's root directory.

## Formatting and Linting
WorldQL uses [rustfmt](https://github.com/rust-lang/rustfmt) for formatting and [Clippy](https://github.com/rust-lang/rust-clippy) for linting. Both come as standard with a Rustup toolchain installation, but if you don't have them you can simply use Rustup to install them. **Note that the current rustfmt formatting rules requires the use of Rust nightly.**

```sh
# Install Rustup (requires nightly toolchain)
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly

# Install Clippy
rustup component add clippy
```

Once both are installed, simply run `cargo +nightly fmt --verbose` to apply formatting and `cargo clippy` to lint. [GitHub Actions CI](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml) runs both formatting and linting checks on pushes to `trunk` and for any pull requests to ensure standard formatting and catch lint failures.
