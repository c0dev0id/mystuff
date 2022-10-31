//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

#![allow(non_snake_case)]

use crate::common::simple_types::*;
use crate::crypto;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredential {
    pub(crate) reserved: ReservedBytes,
    pub(crate) credential: crypto::credentials::ProfileKeyCredential,
    pub(crate) uid_bytes: UidBytes,
    pub(crate) profile_key_bytes: ProfileKeyBytes,
}