#![no_std]

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

use digest::Digest;
use evm::{
    executor::stack::{PrecompileFailure, PrecompileOutput},
    ExitSucceed,
};
use sha3::{Sha3_256, Sha3_512};

pub struct Sha3FIPS256;

impl Sha3FIPS256 {
    pub const BASE: u64 = 60;
    pub const WORD: u64 = 12;

    pub fn execute(input: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        let output = Sha3_256::digest(input);
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: output.to_vec(),
        })
    }
}

pub struct Sha3FIPS512;

impl Sha3FIPS512 {
    pub const BASE: u64 = 60;
    pub const WORD: u64 = 12;

    pub fn execute(input: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        let output = Sha3_512::digest(input);
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: output.to_vec(),
        })
    }
}
