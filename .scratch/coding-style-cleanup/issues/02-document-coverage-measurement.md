# 02: Document how to measure test coverage

**What to build:** A contributor or agent can measure line and branch coverage with one documented command, `cargo llvm-cov --html`. `CLAUDE.md` lists it with its one-time setup. The coding style requires every branch and every error path to be tested, and the test-cleanup tickets (08–10) use this command to check that, comparing against a baseline recorded here.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] The "Commands" section of `CLAUDE.md` lists the coverage command and its one-time setup (installing `cargo-llvm-cov` and the `llvm-tools-preview` component)
- [x] The command runs cleanly on the current code and produces an HTML report
- [x] No dependency is added to the crate manifest
- [x] Baseline coverage per module (`builders`, `wire`, `decode`, `FieldNumber`) is recorded under a `## Comments` heading in this ticket file

## Comments

### Deviation: two commands

Stable `cargo llvm-cov --html` records no branch data, and `--branch` needs nightly. `CLAUDE.md` therefore documents the stable command for line and region coverage plus a nightly variant, `cargo +nightly llvm-cov --branch --doctests --html`, for branches. `--doctests` (also nightly-only) is included because `cargo test` runs the README doctests too. LLVM branch coverage counts only `if`/`&&`/`||`, so untested error-path `match` arms show up only as missed regions.

### Baseline coverage (2026-10-11, commit 16a6842, cargo-llvm-cov 0.9.1)

Modules aggregate the files under `src/<module>/`. `FieldNumber` is `field_number.rs` plus `field_number_into_error.rs`. Each cell shows the percentage, then (covered/total).

`cargo llvm-cov --html` (stable rustc 1.95.0, unit tests and proptests only; stable records no branch data):

| Module | Lines | Regions |
|---|---|---|
| builders | 38.3% (296/773) | 48.4% (624/1288) |
| decode | 88.0% (73/83) | 82.1% (92/112) |
| FieldNumber | 68.2% (45/66) | 69.4% (59/85) |
| wire | 63.9% (446/698) | 64.5% (523/811) |
| **total** | 53.1% (860/1620) | 56.5% (1298/2296) |

`cargo +nightly llvm-cov --branch --doctests --html` (adds branches and the README doctests; doctest coverage is attributed to `lib.rs`, which includes `README.md`). **This is the reference baseline for tickets 08–10**, since only it has branch data:

| Module | Lines | Regions | Branches |
|---|---|---|---|
| builders | 52.1% (403/773) | 60.6% (781/1288) | 50.0% (8/16) |
| decode | 89.2% (74/83) | 84.8% (95/112) | 100.0% (2/2) |
| FieldNumber | 77.3% (51/66) | 77.6% (66/85) | 75.0% (3/4) |
| wire | 65.5% (457/698) | 67.0% (543/811) | 51.0% (50/98) |
| lib.rs (doctests) | 90.6% (77/85) | 93.8% (166/177) | 66.7% (16/24) |
| **total** | 62.3% (1062/1705) | 66.8% (1651/2473) | 54.9% (79/144) |

Notable gaps: `msg_len_packed_scribe.rs`, `msg_ser_packed_scribe.rs` and `wire_len_calc.rs` are at 0% in both runs. Proptests shift `wire` by a few lines between runs.
