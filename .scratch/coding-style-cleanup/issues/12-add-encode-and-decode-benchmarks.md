# 12: Add criterion benchmarks for encoding and decoding

**What to build:** `cargo bench` runs criterion benchmarks for the hot paths, so performance-motivated changes can show before/after numbers, as the coding style requires. The benchmarks cover a representative message with nested messages, packed fields, strings and bytes, for example the one the `gen_complex_msg` example builds. They measure:

- encoding with a reused `MsgBuilder` (the steady state),
- encoding with a fresh `MsgBuilder` (first use, including buffer growth),
- decoding every record, including nested messages and packed fields.

`criterion` becomes the second dev-dependency, and the crate still has no runtime dependencies. Dev-dependencies don't affect the MSRV build. CI compiles the benchmarks so they can't rot.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] `criterion` is a dev-dependency, and no runtime dependency is added
- [ ] `cargo bench` runs the three benchmarks above on a representative message
- [ ] The "No runtime dependencies" constraint in `CLAUDE.md` names both dev-dependencies, and the "Commands" section lists `cargo bench` and the CI benchmark check
- [ ] CI compiles the benchmarks
- [ ] Baseline numbers for all three benchmarks are recorded under a `## Comments` heading in this ticket file
- [ ] All CI checks pass
