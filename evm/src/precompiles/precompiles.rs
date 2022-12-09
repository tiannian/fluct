use evm::executor::stack::{PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet};
use fluct_evm_precompile_blake2::Blake2F;
use fluct_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use fluct_evm_precompile_curve25519::{Curve25519Add, Curve25519ScalarMul};
use fluct_evm_precompile_ed25519::Ed25519Verify;
use fluct_evm_precompile_modexp::Modexp;
use fluct_evm_precompile_sha3fips::{Sha3FIPS256, Sha3FIPS512};
use fluct_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use primitive_types::H160;

use super::linear_cost::call_linear_cost;

pub struct Precompiles {}

impl Default for Precompiles {
    fn default() -> Self {
        Self {}
    }
}

impl Precompiles {
    pub const SECP256K1_RECOVER: H160 = hash(1);
    pub const SHA256: H160 = hash(2);
    pub const RIPEMD160: H160 = hash(3);
    pub const IDENTITY: H160 = hash(4);
    pub const MODEXP: H160 = hash(5);
    pub const BN128_ADD: H160 = hash(6);
    pub const BN128_MUL: H160 = hash(7);
    pub const BN128_PAIRING: H160 = hash(8);
    pub const BLAKE2F: H160 = hash(9);

    pub const SHA3FIPS256: H160 = hash(1024);
    pub const SHA3FIPS512: H160 = hash(1025);
    pub const SECP256K1_RECOVER_PK: H160 = hash(1026);
    pub const CURVE25519_ADD: H160 = hash(1027);
    pub const CURVE25519_MUL: H160 = hash(1028);
    pub const ED25519_VERIFY: H160 = hash(1029);

    pub const ALL_ADDRESSES: [H160; 15] = [
        Self::SECP256K1_RECOVER,
        Self::SHA256,
        Self::RIPEMD160,
        Self::IDENTITY,
        Self::MODEXP,
        Self::BN128_ADD,
        Self::BN128_MUL,
        Self::BN128_PAIRING,
        Self::BLAKE2F,
        Self::SHA3FIPS256,
        Self::SHA3FIPS512,
        Self::SECP256K1_RECOVER_PK,
        Self::CURVE25519_ADD,
        Self::CURVE25519_MUL,
        Self::ED25519_VERIFY,
    ];
}

impl PrecompileSet for Precompiles {
    fn execute(
        &self,
        handle: &mut impl PrecompileHandle,
    ) -> Option<Result<PrecompileOutput, PrecompileFailure>> {
        let addr = handle.code_address();

        match addr {
            a if a == Self::SECP256K1_RECOVER => Some(call_linear_cost(
                handle,
                ECRecover::execute,
                ECRecover::BASE,
                ECRecover::WORD,
            )),
            a if a == Self::SHA256 => Some(call_linear_cost(
                handle,
                Sha256::execute,
                Sha256::BASE,
                Sha256::WORD,
            )),
            a if a == Self::RIPEMD160 => Some(call_linear_cost(
                handle,
                Ripemd160::execute,
                Ripemd160::BASE,
                Ripemd160::WORD,
            )),
            a if a == Self::IDENTITY => Some(call_linear_cost(
                handle,
                Identity::execute,
                Identity::BASE,
                Identity::WORD,
            )),
            a if a == Self::MODEXP => Some(Modexp::execute(handle)),
            a if a == Self::BN128_ADD => Some(Bn128Add::execute(handle)),
            a if a == Self::BN128_MUL => Some(Bn128Mul::execute(handle)),
            a if a == Self::BN128_PAIRING => Some(Bn128Pairing::execute(handle)),
            a if a == Self::BLAKE2F => Some(Blake2F::execute(handle)),
            a if a == Self::SHA3FIPS256 => Some(call_linear_cost(
                handle,
                Sha3FIPS256::execute,
                Sha3FIPS256::WORD,
                Sha3FIPS256::BASE,
            )),
            a if a == Self::SHA3FIPS512 => Some(call_linear_cost(
                handle,
                Sha3FIPS512::execute,
                Sha3FIPS512::WORD,
                Sha3FIPS512::BASE,
            )),
            a if a == Self::SECP256K1_RECOVER_PK => Some(call_linear_cost(
                handle,
                ECRecoverPublicKey::execute,
                ECRecoverPublicKey::WORD,
                ECRecoverPublicKey::BASE,
            )),
            a if a == Self::CURVE25519_ADD => Some(call_linear_cost(
                handle,
                Curve25519Add::execute,
                Curve25519Add::WORD,
                Curve25519Add::BASE,
            )),
            a if a == Self::CURVE25519_MUL => Some(call_linear_cost(
                handle,
                Curve25519ScalarMul::execute,
                Curve25519ScalarMul::WORD,
                Curve25519ScalarMul::BASE,
            )),
            a if a == Self::ED25519_VERIFY => Some(call_linear_cost(
                handle,
                Ed25519Verify::execute,
                Ed25519Verify::WORD,
                Ed25519Verify::BASE,
            )),
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160) -> bool {
        Precompiles::ALL_ADDRESSES.contains(&address)
    }
}

const fn hash(a: u64) -> H160 {
    let mut inner = [0u8; 20];

    let bytes = a.to_be_bytes();

    inner[12] = bytes[0];
    inner[13] = bytes[1];
    inner[14] = bytes[2];
    inner[15] = bytes[3];
    inner[16] = bytes[4];
    inner[17] = bytes[5];
    inner[18] = bytes[6];
    inner[19] = bytes[7];

    H160(inner)
}
