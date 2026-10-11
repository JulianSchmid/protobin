# 07: Document every public item, enforced by `missing_docs`

**What to build:** Every public item has a doc comment written for crate users, following the "Documentation" section of the coding style:

- Simple items get a one-line summary.
- Complicated items give a summary, then a doc-test example, then only the details a user needs.
- Related items are linked with intra-doc links.
- Types that implement part of the wire format link the matching section of the protobuf encoding spec.

The crate enables `missing_docs`, so CI catches any undocumented public item from now on. When this ticket was written, there were 55, mostly in `wire` and `decode`.

**Blocked by:** 04 and 06 (both change or add public items)

**Status:** ready-for-agent

- [ ] `missing_docs` is enabled at crate level and reports nothing
- [ ] Complicated items have a passing doc-test example
- [ ] Wire-format types link the protobuf encoding spec
- [ ] All CI checks pass, including `cargo doc` with `-D warnings`
