# 13: Reserve the exact message size before serialization

**What to build:** After the length phase, the exact size of the encoded message is known. The serialization phase reserves that much capacity in the output buffer before writing, so encoding with a fresh `MsgBuilder` (or a message bigger than any before) grows the buffer once instead of repeatedly as bytes are appended. There is no API change.

**Blocked by:** 11 (allocation-counting test), 12 (benchmarks for before/after numbers)

**Status:** ready-for-agent

- [ ] The serialization phase reserves the exact remaining capacity for the whole message before writing its first byte
- [ ] The allocation-counting test checks that a fresh `MsgBuilder` grows its output buffer exactly once
- [ ] The commit message includes before/after numbers from the fresh-builder and reused-builder encode benchmarks
- [ ] `Changelog.md` under `## 0.7.0` records the change under "Changed"
- [ ] All CI checks pass
