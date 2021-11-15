# WorldQL Server [![Build](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml/badge.svg)](https://github.com/WorldQL/worldql_server/actions/workflows/build.yml)
*Rust implementation of the WorldQL server.*

## Setup Instructions
> **:warning: This project can only be built on unix-based systems due to the ZeroMQ library we're using only supporting epoll.** See [this issue](https://github.com/cetra3/tmq/issues/17) for more information.

These instructions assume you are using a Debian-based Linux (or WSL) distro, such as Ubuntu. If using WSL, make sure you are using WSL2, as WSL1 has compatibility issues and may not work as expected.

First, install dependency packages for the Rust toolchains.
```sh
$ sudo apt-get install -y curl build-essential cmake
```

Next you will need to install Rust, the easiest way is by using [Rustup](https://rustup.rs/).

### Cloning the Project
**If using WSL, ensure you clone the project inside the WSL filesystem.** If you use the Windows filesystem (any paths starting with `/mnt`) then you may experience slower compile times.

### Using VSCode
Ensure you have the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension for excellent Rust language support. If using WSL, you will also need the [Remote - WSL](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) extension to be able to open the project from within the WSL filesystem.

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
