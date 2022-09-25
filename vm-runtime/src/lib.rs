// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

pub mod interpreter;
pub mod loader;

extern crate alloc;
extern crate core;
