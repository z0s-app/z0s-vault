# Security

This program is experimental and has not been audited.

## Reporting a problem

Please report vulnerabilities privately through the bug report form at
[z0s.app/fund](https://z0s.app/fund). Sign in with the wallet a
bounty should go to and leave a contact so we can reply.

Do not open a public issue for anything that could put funds at risk until it
is fixed.

## Scope

- The on-chain program in this repository
  (`EvB3Ssbh15KNTH6rsE3qQnidimUBNvT8hypfxfZakZDn`).
- Anything that lets someone move SOL out of a vault without a valid
  signature from its key, lock funds, or reuse a signature for a different
  spend.

## Known limits

- The program does not cap deposits; the 0.1 SOL cap is enforced by the z0s
  interface only.
- Each key must sign only once. Signing twice with the same key weakens it.
- Every Solana transaction is still paid for with an Ed25519 signature.
- The program is upgradeable by the authority listed in the README.
