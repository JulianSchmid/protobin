# 11: Verify allocation-free decoding and steady-state encoding

**What to build:** A test that fails as soon as library code allocates on a hot path, enforcing the "Performance & allocation" section of the coding style. An integration test installs a counting global allocator and checks two things:

- Decoding a representative message allocates nothing. The message includes every wire type, nested messages, packed fields and string/bytes access through `WireLenRef`.
- Encoding that message a second time with the same `MsgBuilder` allocates nothing, because the buffers are already large enough. The encoding covers every `MsgScribe` field kind, including sub-messages, packed fields and `add_display_str` / `add_debug_str`.

The test adds no dependency. A global allocator applies to every test in its test binary, and those tests run on parallel threads, so the count must be kept per thread (or the binary must hold a single test). Otherwise the numbers are unreliable.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] An integration test counts allocations with a test-only global allocator, adding no dependency
- [ ] Decoding the representative message, including all nested records, packed values and string/bytes accesses, counts zero allocations
- [ ] Encoding with a reused `MsgBuilder` counts zero allocations on the second message, covering every `MsgScribe` field kind
- [ ] Counts are reliable while other tests run in parallel
- [ ] All CI checks pass
