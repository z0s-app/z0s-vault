use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::instructions::is_vault;
use crate::wots::{recover_root, SIG_LEN};

// Spend part of a vault: send `amount` to `split`, the remainder to `refund`,
// and close the vault. data = signature (952) || amount (8, LE) || bump (1).
// The signed message binds the amount and both accounts, and the checksum means
// the signature cannot be re-used for a different spend. Read by reference so
// the signature never lands on the stack.
pub fn split_vault(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() != SIG_LEN + 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let signature: &[u8; SIG_LEN] = data[..SIG_LEN].try_into().unwrap();
    let amount = u64::from_le_bytes(data[SIG_LEN..SIG_LEN + 8].try_into().unwrap());
    let bump = data[SIG_LEN + 8];

    let [vault, split, refund] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // message = amount (8, LE) || split.key (32) || refund.key (32)
    let mut message = [0u8; 72];
    message[0..8].copy_from_slice(&amount.to_le_bytes());
    message[8..40].copy_from_slice(split.key());
    message[40..72].copy_from_slice(refund.key());

    let root = recover_root(signature, &message);
    if !is_vault(&root, bump, vault.key()) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let balance = vault.lamports();
    if amount > balance {
        return Err(ProgramError::InsufficientFunds);
    }

    *split.try_borrow_mut_lamports()? += amount;
    *refund.try_borrow_mut_lamports()? += balance - amount;
    vault.close()
}
