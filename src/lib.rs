//! # ambientcg
//! Provides a relatively user-friendly Rust API for interacting with the
//! ambientCG v2 web API.
//! 
//! ambientCG is a public domain, free source of high quality PBR materials,
//! HDRIs, models and more. This crate isn't officially supported by, endorsed
//! by or created by ambientCG; instead, it's a community-built wrapper around
//! their own web APIs.
//! 
//! This crate was largely made for my own personal usage. It's incomplete and
//! probably could do with a lot more work. Nonetheless I'm publishing it now,
//! in it's semi-complete state, so that I can more easily use it in other
//! projects I'm working on. I don't intend for this to be used widely at the
//! moment.

pub mod errors;
pub mod json;
pub mod request;
pub mod response;

mod creation_method;
mod data_type;
mod sort;

pub use creation_method::*;
pub use data_type::*;
pub use sort::*;