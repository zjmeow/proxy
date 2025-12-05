use crate::error::MyProgramError;
use crate::programs::JUPITER_PROGRAM_ID;
use crate::util;
use crate::util::{invoke_dynamic_unchecked, to_account_metas};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::{AccountMeta, Instruction};
use pinocchio::ProgramResult;
use pinocchio_token::instructions::{CloseAccount, InitializeAccount3};

pub fn process(
    jup_account: &[AccountInfo],
    pool_accounts: &[AccountInfo],
    jup_data: &[u8],
    pool_data: &[u8],
    mint_a: &AccountInfo,
    mint_b: &AccountInfo,
    to_program: &AccountInfo,
    singer: &AccountInfo,
) -> ProgramResult {
    let before_start = util::reload_amount(mint_a)?; // swol 余额
    let before_mid = util::reload_amount(mint_b)?;
    let wsol_mint = util::reload_mint(mint_a)?;
    let close = CloseAccount {
        account: mint_a,
        destination: singer,
        authority: singer,
    };
    // SOL 开场的话？ SOL->BSOL JUP : 初始化 BSOL -> WSOL WSOL需要关单，要不然不知道到底赚了没有，所以 wsol 需要一直保持在未创建或者值为 0 的状态
    // BSOL->WSOL 关单 WSOL-> SOL -> BSOL 查看 BSOL 是否有变化，这个好像没啥问题，当没有 wsol 的时候 jup 会帮我创建一个
    close.invoke()?;
    let swap_instruction = Instruction {
        program_id: to_program.key(),
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
