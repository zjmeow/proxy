use crate::error::MyProgramError;
use crate::programs::{HYLO, JUPITER_PROGRAM_ID};
use crate::util;
use crate::util::{invoke_dynamic_unchecked, to_account_metas};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::Instruction;
use pinocchio::ProgramResult;
use pinocchio_log::log;

pub fn process(
    jup_account: &[AccountInfo],
    pool_accounts: &[AccountInfo],
    jup_data: &[u8],
    pool_data: &[u8],
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
    let new_jup_data = util::replace_u64_at(jup_data, -19, input)?;
    let jup_instruction = Instruction {
        program_id: &JUPITER_PROGRAM_ID,
        accounts: &*to_account_metas(jup_account),
        data: &new_jup_data,
    };
    invoke_dynamic_unchecked(&jup_instruction, jup_account)?;
    let after_start = util::reload_amount(mint_a)?;
    if after_start < before_start {
        return Err(MyProgramError::NoProfit.into());
    }
    Ok(())
}
