use pinocchio::account_info::AccountInfo;
use pinocchio::cpi::invoke_signed_unchecked;
use pinocchio::instruction::{Account, AccountMeta, Instruction};
use pinocchio::program_error::ProgramError;
use pinocchio::ProgramResult;
use pinocchio_token::state::TokenAccount;

pub fn invoke_dynamic_unchecked(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
) -> ProgramResult {
    let accounts: Vec<Account> = account_infos.iter().map(Account::from).collect();
    unsafe {
        invoke_signed_unchecked(instruction, &accounts, &[]);
    }
    Ok(())
}

pub fn to_account_metas(accounts: &[AccountInfo]) -> Vec<AccountMeta> {
    let mut metas = Vec::with_capacity(accounts.len());
    metas.append(
        &mut accounts
            .iter()
            .map(|acc| match acc.is_writable() {
                false => AccountMeta::new(acc.key(), acc.is_writable(), acc.is_signer()),
                true => AccountMeta::new(acc.key(), acc.is_writable(), acc.is_signer()),
            })
            .collect(),
    );
    metas
}

pub fn reload_amount(account: &AccountInfo) -> Result<u64, ProgramError> {
    let num = TokenAccount::from_account_info(account)?.amount();
    Ok(num)
}

pub fn replace_u64_at(data: &[u8], position: isize, new_value: u64) -> Result<Vec<u8>, ProgramError> {
    let len = data.len();

    // 计算实际位置
    let actual_position = if position >= 0 {
        position as usize
    } else {
        // 负数从末尾计算
        len.checked_sub((-position) as usize)
            .ok_or(ProgramError::InvalidInstructionData)?
    };

    // 检查是否有足够空间存储 u64
    if actual_position + 8 > len {
        return Err(ProgramError::InvalidInstructionData);
    }
    let mut modified = data.to_vec();
    modified[actual_position..actual_position + 8].copy_from_slice(&new_value.to_le_bytes());
    Ok(modified)
}