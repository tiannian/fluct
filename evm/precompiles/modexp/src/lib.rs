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

extern crate alloc;

use core::{
    cmp::{max, Ordering},
    ops::BitAnd,
};

use alloc::vec::Vec;
use evm::{
    executor::stack::{PrecompileFailure, PrecompileHandle, PrecompileOutput},
    ExitError, ExitSucceed,
};
use num::{BigUint, FromPrimitive, One, ToPrimitive, Zero};

pub struct Modexp;

const MIN_GAS_COST: u64 = 200;

// Calculate gas cost according to EIP 2565:
// https://eips.ethereum.org/EIPS/eip-2565
fn calculate_gas_cost(
    base_length: u64,
    exp_length: u64,
    mod_length: u64,
    exponent: &BigUint,
) -> u64 {
    fn calculate_multiplication_complexity(base_length: u64, mod_length: u64) -> u64 {
        let max_length = max(base_length, mod_length);
        let mut words = max_length / 8;
        if max_length % 8 > 0 {
            words += 1;
        }

        // Note: can't overflow because we take words to be some u64 value / 8, which is
        // necessarily less than sqrt(u64::MAX).
        // Additionally, both base_length and mod_length are bounded to 1024, so this has
        // an upper bound of roughly (1024 / 8) squared
        words * words
    }

    fn calculate_iteration_count(exp_length: u64, exponent: &BigUint) -> u64 {
        let mut iteration_count: u64 = 0;

        if exp_length <= 32 && exponent.is_zero() {
            iteration_count = 0;
        } else if exp_length <= 32 {
            iteration_count = exponent.bits() - 1;
        } else if exp_length > 32 {
            // construct BigUint to represent (2^256) - 1
            let bytes: [u8; 32] = [0xFF; 32];
            let max_256_bit_uint = BigUint::from_bytes_be(&bytes);

            // from the EIP spec:
            // (8 * (exp_length - 32)) + ((exponent & (2**256 - 1)).bit_length() - 1)
            //
            // Notes:
            // * exp_length is bounded to 1024 and is > 32
            // * exponent can be zero, so we subtract 1 after adding the other terms (whose sum
            //   must be > 0)
            // * the addition can't overflow because the terms are both capped at roughly
            //   8 * max size of exp_length (1024)
            iteration_count =
                (8 * (exp_length - 32)) + exponent.bitand(max_256_bit_uint).bits() - 1;
        }

        max(iteration_count, 1)
    }

    let multiplication_complexity = calculate_multiplication_complexity(base_length, mod_length);
    let iteration_count = calculate_iteration_count(exp_length, exponent);
    max(
        MIN_GAS_COST,
        multiplication_complexity * iteration_count / 3,
    )
}

// ModExp expects the following as inputs:
// 1) 32 bytes expressing the length of base
// 2) 32 bytes expressing the length of exponent
// 3) 32 bytes expressing the length of modulus
// 4) base, size as described above
// 5) exponent, size as described above
// 6) modulus, size as described above
//
//
// NOTE: input sizes are bound to 1024 bytes, with the expectation
//       that gas limits would be applied before actual computation.
//
//       maximum stack size will also prevent abuse.
//
//       see: https://eips.ethereum.org/EIPS/eip-198

impl Modexp {
    pub fn execute(
        handle: &mut impl PrecompileHandle,
    ) -> Result<PrecompileOutput, PrecompileFailure> {
        let input = handle.input();

        if input.len() < 96 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("input must contain at least 96 bytes".into()),
            });
        };

        // reasonable assumption: this must fit within the Ethereum EVM's max stack size
        let max_size_big = BigUint::from_u32(1024).expect("can't create BigUint");

        let mut buf = [0; 32];
        buf.copy_from_slice(&input[0..32]);
        let base_len_big = BigUint::from_bytes_be(&buf);
        if base_len_big > max_size_big {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("unreasonably large base length".into()),
            });
        }

        buf.copy_from_slice(&input[32..64]);
        let exp_len_big = BigUint::from_bytes_be(&buf);
        if exp_len_big > max_size_big {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("unreasonably large exponent length".into()),
            });
        }

        buf.copy_from_slice(&input[64..96]);
        let mod_len_big = BigUint::from_bytes_be(&buf);
        if mod_len_big > max_size_big {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("unreasonably large modulus length".into()),
            });
        }

        // bounds check handled above
        let base_len = base_len_big.to_usize().expect("base_len out of bounds");
        let exp_len = exp_len_big.to_usize().expect("exp_len out of bounds");
        let mod_len = mod_len_big.to_usize().expect("mod_len out of bounds");

        // input length should be at least 96 + user-specified length of base + exp + mod
        let total_len = base_len + exp_len + mod_len + 96;
        if input.len() < total_len {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("insufficient input size".into()),
            });
        }

        // Gas formula allows arbitrary large exp_len when base and modulus are empty, so we need to handle empty base first.
        let r = if base_len == 0 && mod_len == 0 {
            handle.record_cost(MIN_GAS_COST)?;
            BigUint::zero()
        } else {
            // read the numbers themselves.
            let base_start = 96; // previous 3 32-byte fields
            let base = BigUint::from_bytes_be(&input[base_start..base_start + base_len]);

            let exp_start = base_start + base_len;
            let exponent = BigUint::from_bytes_be(&input[exp_start..exp_start + exp_len]);

            // do our gas accounting
            let gas_cost =
                calculate_gas_cost(base_len as u64, exp_len as u64, mod_len as u64, &exponent);

            handle.record_cost(gas_cost)?;
            let input = handle.input();

            let mod_start = exp_start + exp_len;
            let modulus = BigUint::from_bytes_be(&input[mod_start..mod_start + mod_len]);

            if modulus.is_zero() || modulus.is_one() {
                BigUint::zero()
            } else {
                base.modpow(&exponent, &modulus)
            }
        };

        // write output to given memory, left padded and same length as the modulus.
        let bytes = r.to_bytes_be();

        // always true except in the case of zero-length modulus, which leads to
        // output of length and value 1.
        match bytes.len().cmp(&mod_len) {
            Ordering::Equal => Ok(PrecompileOutput {
                exit_status: ExitSucceed::Returned,
                output: bytes.to_vec(),
            }),
            Ordering::Less => {
                let mut ret = Vec::with_capacity(mod_len);
                ret.extend(core::iter::repeat(0).take(mod_len - bytes.len()));
                ret.extend_from_slice(&bytes[..]);
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: ret.to_vec(),
                })
            }
            Ordering::Greater => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("failed".into()),
            }),
        }
    }
}
