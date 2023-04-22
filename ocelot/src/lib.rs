// -*- mode: rust; -*-
//
// This file is part of ocelot.
// Copyright © 2019 Galois, Inc.
// See LICENSE for licensing information.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::type_complexity)]
#![cfg_attr(feature = "nightly", feature(test))]
#![cfg_attr(feature = "nightly", feature(stdsimd))]
#![cfg_attr(feature = "nightly", deny(missing_docs))]

extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(test)]
#[macro_use]
extern crate std;

mod errors;
#[cfg(feature = "utils_transpose")]
mod utils;

pub use crate::errors::Error;
pub mod oprf;
pub mod ot;
