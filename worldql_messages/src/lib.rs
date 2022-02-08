//! # WorldQL Messages
//! Structs used to communicate with the WorldQL server.
//!
//! # Naming
//! The naming of this crate's API is from the perspective of the WorldQL Server itself.
//!
//! Incoming messages refer to messages that are server-bound, and outgoing messages
//! refer to messages that are client-bound.
//!
//! # Serialization
//! Enable the `msgpack` or `json` cargo features to expose serialization traits.
//! These are enabled by default so if you don't want them you can use `no-default-features`

#![forbid(unsafe_code)]
#![deny(private_in_public)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::unused_self,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::fn_params_excessive_bools,
    clippy::inefficient_to_string,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs
)]

pub mod client_bound;
pub mod common;
pub mod server_bound;

#[cfg(any(feature = "json", feature = "msgpack"))]
pub mod serialization;

mod macros;
