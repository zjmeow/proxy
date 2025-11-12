use crate::{instruction, jup_swap};
use crate::processor::jup_swap;
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
    log!("Entrypoint {}", instruction_data);
    let (&v, payload) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match v {
        0 => {
            let jup_payload = instruction::JupPayload::try_from_slice(&payload)
                .or(Err(ProgramError::InvalidArgument))?;
            return jup_swap(accounts, jup_payload);
        }
        _ => return Err(ProgramError::Custom(10001)),
    };
}