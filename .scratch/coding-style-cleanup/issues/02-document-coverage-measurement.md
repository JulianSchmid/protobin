# 02: Document how to measure test coverage

**What to build:** A contributor or agent can measure line and branch coverage with one documented command, `cargo llvm-cov --html`. `CLAUDE.md` lists it with its one-time setup. The coding style requires every branch and every error path to be tested, and the test-cleanup tickets (08–10) use this command to check that, comparing against a baseline recorded here.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] The "Commands" section of `CLAUDE.md` lists the coverage command and its one-time setup (installing `cargo-llvm-cov` and the `llvm-tools-preview` component)
- [ ] The command runs cleanly on the current code and produces an HTML report
- [ ] No dependency is added to the crate manifest
- [ ] Baseline coverage per module (`builders`, `wire`, `decode`, `FieldNumber`) is recorded under a `## Comments` heading in this ticket file
