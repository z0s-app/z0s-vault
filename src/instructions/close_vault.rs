use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::instructions::is_vault;
use crate::wots::{recover_root, SIG_LEN};

// Empty a vault into `refund` and close it. data = signature (952) || bump (1).
// The signed message is the refund account, so the whole balance can only move
// where the holder authorised. Read by reference, no stack copy.
pub fn close_vault(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() != SIG_LEN + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let signature: &[u8; SIG_LEN] = data[..SIG_LEN].try_into().unwrap();
    let bump = data[SIG_LEN];

    let [vault, refund] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let root = recover_root(signature, refund.key());
    if !is_vault(&root, bump, vault.key()) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    *refund.try_borrow_mut_lamports()? += vault.lamports();
    vault.close()
}
