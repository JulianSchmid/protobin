# 01: Raise MSRV to 1.81

**What to build:** The crate declares and builds on Rust 1.81 as its minimum supported version, so later tickets can use `core::error::Error` (stable since 1.81). The CI MSRV job builds on 1.81 and stays build-only: CI resolves the newest proptest, which needs a newer toolchain than 1.81, so tests can't run there. The MSRV constraint in `CLAUDE.md` states the new version and its reason. This is part of the 0.7.0 coding-style cleanup (see `docs/coding-style.md`).

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [x] The crate manifest declares `rust-version` 1.81
- [x] The CI MSRV job builds with toolchain 1.81, and its job name says so
- [x] `cargo +1.81 build` succeeds locally
- [x] The MSRV constraint in `CLAUDE.md` says 1.81, gives `core::error::Error` as the reason, and no longer cites the 1.65 GAT / `let ... else` rationale
- [x] `Changelog.md` has a `## 0.7.0` heading (create it if missing), with the MSRV raise recorded under "Changed"
- [x] All CI checks pass
