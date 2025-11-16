use crate::instruction::{jup_pool, pool_jup};
use borsh::BorshDeserialize;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::{entrypoint, ProgramResult};
use pinocchio_log::log;

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (&v, payload) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    log!("ins {}", v);
    match v {
        0 => {
            let (jup_account_length, jup_data, pool_data) = parse_pool_to_jup_data(payload)?;
            let mint_a = &accounts[0];
            let mint_b = &accounts[1];
            let remaining_accounts = &accounts[2..];
            let (jup_accounts, pool_accounts) =
                remaining_accounts.split_at(jup_account_length as usize);

            pool_jup::process(
                jup_accounts,
                pool_accounts,
                jup_data,
                pool_data,
                mint_a,
                mint_b,
            )?;
            Ok(())
        }
        1 => {
            let (jup_account_length, jup_data, pool_data) = parse_pool_to_jup_data(payload)?;
            let mint_a = &accounts[0];
            let mint_b = &accounts[1];
            let remaining_accounts = &accounts[2..];
            let (jup_accounts, pool_accounts) =
                remaining_accounts.split_at(jup_account_length as usize);
            log!("jup_data {}", jup_data);
            log!("pool_data {}", pool_data);
            log!("jup account {}", jup_accounts.len());
            log!("pool account {}", pool_accounts.len());
            jup_pool::process(
                jup_accounts,
                pool_accounts,
                jup_data,
                pool_data,
                mint_a,
                mint_b,
            )?;
            Ok(())
        }
        _ => Err(ProgramError::Custom(10001)),
    }
}
#[inline]
fn  parse_pool_to_jup_data(payload: &[u8]) -> Result<(u32, &[u8], &[u8]), ProgramError> {
    let (header_bytes, rest) = payload
        .split_first_chunk::<8>()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let jup_data_length = u32::from_le_bytes([
        header_bytes[0],
        header_bytes[1],
        header_bytes[2],
        header_bytes[3],
    ]) as usize;

    let jup_account_length = u32::from_le_bytes([
        header_bytes[4],
        header_bytes[5],
        header_bytes[6],
        header_bytes[7],
    ]);

    let (jup_data, pool_data) = rest
        .split_at_checked(jup_data_length)
        .ok_or(ProgramError::InvalidInstructionData)?;

    Ok((jup_account_length, jup_data, pool_data))
}
