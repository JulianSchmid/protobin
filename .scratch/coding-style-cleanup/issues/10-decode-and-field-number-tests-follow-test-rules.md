# 10: Decode and `FieldNumber` tests follow the test rules

**What to build:** The tests for `MsgDecoder`, `MsgRecordRef` and `FieldNumber` follow the "Tests" section of the coding style:

- The test module is called `mod test`.
- There is one test function per method, named after the method, with each case in its own `{ ... }` block with a comment.
- Errors are checked with an exact `assert_eq!`.
- Property tests are preferred.

Every branch is covered, including every `DecodeError` path the decoder can produce and every `FieldNumber` range check. Behaviour is unchanged.

**Blocked by:** 02 (coverage command and baseline), 06 (new `DecodeError` values)

**Status:** ready-for-agent

- [ ] `decode` and `FieldNumber` have `mod test` test modules following the per-method structure
- [ ] Every `DecodeError` path and every `FieldNumber` range check is tested with an exact `assert_eq!`
- [ ] The coverage command shows every branch of `decode` and `FieldNumber` covered, or this ticket's `## Comments` lists each uncovered line with the reason
- [ ] All CI checks pass
