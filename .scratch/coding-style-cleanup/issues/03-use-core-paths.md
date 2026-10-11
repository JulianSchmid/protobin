# 03: Use `core::` paths wherever `core` has the item

**What to build:** Library and test code use `core::` paths (`core::fmt`, `core::ops::RangeInclusive`, ...) for every item that exists in `core`, as the coding style requires. `std::` remains only for items `core` lacks. This is a pure refactor with no change in behaviour.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] No `std::` path remains for an item that exists in `core`, in library code or tests
- [ ] Every remaining `std::` use is for an item that `core` lacks
- [ ] Behaviour is unchanged and all CI checks pass
