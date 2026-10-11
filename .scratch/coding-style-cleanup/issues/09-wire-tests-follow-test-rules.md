# 09: Wire tests follow the test rules

**What to build:** The `wire` tests follow the "Tests" section of the coding style:

- The test module is called `mod test`.
- There is one test function per method, named after the method, with each case in its own `{ ... }` block with a comment.
- Round-trip tests of an encode/decode pair are named `from_and_as_<type>`.
- Every error is checked with an exact `assert_eq!` on the expected error value.
- Property tests are preferred.

Every branch is covered, especially every error path. Behaviour is unchanged.

**Blocked by:** 02 (coverage command and baseline), 04 (renamed `try_as_*` methods), 06 (new `DecodeError` values)

**Status:** ready-for-agent

- [ ] Every test module in `wire` is called `mod test`, follows the per-method structure and uses the `from_and_as_<type>` naming for round-trip tests
- [ ] Every error path is tested with an exact `assert_eq!`
- [ ] The coverage command shows every branch of `wire` covered, or this ticket's `## Comments` lists each uncovered line with the reason
- [ ] All CI checks pass
