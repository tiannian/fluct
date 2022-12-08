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

use std::cmp::min;

use digest::Digest;
use evm::{
    executor::stack::{PrecompileFailure, PrecompileOutput},
    ExitError, ExitSucceed,
};
use k256::ecdsa::{self, recoverable};
use sha2::Sha512_256;
use sha3::Keccak256;

/// The identity precompile.
pub struct Identity;

impl Identity {
    pub const BASE: u64 = 15;
    pub const WORD: u64 = 3;

    pub fn execute(input: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: input.to_vec(),
        })
    }
}

/// The ecrecover precompile.
pub struct ECRecover;

fn secp256k1_ecdsa_recover(sig: [u8; 65], msg: [u8; 32]) -> Result<[u8; 33], ecdsa::Error> {
    let signture = recoverable::Signature::try_from(sig.as_ref())?;
    let pk = signture.recover_verifying_key_from_digest_bytes(&msg.into())?;
    Ok(pk.to_bytes().into())
}

impl ECRecover {
    pub const BASE: u64 = 3000;
    pub const WORD: u64 = 0;

    pub fn execute(i: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        let mut input = [0u8; 128];
        input[..min(i.len(), 128)].copy_from_slice(&i[..min(i.len(), 128)]);

        let mut msg = [0u8; 32];
        let mut sig = [0u8; 65];

        msg[0..32].copy_from_slice(&input[0..32]);
        sig[0..32].copy_from_slice(&input[64..96]);
        sig[32..64].copy_from_slice(&input[96..128]);
        sig[64] = input[63];

        let result = match secp256k1_ecdsa_recover(sig, msg) {
            Ok(pubkey) => {
                let mut address = Keccak256::digest(&pubkey);
                address[0..12].copy_from_slice(&[0u8; 12]);
                address.to_vec()
            }
            Err(_) => [0u8; 0].to_vec(),
        };

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: result,
        })
    }
}

/// The ripemd precompile.
pub struct Ripemd160;

impl Ripemd160 {
    pub const BASE: u64 = 600;
    pub const WORD: u64 = 120;

    pub fn execute(input: &[u8], _cost: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        let mut ret = [0u8; 32];
        ret[12..32].copy_from_slice(&ripemd::Ripemd160::digest(input));
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: ret.into(),
        })
    }
}

/// The sha256 precompile.
pub struct Sha256;

impl Sha256 {
    pub const BASE: u64 = 60;
    pub const WORD: u64 = 12;

    pub fn execute(input: &[u8], _cost: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        let ret = Sha512_256::digest(input);
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: ret.to_vec(),
        })
    }
}

/// The ECRecoverPublicKey precompile.
/// Similar to ECRecover, but returns the pubkey (not the corresponding Ethereum address)
pub struct ECRecoverPublicKey;

impl ECRecoverPublicKey {
    pub const BASE: u64 = 3000;
    pub const WORD: u64 = 0;

    pub fn execute(i: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        let mut input = [0u8; 128];
        input[..min(i.len(), 128)].copy_from_slice(&i[..min(i.len(), 128)]);

        let mut msg = [0u8; 32];
        let mut sig = [0u8; 65];

        msg[0..32].copy_from_slice(&input[0..32]);
        sig[0..32].copy_from_slice(&input[64..96]);
        sig[32..64].copy_from_slice(&input[96..128]);
        sig[64] = input[63];

        let pubkey = secp256k1_ecdsa_recover(sig, msg).map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("Public key recover failed".into()),
        })?;

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: pubkey.to_vec(),
        })
    }
}
