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

use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
    traits::Identity,
};
use evm::{executor::stack::{PrecompileFailure, PrecompileOutput}, ExitError, ExitSucceed};

// Adds at most 10 curve25519 points and returns the CompressedRistretto bytes representation
pub struct Curve25519Add;

impl Curve25519Add {
    pub const BASE: u64 = 60;
    pub const WORD: u64 = 12;

    pub fn execute(input: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        if input.len() % 32 != 0 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("input must contain multiple of 32 bytes".into()),
            });
        };

        if input.len() > 320 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    "input cannot be greater than 320 bytes (10 compressed points)".into(),
                ),
            });
        };

        let mut points = Vec::new();
        let mut temp_buf = <&[u8]>::clone(&input);
        while !temp_buf.is_empty() {
            let mut buf = [0; 32];
            buf.copy_from_slice(&temp_buf[0..32]);
            let point = CompressedRistretto::from_slice(&buf);
            points.push(point);
            temp_buf = &temp_buf[32..];
        }

        let sum = points
            .iter()
            .fold(RistrettoPoint::identity(), |acc, point| {
                let pt = point.decompress().unwrap_or_else(RistrettoPoint::identity);
                acc + pt
            });

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: sum.compress().to_bytes().to_vec()
        })
    }
}

// Multiplies a scalar field element with an elliptic curve point
pub struct Curve25519ScalarMul;

impl Curve25519ScalarMul {
    pub const BASE: u64 = 60;
    pub const WORD: u64 = 12;

    pub fn execute(input: &[u8], _: u64) -> Result<PrecompileOutput, PrecompileFailure> {
        if input.len() != 64 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    "input must contain 64 bytes (scalar - 32 bytes, point - 32 bytes)".into(),
                ),
            });
        };

        // first 32 bytes is for the scalar value
        let mut scalar_buf = [0; 32];
        scalar_buf.copy_from_slice(&input[0..32]);
        let scalar = Scalar::from_bytes_mod_order(scalar_buf);

        // second 32 bytes is for the compressed ristretto point bytes
        let mut pt_buf = [0; 32];
        pt_buf.copy_from_slice(&input[32..64]);
        let point: RistrettoPoint = CompressedRistretto::from_slice(&pt_buf)
            .decompress()
            .unwrap_or_else(RistrettoPoint::identity);

        let scalar_mul = scalar * point;
        Ok(PrecompileOutput{
            exit_status: ExitSucceed::Returned,
            output: scalar_mul.compress().to_bytes().to_vec(),
        })
    }
}
