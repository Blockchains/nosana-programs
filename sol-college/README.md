# Sol.College Nosana lab (devnet only)

This branch (`sol-college-devnet`) is a practice copy of nosana-ci/nosana-programs @ 79b6971 for Sol.College.
It is **not** the Nosana network: the programs are deployed on Solana **devnet** under fresh lab program ids,
paid for with a practice NOS mint that has no value. Nothing here talks to Nosana's mainnet programs or its private services.

| What | Lab address (devnet) |
|---|---|
| nosana-jobs | `4nFPjT5MgCtYheCHGACin9S8N56Cg1RTBAhFQv43Ybxb` |
| nosana-staking | `qqA8dv9jjMYZJgVKBdiCjSL9MYQviiEN4R93dVuYeJR` |
| nosana-rewards | `EWdwdb7Jbjd2hSYAUKz5NTTCJSRgkZNH6tbXprAv3LEY` |
| practice NOS mint | `GKkroxnqiJvxyVUXKu9BAEttwGkZH5zSn8zvZ5b5fPt7` |
| lab authority / market admin | `sZWwGfLUsuZSYW2vkF4RFBgQRQeq1HqXzzWPDPPFWQD` |
| upgrade authority | `Dc4Ho4frauwKjtrGiBNuDLpZv7LM8WS7CG6inggNtRwM` |
| market | `FxnjqvKLY7GXBrcUQwYdJ4SHNH9xq3F6BcAFMcZfx1Vh` |

The only source change is `common/src/id.rs`: the non-mainnet/devnet ids point at the lab keys above.

## Build (pinned)

* Solana `cargo-build-sbf` 4.1.0, `--tools-version v1.52`, rustc 1.89 (see `rust-toolchain.toml`), Anchor crates 1.0.2 from `Cargo.lock`.
* `cargo-build-sbf --tools-version v1.52` in each `programs/*` directory.
* sha256 of the deployed bytecode (checked against `solana program dump`):
  jobs `20e3ae8b7a16543a9e7078ce24a774543fbe5668cc46dd577c78e88d8de1a7a5`,
  rewards `a507873aedd67b25fd31e1880b3a364a65d2ef493bb1e0e27ec53c3245424912`,
  staking `85d07ac900b4a4085837d971b593327f4407ac4014f61b931f98f3206cb3b85e`.

## The campus worker (why there is no `nosana-node start`)

`nosana-node start` cannot run standalone: it registers with Nosana's private **host-manager** service
(register, market request, benchmark, heartbeat), tunnels through Nosana's FRP server, and pulls Nosana's images.
That service is not published. The lab therefore splits the node into the parts that are public:

* chain: `@nosana/kit` 2.24.5 (`createNosanaClient('devnet', …)`; devnet is forced, the node defaults to mainnet);
* execution: the **official executor** `nosana-node run <job.json> --podman <socket>` (nosana-ci/nosana-node @ 3f07617), against rootless podman 5.4.2;
* storage: a lab content store keyed by real IPFS CIDv0 (dag-pb/UnixFS), in place of Nosana's Pinata/IPFS gateway. Every read is hash-checked.

`lab/worker.mjs` runs the executor and classifies failures (`execution failure` with exit code and stderr, or `monitoring`).

## Initialise and operate

```
cd sol-college/lab && npm ci --ignore-scripts
DEVNET_RPC=<devnet url> KEYS=<dir with lab keypairs> node init.mjs        # fund, practice NOS, stake, rewards, market
DEVNET_RPC=<devnet url> KEYS=<dir> NOSANA_NODE_REPO=<built nosana-node> node lifecycle.mjs
```

`lifecycle.mjs` covers:
* **success**: list a job, claim it with `work`, run the deterministic container, post the result CID with `finish`, and the client reads state COMPLETED and the result;
* **rejection**: a wallet with no stake account cannot `work` (AccountNotInitialized); a wallet that is not the assigned node cannot `finish` (Unauthorized); finishing a finished job is refused by kit and, replayed on-chain, by the program (AccountNotInitialized); a wallet that did not post a job cannot `recover` it (Unauthorized);
* **failure diagnosis**: a job that exits 3 is finished with its result CID, and the client diagnoses it from chain + store (exit 3, stderr `lab error: missing input file /data/in.csv`);
* **recovery**: the node `quit`s a claimed job (state STOPPED), the client `recover`s the 600-unit deposit, re-lists, and the node completes it;
* **regression**: the deterministic job gives the same digest `31eef93a…4022a` on every run.

Evidence: `evidence/init-2026-11-22.jsonl`, `evidence/lifecycle-2026-11-22.jsonl` (devnet signatures).

## Reset

* Soft reset (start of every `lifecycle.mjs` run): delist queued jobs, take the node out of the node queue (`stop`), `recover` stopped jobs.
* Full reset: generate new program/mint/authority keys, patch `common/src/id.rs`, rebuild, deploy, run `init.mjs`. Close old programs with `solana program close` to reclaim SOL.

## Improvement backlog

1. `@nosana/kit` 2.24.5 `stake()` ignores `config.stakeAddress` (no `programAddress`), so it always targets mainnet staking; the lab calls the generated staking client directly.
2. kit and its generated jobs client have no `recover` instruction; the lab builds it from the Anchor discriminator.
3. kit errors hide the program logs; read `e.details.context.logs`.
4. `nosana-node` network option defaults to mainnet; a devnet-first or explicit flag would be safer for teaching.
5. A public, self-hostable host-manager (or a `--standalone` mode) would let `nosana-node start` run on a campus.
6. nosana-kit and nosana-node have no LICENSE file (package.json says MIT/ISC); this needs clarifying before redistribution.
7. The dashboard (GPL-3.0) depends on `api.nosana.com` (deployments, credits, payments, templates); it is not adapted.
