use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

// Open a vault at the program address for `root`, the public root of a fresh
// checksummed Winternitz key. data = root (32) || bump (1). Funds are added
// afterwards by a plain transfer.
pub fn open_vault(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() != 33 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let root: &[u8; 32] = data[..32].try_into().unwrap();
    let bump = [data[32]];

    let [payer, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let lamports = Rent::get()?.minimum_balance(0);
    let seeds = [Seed::from(root), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    CreateAccount {
        from: payer,
        to: vault,
        lamports,
        space: 0,
        owner: &crate::ID,
    }
    .invoke_signed(&signers)
}
