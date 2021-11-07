# worldql_server
Rust implementation of the WorldQL server.

## Setup instructions

This project can only be built on Linux due to the ZeroMQ library we're using only supporting epoll.

First, install dependencies:
```shell
sudo apt install build-essential pkg-config cmake libzmq3-dev
```

Then install Rust from the website (use the WSL script if using WSL).

Finally, clone the repository and `cargo run`.


### For CLion
Simply open the project after installing the Rust plugin and everything should work correctly. You only need to change one setting. Under _Languages & Frameworks_ > _Rust_ > _Rustfmt_ check "Use rustfmt instead of built-in formatter".