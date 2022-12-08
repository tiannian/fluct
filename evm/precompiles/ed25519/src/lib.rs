// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ed25519_dalek::{PublicKey, Signature, Verifier};
use evm::{
    executor::stack::{PrecompileFailure, PrecompileOutput},
    ExitError, ExitSucceed,
};

pub struct Ed25519Verify;

impl Ed25519Verify {
    pub const BASE: u64 = 15;
    pub const WORD: u64 = 3;

    pub fn execute(input: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        if input.len() < 128 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("input must contain 128 bytes".into()),
            });
        };

        let mut i = [0u8; 128];
        i[..128].copy_from_slice(&input[..128]);

        let mut buf = [0u8; 4];

        let msg = &i[0..32];
        let pk = PublicKey::from_bytes(&i[32..64]).map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("Public key recover failed".into()),
        })?;
        let sig = Signature::try_from(&i[64..128]).map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("Signature recover failed".into()),
        })?;

        // https://docs.rs/rust-crypto/0.2.36/crypto/ed25519/fn.verify.html
        if pk.verify(msg, &sig).is_ok() {
            buf[3] = 0u8;
        } else {
            buf[3] = 1u8;
        };

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: buf.to_vec(),
        })
    }
}
