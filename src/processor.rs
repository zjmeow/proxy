use crate::programs::JUPITER_PROGRAM_ID;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::{Instruction};
use pinocchio::ProgramResult;
use crate::util::{invoke_dynamic_unchecked, to_account_metas};

pub fn jup_swap(accounts: &[AccountInfo], data: Vec<u8>) -> ProgramResult {
    let swap_instruction = Instruction {
        program_id: &JUPITER_PROGRAM_ID,
        accounts: &*to_account_metas(accounts),
        data: &data,
    };
    invoke_dynamic_unchecked(&swap_instruction, accounts)?;
    Ok(())
}
