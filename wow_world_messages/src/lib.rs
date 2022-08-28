#![allow(dead_code)]
#![forbid(unsafe_code)]
#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::missing_panics_doc,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::doc_markdown,
    clippy::unseparated_literal_suffix
)]
#![allow(
    unused,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_errors_doc,
    clippy::upper_case_acronyms,
    clippy::missing_const_for_fn,
    clippy::approx_constant,
    clippy::clone_on_copy,
    clippy::derive_partial_eq_without_eq
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use crate::util::read_u8_le;
use std::fmt::{Display, Formatter};
use std::io::{Error, Read, Write};

use wow_srp::header_crypto::{Decrypter, Encrypter};

pub mod errors;
pub mod helper;
mod traits;
pub(crate) mod util;
mod world;

pub use traits::*;
pub use world::*;

#[cfg(feature = "vanilla")]
pub use helper::aura_mask::AuraMask;
pub use helper::guid::Guid;
#[cfg(feature = "vanilla")]
pub use helper::update_mask::{
    UpdateContainer, UpdateCorpse, UpdateDynamicObject, UpdateGameObject, UpdateItem, UpdateMask,
    UpdatePlayer, UpdateUnit,
};

pub const DEFAULT_PORT: u16 = 8085;
