use crate::error::MyProgramError;
use crate::programs::{HYLO, JUPITER_PROGRAM_ID};
use crate::util;
use crate::util::{invoke_dynamic_unchecked, to_account_metas};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::Instruction;
use pinocchio::program_error::ProgramError;
use pinocchio::ProgramResult;

pub fn process(
    jup_account: &[AccountInfo],
    pool_accounts: &[AccountInfo],
    pool_data: &[u8],
    jup_data: &[u8],
    mint_a: &AccountInfo,
    mint_b: &AccountInfo,
) -> ProgramResult {
    let before_start = util::reload_amount(mint_a)?;
    let before_mid = util::reload_amount(mint_b)?;
    // 换xsol
    let swap_instruction = Instruction {
        program_id: &HYLO,
        accounts: &*to_account_metas(pool_accounts),
        data: &pool_data,
    };
    invoke_dynamic_unchecked(&swap_instruction, pool_accounts)?;
    let after_mid = util::reload_amount(mint_b)?;
    let input = after_mid - before_mid;
    let new_jup_data = modify_last_u64(jup_data, input)?;
    let jup_instruction = Instruction {
        program_id: &JUPITER_PROGRAM_ID,
        accounts: &*to_account_metas(jup_account),
        data: &new_jup_data,
    };
    invoke_dynamic_unchecked(&jup_instruction, jup_account)?;
    let after_start = util::reload_amount(mint_a)?;
    if after_start < before_start {
        return Err(MyProgramError::PdaMismatch.into());
    }
    Ok(())
}

#[inline]
fn modify_last_u64(jup_data: &[u8], new_value: u64) -> Result<Vec<u8>, ProgramError> {
    // 最后结构: u64 (8 bytes) + u64 (8 bytes) + u16 (2 bytes) + u8 (1 byte) = 19 bytes
    const TAIL_SIZE: usize = 19;

    if jup_data.len() < TAIL_SIZE {
        return Err(ProgramError::InvalidInstructionData);
    }

    let mut modified = jup_data.to_vec();

    // 第一个 u64 的位置是从后往前数第 19 个字节开始
    let first_u64_start = modified.len() - TAIL_SIZE;
    let first_u64_end = first_u64_start + 8;

    // 替换为新值 (小端序)
    modified[first_u64_start..first_u64_end].copy_from_slice(&new_value.to_le_bytes());

    Ok(modified)
}

#[inline]
fn parse_pool_to_jup_data(payload: &[u8]) -> Result<(u32, &[u8], &[u8]), ProgramError> {
    let (header_bytes, rest) = payload
        .split_first_chunk::<8>()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let jup_data_length = u32::from_le_bytes([
        header_bytes[0], header_bytes[1], header_bytes[2], header_bytes[3]
    ]) as usize;

    let jup_account_length = u32::from_le_bytes([
        header_bytes[4], header_bytes[5], header_bytes[6], header_bytes[7]
    ]);

    let (jup_data, pool_data) = rest
        .split_at_checked(jup_data_length)
        .ok_or(ProgramError::InvalidInstructionData)?;

    Ok((jup_account_length, jup_data, pool_data))
}