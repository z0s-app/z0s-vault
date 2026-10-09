// Checksummed Winternitz recovery, the mirror of the client-side signer.
// The published solana-winternitz 0.1.1 omits the checksum, which lets a signed
// message be stretched into a signature for another message. The two checksum
// chains here close that: lowering any message digit raises the checksum, and a
// signature chunk can only be hashed forward, never back. The client signer and
// this recovery must stay byte-for-byte identical.
//
// Hashing goes through pinocchio's own syscall declarations, which are correct
// under both the old and the static (sBPF v3) syscall conventions. The older
// hand-rolled syscall crates break under v3.

pub const HASH_LEN: usize = 28;
pub const MSG_CHAINS: usize = 32;
pub const CHAINS: usize = 34; // 32 message + 2 checksum
pub const SIG_LEN: usize = CHAINS * HASH_LEN; // 952

#[inline(always)]
pub(crate) fn keccak(parts: &[&[u8]]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        pinocchio::syscalls::sol_keccak256(
            parts as *const _ as *const u8,
            parts.len() as u64,
            out.as_mut_ptr(),
        );
    }
    out
}

// The 34 digits a message signs over: 32 digest bytes then a 2-byte checksum.
fn digits(message: &[u8]) -> [u16; CHAINS] {
    let digest = keccak(&[message]);
    let mut checksum: u16 = 0;
    for &byte in digest.iter().take(MSG_CHAINS) {
        checksum += 255 - byte as u16;
    }
    let mut out = [0u16; CHAINS];
    for i in 0..MSG_CHAINS {
        out[i] = digest[i] as u16;
    }
    out[32] = (checksum >> 8) & 0xff;
    out[33] = checksum & 0xff;
    out
}

// The public root a signature implies for a message. Equals the key's real
// root only when the signature was made for this exact message with this key.
pub fn recover_root(signature: &[u8; SIG_LEN], message: &[u8]) -> [u8; 32] {
    let d = digits(message);
    let mut buf = [0u8; SIG_LEN];
    let mut i = 0usize;
    while i < CHAINS {
        let base = i * HASH_LEN;
        let mut v = [0u8; HASH_LEN];
        v.copy_from_slice(&signature[base..base + HASH_LEN]);
        let mut n = d[i];
        while n > 0 {
            let full = keccak(&[&v]);
            v.copy_from_slice(&full[..HASH_LEN]);
            n -= 1;
        }
        buf[base..base + HASH_LEN].copy_from_slice(&v);
        i += 1;
    }
    keccak(&[&buf])
}
