use evm::{
    executor::stack::{PrecompileFailure, PrecompileHandle, PrecompileOutput},
    ExitError,
};

pub fn ensure_linear_cost(
    target_gas: Option<u64>,
    len: u64,
    base: u64,
    word: u64,
) -> Result<u64, PrecompileFailure> {
    let cost = base
        .checked_add(word.checked_mul(len.saturating_add(31) / 32).ok_or(
            PrecompileFailure::Error {
                exit_status: ExitError::OutOfGas,
            },
        )?)
        .ok_or(PrecompileFailure::Error {
            exit_status: ExitError::OutOfGas,
        })?;

    if let Some(target_gas) = target_gas {
        if cost > target_gas {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::OutOfGas,
            });
        }
    }

    Ok(cost)
}

pub fn call_linear_cost(
    handle: &mut impl PrecompileHandle,
    execute: impl Fn(&[u8], u64) -> Result<PrecompileOutput, PrecompileFailure>,
    base: u64,
    word: u64,
) -> Result<PrecompileOutput, PrecompileFailure> {
    let target_gas = handle.gas_limit();
    let cost = ensure_linear_cost(target_gas, handle.input().len() as u64, base, word)?;

    handle.record_cost(cost)?;
    Ok(execute(handle.input(), cost)?)
}
