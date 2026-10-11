# 04: Give fallible `WireValueRef` conversions `try_` names

**What to build:** Users convert a `WireValueRef` to a fixed-size type with `try_as_fixed32`, `try_as_sfixed32`, `try_as_float`, `try_as_fixed64`, `try_as_sfixed64` and `try_as_double`. That matches the existing varint conversions (`try_as_uint32`, ...) and the rule that methods able to fail carry a `try_` prefix. The old `as_*` names keep compiling but forward to the new methods and are deprecated since 0.7.0, with a note naming the replacement (e.g. "Use `try_as_fixed32` instead"). They stay until 1.0. The infallible `as_*` methods on `WireI32` / `WireI64` are unchanged.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] The six `try_as_*` methods exist with the old behaviour and docs
- [ ] The six old names forward to them and are marked deprecated since `0.7.0`, with a note naming the replacement
- [ ] The crate's own code, README, examples and tests use the new names, so the build emits no deprecation warnings
- [ ] A test calls each deprecated name (allowing the deprecation lint) and checks that it still matches its replacement
- [ ] `Changelog.md` under `## 0.7.0` lists the new methods under "Added" and the old names under "Deprecated"
- [ ] All CI checks pass
