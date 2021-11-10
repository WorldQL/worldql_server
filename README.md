# WorldQL Server [![Build](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml/badge.svg)](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml)
*Rust implementation of the WorldQL server.*

## Setup Instructions
> **:warning: This project can only be built on Linux due to the ZeroMQ library we're using only supporting epoll.** See [this issue](https://github.com/cetra3/tmq/issues/17) for more information.

Firstly, ensure you have Rust installed. The easiest way to install Rust is with [Rustup](https://rustup.rs/). WSL2 is supported but you may have issues with Rustup on WSL1.

### Using VSCode
You will need the [Remote - WSL extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) to open the project from within WSL, and the [Rust Analyzer extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) for excellent Rust language support. With both of these installed simply open the project using the Remote - WSL extension.

### Using CLion
Simply open the project after installing the [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust/docs) and everything should work correctly; you will only need to change one setting. Under `Languages & Frameworks` > `Rust` > `rustfmt` ensure `Use rustfmt instead of built-in formatter` is checked.

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
