mod close_vault;
mod open_vault;
mod split_vault;

pub use close_vault::close_vault;
pub use open_vault::open_vault;
pub use split_vault::split_vault;

use pinocchio::pubkey::Pubkey;

// Fast PDA check: a vault's address is the program address for seed = root.
// Recompute it with sha256 (Solana's PDA hash) and compare.
#[inline(always)]
pub(crate) fn is_vault(root: &[u8; 32], bump: u8, vault: &Pubkey) -> bool {
    let mut out = [0u8; 32];
    let parts: [&[u8]; 4] = [root, &[bump], crate::ID.as_ref(), b"ProgramDerivedAddress"];
    unsafe {
        pinocchio::syscalls::sol_sha256(
            &parts as *const _ as *const u8,
            parts.len() as u64,
            out.as_mut_ptr(),
        );
    }
    out.eq(vault)
}
