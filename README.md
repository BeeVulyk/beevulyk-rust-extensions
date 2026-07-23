# beevulyk-rust-extensions

## Origin

This crate is a BeeVulyk fork of [`ITYFT/yft-rust-extension`](https://github.com/ITYFT/yft-rust-extension)
at tag `0.1.2`. There is no domain coupling — the fork is a pure rename
(crate name and package metadata only). No code was rewritten. Upstream
functionality is preserved as-is.

Fork versioning restarts at `0.1.0`.

---

General-purpose Rust utility library: `AppStates`, `MyTimer`, date/time helpers,
string builders, collections, event loop, object pools, task completion,
base64/hex helpers, and array-bytes iterators. All dependencies come from
crates.io — no git deps.

Enable features per use case (see `Cargo.toml` `[features]`): `app-states`,
`date`, `event-loop`, `timers`, `objects-pool`, `task-completion`, `base64`,
`hex`, `array-bytes-iterator`, `async-array-bytes-iterator`.

## Test

```bash
cargo test --all-features
```
