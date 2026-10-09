# z0s-vault

A Solana program that holds SOL behind a checksummed Winternitz one-time
signature instead of an elliptic-curve key. It is the on-chain half of the
vault at [z0s.vercel.app/vault](https://z0s.vercel.app/vault).

> **Experimental and unaudited.** Do not deposit more than you can afford to
> lose. The z0s interface caps each vault at 0.1 SOL; **the program itself does
> not enforce a cap.**

| | |
|---|---|
| Program ID | `EvB3Ssbh15KNTH6rsE3qQnidimUBNvT8hypfxfZakZDn` |
| Network | Solana mainnet-beta |
| Loader | BPF Loader Upgradeable |
| Upgrade authority | `3mR75xXhaJ6qgXBubc7ZZRk7D46VWzjJb5Fg4KJ5YrKD` (upgradeable) |
| Deployed binary | 20,792 bytes, SHA-256 `57bc3e6e8d7250775724d2a6f8b0a2d34d948b67719a71a14d8652fffa7d9afb` |

## This source is the deployed program

Building this repository reproduces the on-chain binary byte for byte:

```sh
cargo build-sbf --arch v3
shasum -a 256 target/deploy/z0s_vault.so

solana program dump EvB3Ssbh15KNTH6rsE3qQnidimUBNvT8hypfxfZakZDn onchain.so
shasum -a 256 onchain.so
```

Both hashes are
`57bc3e6e8d7250775724d2a6f8b0a2d34d948b67719a71a14d8652fffa7d9afb` with
`cargo-build-sbf 4.1.0`, `platform-tools v1.54` (rustc 1.89.0). Other
toolchain versions may produce different bytes.

The program is still upgradeable, so the authority key above can replace its
code. Check the hash again before you trust it, and see
`solana program show EvB3Ssbh15KNTH6rsE3qQnidimUBNvT8hypfxfZakZDn` for the
current authority. The plan is to make the program immutable once it is
audited.

## What it changes

This is a fork of Dean Little's
[solana-winternitz-vault](https://github.com/blueshift-gg/solana-winternitz-vault)
(MIT) with one change. The upstream signature scheme signs the 32 bytes of a
Keccak-256 digest with no checksum. A signature reveals points partway along
each hash chain, and anyone can hash forward from them, so a forger can sign any
message whose digest bytes are all no larger than the signed ones. Finding one
is a brute-force search, feasible in practice for many signatures.

This program appends the standard Winternitz checksum (as in RFC 8391):

```
d_0 .. d_31 = keccak256(message)
C           = sum(255 - d_i)          in [0, 8160]
d_32, d_33  = C >> 8, C & 0xff
```

Lowering any message digit raises `C`, which raises a checksum digit, and
raising a digit would mean walking a chain backward: a preimage of the hash.
The full argument is at
[z0s.vercel.app/docs#checksum](https://z0s.vercel.app/docs#checksum).

## Scheme

- Hash: Keccak-256 truncated to 28 bytes (224 bits) per chain step.
- 34 chains: 32 message digits plus 2 checksum digits, base 256.
- Public key: `p_i = H^256(s_i)`; root `R = keccak256(p_0 || ... || p_33)`.
- Signature: `σ_i = H^(256 - d_i)(s_i)`, 34 × 28 = 952 bytes.
- Vault address: the program-derived address of seed `R`, so no Ed25519 key
  exists for it.

One key signs once. Every spend closes the vault.

## Instructions

The first byte of instruction data selects the instruction.

| Tag | Name | Data after the tag | Accounts |
|---|---|---|---|
| 0 | open | root (32) + bump (1) | payer (signer, writable), vault (writable), system program |
| 1 | split | signature (952) + amount (8, LE) + bump (1) | vault, split, refund (writable) |
| 2 | close | signature (952) + bump (1) | vault, refund (writable) |

- **open** creates the vault account (0 bytes, owned by this program) with
  rent paid by the payer. Deposits are plain system transfers.
- **close** signs the 32-byte refund address. It recovers the root from the
  signature, checks that it derives this vault, moves the whole balance to
  refund, and closes the vault. Changing the refund address invalidates the
  signature, so a submitted withdrawal cannot be redirected.
- **split** signs `amount || split || refund` (72 bytes), pays `amount` to
  split and the rest to refund, and closes the vault. Its transaction exceeds
  Solana's 1,232-byte limit without an address lookup table, so the z0s
  interface does not use it yet.

## Build

Requires the Solana CLI (Agave) with `cargo build-sbf`. The program must be
built for sBPF v3:

```sh
cargo build-sbf --arch v3
```

## Security

Unaudited. See [SECURITY.md](SECURITY.md) to report a problem privately.

## License

MIT. See [LICENSE](LICENSE).
