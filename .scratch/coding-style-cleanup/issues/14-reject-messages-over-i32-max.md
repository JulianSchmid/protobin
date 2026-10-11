# 14: Return an error for messages over `i32::MAX` bytes instead of overflowing

**What to build:** Encoding a message, sub-message or packed field longer than `i32::MAX` bytes (~2 GiB) returns an error instead of panicking or writing wrong bytes. The coding style's hard rule requires this: values passed to the builders always lead to correct output or a returned error. Protobuf itself caps serialized messages at 2 GiB, so rejecting them is correct.

Today the length phase tracks sizes in `i32` without checks:

- `MsgLenBuilder::add_string_field`, `add_bytes_field`, `add_display_str_field` and `add_debug_str_field` cast lengths with `len as i32`. A length over `i32::MAX` wraps to a negative value.
- Every `cur_len += ...` in `MsgLenBuilder`, `MsgLenPackedScribe` and `MsgLenBuilder::end_len_area` overflows once the total passes `i32::MAX`. Debug builds panic with "attempt to add with overflow". Release builds wrap.
- The serialization phase then writes the wrapped, negative length with `add_var_int32`, a 10-byte varint, so the output is silently wrong.

**Open question (needs a decision before implementation):** where does the error surface?

1. **The length phase's `end()` returns a `Result`.** The length builder keeps adding with checked arithmetic and records the first overflow. `MsgScribe::end` for `MsgLenBuilder` then returns `Err` instead of a `MsgSerBuilder`. The `add_*` signatures and chaining stay as they are; only the `End` type and the code handling it change. The serialization phase never sees an invalid length.
2. **Every length-phase `add_*` returns a `Result`**, like `add_display_str` already does. The error surfaces at the offending field, but every `MsgScribe` method signature changes and chaining with `.` breaks.
3. **Widen the internal lengths to `u64`** and check against the 2 GiB limit once, in `end()`. This is the same API change as option 1, with simpler arithmetic but bigger `lens` entries.

Option 1 changes the API least. Whichever option is chosen, the error gets its own type, e.g. `EncodeError::MsgTooLong { len }` in `builders/encode_error.rs`, with the usual `Display`, `Error`, derive and test requirements.

**Testing hint:** the length phase never touches the data, so tests can reach 2 GiB without allocating it by adding the same 1 MiB slice 2048 times.

**Blocked by:** None (can start immediately). Coordinate with 13, which reserves the length phase's total before serialization.

**Status:** needs-triage

- [ ] The open question above is decided and recorded in this ticket
- [ ] No length-phase method panics or wraps for messages, sub-messages or packed fields longer than `i32::MAX` bytes; encoding them returns the new error
- [ ] Each overflow path (string, bytes, `Display`, `Debug`, packed elements, nested sub-message prefixes) has a test checking the exact error
- [ ] `Changelog.md` under `## 0.7.0` records the API change under "Changed" (marked **Breaking**) and the overflow under "Fixed"
- [ ] All CI checks pass
