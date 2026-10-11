# 03: Use `core::` paths wherever `core` has the item

**What to build:** Library and test code use `core::` paths (`core::fmt`, `core::ops::RangeInclusive`, ...) for every item that exists in `core`, as the coding style requires. `std::` remains only for items `core` lacks. This is a pure refactor with no change in behaviour.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] No `std::` path remains for an item that exists in `core`, in library code or tests
- [x] Every remaining `std::` use is for an item that `core` lacks
- [x] Behaviour is unchanged and all CI checks pass

## Comments

- The remaining `std::` uses are `std::collections::hash_map::DefaultHasher` in the error tests, plus `std::{env, fs, io, process}` in the examples. `core` has none of these.
- Going beyond library and test code: `examples/gen_complex_msg.rs` imported the deprecated `std::{i32, i64, u32}` modules. These modules also exist in `core`, so I dropped the imports, and the constants now resolve to the primitives' associated constants. The generated `.proto` and `.bin` files are byte-identical.
- No Changelog entry: `std::fmt` re-exports `core::fmt`, so the public signatures name the same types.
- Noticed in review, out of scope, no ticket yet: `FmtByteCounter` is defined twice, once each in `msg_len_builder.rs` and `msg_ser_builder.rs`.
