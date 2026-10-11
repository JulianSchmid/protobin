# 08: Builder tests follow the test rules

**What to build:** The `builders` tests follow the "Tests" section of the coding style:

- The test module is called `mod test`.
- There is one test function per method, named after the method, with each case in its own `{ ... }` block with a comment. Exceptions are allowed where splitting genuinely reads better.
- Errors are checked with an exact `assert_eq!`.
- Property tests are preferred.

Every branch is covered, including each API-misuse panic, checked with `#[should_panic]` and the expected message. Behaviour is unchanged.

**Blocked by:** 02 (coverage command and baseline)

**Status:** ready-for-agent

- [ ] Every test module in `builders` is called `mod test`, with one test function per method as described above
- [ ] Each API-misuse panic has a `#[should_panic]` test with its expected message
- [ ] The coverage command shows every branch of `builders` covered, or this ticket's `## Comments` lists each uncovered line with the reason
- [ ] All CI checks pass
