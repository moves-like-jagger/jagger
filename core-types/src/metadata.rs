// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

/// Representation of metadata,
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Metadata {
    /// The key identifying the type of metadata.
    pub key: Vec<u8>,
    /// The value of the metadata.
    pub value: Vec<u8>,
}