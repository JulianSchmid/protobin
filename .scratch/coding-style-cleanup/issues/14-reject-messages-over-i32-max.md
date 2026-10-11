# 14: Return an error for messages over `i32::MAX` bytes instead of overflowing

**What to build:** Encoding a message, sub-message or packed field longer than `i32::MAX` bytes (~2 GiB) returns an error instead of panicking or writing wrong bytes. The coding style's hard rule requires this: values passed to the builders always lead to correct output or a returned error. Protobuf itself caps serialized messages at 2 GiB, so rejecting them is correct.

Today the length phase tracks sizes in `i32` without checks:

- `MsgLenBuilder::add_string_field`, `add_bytes_field`, `add_display_str_field` and `add_debug_str_field` cast lengths with `len as i32`. A length over `i32::MAX` wraps to a negative value.
- Every `cur_len += ...` in `MsgLenBuilder`, `MsgLenPackedScribe` and `MsgLenBuilder::end_len_area` overflows once the total passes `i32::MAX`. Debug builds panic with "attempt to add with overflow". Release builds wrap.
- The serialization phase then writes the wrapped, negative length with `add_var_int32`, a 10-byte varint, so the output is silently wrong.

**Decision (triaged 2026-10-11):** the error surfaces from the length phase's `end()`, and the work lands on the `coding-style-cleanup` branch as part of 0.7.0.

- `MsgScribe::End` for `MsgLenBuilder` becomes `Result<MsgSerBuilder<'a>, EncodeError>`. The serialization phase's `End` stays `&'a [u8]`.
- The length phase never panics or wraps on oversized input. It records the first overflow and keeps going, and `end()` returns it as an error.
- The `add_*`, `start_*` and `end_*` signatures and their `.` chaining are unchanged, so generic `fn ser<S: MsgScribe>(..) -> S::End` code compiles as before. Callers add one `?` between the phases:

  ```rust
  let step2 = ser_person(&person, builder.start(None))?;
  let bytes = ser_person(&person, step2);
  ```

- Every length written to `MsgBuilder.lens` fits in `i32`, so the serialization phase stays infallible.

Rejected: every `add_*` returning a `Result`. Since `MsgScribe` is shared by both phases, that would put a `?` on every line of user code and make the infallible serialization phase return `Result`s.

**Left to the implementer:**

- **How the length phase detects the overflow:** checked `i32` arithmetic, or a wider accumulator (`i64`) checked whenever a sub-message or packed field ends and in `end()`. `lens` can keep its `i32` entries either way. Keep the per-field hot path cheap. If the choice is performance-motivated, back it with numbers.
- **The error type:** a new `EncodeError` in `builders/encode_error.rs`, following the coding style's error rules (`Display`, `core::error::Error`, derives, values in field-shaped variants, and `debug` / `clone_eq_hash` / `fmt` / `source` tests). It should carry what a user needs to find the culprit, e.g. the field number of the innermost sub-message or packed field that overflowed (none for the top-level message) and the length that was reached.

**Testing hint:** the length phase never touches the data, so tests can reach 2 GiB without allocating it by adding the same 1 MiB slice 2048 times.

**Blocked by:** None (can start immediately). Do this before 13, so that 13 reserves a total length that is already known to be valid.

**Status:** ready-for-agent

- [ ] `MsgLenBuilder`'s `MsgScribe::End` is `Result<MsgSerBuilder<'a>, EncodeError>`, and `MsgScribe::End`'s docs describe both phases
- [ ] No length-phase method panics or wraps for messages, sub-messages or packed fields longer than `i32::MAX` bytes; the length phase's `end()` returns the error instead
- [ ] Each overflow path (string, bytes, `Display`, `Debug`, packed elements, nested sub-message prefixes) has a test checking the exact error
- [ ] `EncodeError` follows the coding style's error rules, with its own file and tests
- [ ] The README examples, `examples/` and the "Two-phase encoding" section of `CLAUDE.md` use the new `end()` signature
- [ ] `Changelog.md` under `## 0.7.0` records the `End` type change under "Changed" (marked **Breaking**) and the overflow under "Fixed"
- [ ] All CI checks pass
