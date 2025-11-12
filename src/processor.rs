
use crate::instruction::JupPayload;
use crate::programs::JUPITER_PROGRAM_ID;
use pinocchio::account_info::AccountInfo;
use pinocchio::cpi::invoke_signed;
use pinocchio::instruction::{AccountMeta, Instruction};
use pinocchio::ProgramResult;
const VAULT_SEED: &[u8] = b"vault";
struct Param {}
pub fn jup_swap(accounts: &[AccountInfo], payload: JupPayload) -> ProgramResult {
    let swap_instruction = Instruction {
        program_id: &JUPITER_PROGRAM_ID,
        accounts: &*to_account_metas(accounts),
        data: &*[],
    };
    let mut param = [];
    for acc in accounts {
        param.push(&acc);
    }
    // let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SEED, &[bumps.vault]]];
    let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SEED]];
    invoke_signed(&swap_instruction, &accounts, signer_seeds)?;
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
