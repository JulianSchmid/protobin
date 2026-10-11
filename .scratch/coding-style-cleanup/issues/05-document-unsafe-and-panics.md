# 05: Document `unsafe` blocks and panics, enforced by clippy

**What to build:** Every `unsafe` block explains why it is sound, and every method that can panic says so, as the coding style requires. Both rules become CI-enforced: the crate enables `clippy::undocumented_unsafe_blocks` and `clippy::missing_panics_doc`, and CI's `-D warnings` turns any new violation into a failure.

API misuse is the only allowed panic: unbalanced or mismatched `start_*`/`end_*` calls in the length phase, or a field number in the serialization phase that differs from the length phase. Callers usually reach these through the `MsgScribe` / `PackedScribe` trait methods, so the trait docs must say it too, not only the concrete builders.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] Every `unsafe` block has a `// SAFETY:` comment naming the check that makes it sound
- [ ] Every method that can panic has a `# Panics` doc section, including the `MsgScribe` / `PackedScribe` trait methods whose implementations panic on misuse
- [ ] `clippy::undocumented_unsafe_blocks` and `clippy::missing_panics_doc` are enabled at crate level
- [ ] All CI checks pass
