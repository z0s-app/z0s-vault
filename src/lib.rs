// z0s checksummed Winternitz lamports vault. EXPERIMENTAL, UNAUDITED.
//
// A fork of Dean Little's solana-winternitz-vault (MIT) with one change: the
// signature scheme carries the standard Winternitz checksum (see wots.rs), so a
// signed spend cannot be re-used for a different spend. It holds native SOL
// only. Each key signs exactly once; every spend closes the vault.
//
// Instruction data is read by reference straight from the input buffer, never
// copied onto the stack, so the 952-byte signature does not blow the SBF frame.

#![allow(unexpected_cfgs)]

pub mod instructions;
pub mod wots;

use instructions::{close_vault, open_vault, split_vault};
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

// EvB3Ssbh15KNTH6rsE3qQnidimUBNvT8hypfxfZakZDn
pub const ID: Pubkey = [
    0xce, 0xc7, 0x11, 0x51, 0x38, 0x7a, 0x91, 0x0a, 0x63, 0xcf, 0x6e, 0x63, 0xb8, 0x29, 0x0a, 0x7c,
    0x3d, 0xfb, 0x0a, 0x30, 0x25, 0x6c, 0xad, 0xb0, 0x77, 0xd4, 0xa7, 0xdb, 0x4d, 0x3c, 0xda, 0x2d,
];

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match discriminator {
        0 => open_vault(accounts, data),
        1 => split_vault(accounts, data),
        2 => close_vault(accounts, data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
