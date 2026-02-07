# protobin

[![Crates.io](https://img.shields.io/crates/v/protobin.svg)](https://crates.io/crates/protobin)
[![docs.rs](https://img.shields.io/docsrs/protobin)](https://docs.rs/protobin)
[![License](https://img.shields.io/crates/l/protobin.svg)](https://crates.io/crates/protobin)

Low-level Rust primitives to encode and decode [Protocol Buffer](https://protobuf.dev/) binary messages -- without code generation and with minimal allocations.

## Motivation

Most protobuf libraries in Rust rely on code generation from `.proto` files or require allocating intermediate data structures. `protobin` takes a different approach: it gives you direct access to the wire format through low-level primitives, letting you encode and decode protobuf messages with full control and minimal overhead.

Encoding still requires some allocations (e.g. the internal buffers used by `MsgBuilder`), but these buffers are reusable across messages, so in steady state no new allocations are needed.

This is useful when you:

- Need to write protobuf data on the wire without a `.proto` schema or code generation step
- Want to minimize allocations (the `MsgBuilder` buffers are reusable across messages)
- Need to inspect or decode arbitrary protobuf binary data
- Are building tooling that operates on the wire format directly

## Key Features

- **No code generation** -- encode and decode directly using field numbers and wire types
- **Minimal allocations** -- `MsgBuilder` buffers are reusable, so encoding many messages amortizes to zero additional allocations in steady state
- **Two-phase encoding** -- lengths are pre-calculated before serialization so no data shifting is needed
- **Zero-copy decoding** -- `MsgDecoder` iterates over records by borrowing the input data
- **All protobuf scalar types** -- int32, int64, uint32, uint64, sint32, sint64, fixed32, sfixed32, fixed64, sfixed64, float, double, bool, string, bytes, enums
- **Nested messages and packed repeated fields**
- **No dependencies** (only `proptest` as a dev-dependency for testing)

## How Two-Phase Encoding Works

Protobuf's wire format prefixes every submessage and length-delimited field with its byte length, encoded as a varint. Because a varint's own size depends on the value, you can't know how many bytes the length prefix will occupy until you know the content length -- and shifting serialized data afterwards would be expensive.

`protobin` solves this with a two-phase approach:

1. **Phase 1 (length calculation)** -- walk your data and compute all the nested lengths
2. **Phase 2 (serialization)** -- serialize the data using the pre-calculated lengths

Both phases use the same serialization function through the [`MsgScribe`](builders::MsgScribe) trait, so you write your serialization logic only once.

## Examples

### Encoding a Simple Message

Given this protobuf schema:

```proto
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```

You can encode it like this:

```rust
use protobin::builders::{MsgBuilder, MsgScribe};

struct Person {
    name: String,
    id: i32,
    email: String,
}

/// A single serialization function that works for both phases
/// thanks to the `MsgScribe` trait.
fn ser_person<S: MsgScribe>(p: &Person, mut s: S) -> S::End {
    s.add_string(1.try_into().unwrap(), &p.name);
    s.add_int32(2.try_into().unwrap(), p.id);
    s.add_string(3.try_into().unwrap(), &p.email);
    s.end()
}

fn main() {
    // Create a reusable builder (reuse it across messages to avoid allocations)
    let mut builder = MsgBuilder::new();

    let person = Person {
        name: "Alice".to_owned(),
        id: 42,
        email: "alice@example.com".to_owned(),
    };

    // Phase 1: calculate lengths
    let step2 = ser_person(&person, builder.start(None));
    // Phase 2: serialize into bytes
    let bytes = ser_person(&person, step2);

    assert!(!bytes.is_empty());
}
```

### Encoding Nested Messages

Nested messages use `start_msg` / `end_msg` pairs:

```rust
use protobin::builders::{MsgBuilder, MsgScribe};

// message Inner {
//   uint64 value = 1;
//   string label = 2;
// }
// message Outer {
//   string name = 1;
//   repeated Inner items = 2;
// }

fn ser_inner<S: MsgScribe>(value: u64, label: &str, mut s: S) -> S {
    s.add_uint64(1.try_into().unwrap(), value);
    s.add_string(2.try_into().unwrap(), label);
    s
}

fn ser_outer<S: MsgScribe>(name: &str, items: &[(u64, &str)], mut s: S) -> S {
    s.add_string(1.try_into().unwrap(), name);
    for &(value, label) in items {
        s.start_msg(2.try_into().unwrap());
        s = ser_inner(value, label, s);
        s.end_msg(2.try_into().unwrap());
    }
    s
}

fn main() {
    let mut builder = MsgBuilder::new();

    let items: Vec<(u64, &str)> = vec![(1, "first"), (2, "second")];

    // Phase 1 + Phase 2
    let step2 = ser_outer("example", &items, builder.start(None)).end();
    let bytes = ser_outer("example", &items, step2).end();

    assert!(!bytes.is_empty());
}
```

### Decoding a Message

[`MsgDecoder`](decode::MsgDecoder) iterates over the tag-length-value records in a protobuf binary message, borrowing the input data (zero-copy):

```rust
use protobin::decode::MsgDecoder;
use protobin::wire::WireValueRef;
# use protobin::builders::{MsgBuilder, MsgScribe};

fn decode_person(data: &[u8]) {
    let mut decoder = MsgDecoder::new(data);
    while let Some(record) = decoder.next() {
        let record = record.expect("decode error");
        match record.field_number.value() {
            1 => {
                // string name = 1
                if let WireValueRef::Len(len_ref) = &record.value {
                    if let Ok(name) = len_ref.try_as_string() {
                        println!("name: {name}");
                    }
                }
            }
            2 => {
                // int32 id = 2
                if let Ok(id) = record.value.try_as_int32() {
                    println!("id: {id}");
                }
            }
            3 => {
                // string email = 3
                if let WireValueRef::Len(len_ref) = &record.value {
                    if let Ok(email) = len_ref.try_as_string() {
                        println!("email: {email}");
                    }
                }
            }
            other => {
                println!("unknown field {other}: {:?}", record.value);
            }
        }
    }
}
# fn ser<S: MsgScribe>(mut s: S) -> S::End {
#     s.add_string(1.try_into().unwrap(), "Alice");
#     s.add_int32(2.try_into().unwrap(), 42);
#     s.add_string(3.try_into().unwrap(), "alice@example.com");
#     s.end()
# }
# fn main() {
#     let mut builder = MsgBuilder::new();
#     let step2 = ser(builder.start(None));
#     let bytes = ser(step2);
#     decode_person(bytes);
# }
```

### Decoding Nested Messages

For LEN-typed fields that contain embedded messages, use `as_sub_msg()` to get a sub-decoder:

```rust
use protobin::decode::MsgDecoder;
use protobin::wire::WireValueRef;
# use protobin::builders::{MsgBuilder, MsgScribe};

fn decode_outer(data: &[u8]) {
    let mut decoder = MsgDecoder::new(data);
    while let Some(record) = decoder.next() {
        let record = record.expect("decode error");
        match record.field_number.value() {
            1 => {
                if let WireValueRef::Len(len_ref) = &record.value {
                    if let Ok(name) = len_ref.try_as_string() {
                        println!("name: {name}");
                    }
                }
            }
            2 => {
                // Nested message -- decode the sub-message
                if let WireValueRef::Len(len_ref) = &record.value {
                    let mut sub = len_ref.as_sub_msg();
                    while let Some(inner) = sub.next() {
                        let inner = inner.expect("decode error");
                        println!(
                            "  inner field {}: {:?}",
                            inner.field_number.value(),
                            inner.value
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
# fn ser<S: MsgScribe>(mut s: S) -> S::End {
#     s.add_string(1.try_into().unwrap(), "example");
#     s.start_msg(2.try_into().unwrap());
#     s.add_uint64(1.try_into().unwrap(), 100);
#     s.add_string(2.try_into().unwrap(), "first");
#     s.end_msg(2.try_into().unwrap());
#     s.end()
# }
# fn main() {
#     let mut builder = MsgBuilder::new();
#     let step2 = ser(builder.start(None));
#     let bytes = ser(step2);
#     decode_outer(bytes);
# }
```

### Inspecting Unknown Protobuf Data

You can also decode and print arbitrary protobuf binary data without knowing the schema, which is useful for debugging and tooling:

```rust
use protobin::decode::MsgDecoder;
# use protobin::builders::{MsgBuilder, MsgScribe};

fn dump(data: &[u8]) {
    let mut decoder = MsgDecoder::new(data);
    while let Some(record) = decoder.next() {
        let record = record.expect("decode error");
        println!("{}: {:?}", record.field_number.value(), record.value);
    }
}
# fn ser<S: MsgScribe>(mut s: S) -> S::End {
#     s.add_string(1.try_into().unwrap(), "hello");
#     s.add_uint32(2.try_into().unwrap(), 42);
#     s.end()
# }
# fn main() {
#     let mut builder = MsgBuilder::new();
#     let step2 = ser(builder.start(None));
#     let bytes = ser(step2);
#     dump(bytes);
# }
```

## Supported Protobuf Types

| Protobuf Type | Wire Type | Encode Method | Decode Method |
|---|---|---|---|
| `int32` | VarInt | `add_int32` | `try_as_int32` |
| `int64` | VarInt | `add_int64` | `try_as_int64` |
| `uint32` | VarInt | `add_uint32` | `try_as_uint32` |
| `uint64` | VarInt | `add_uint64` | `try_as_uint64` |
| `sint32` | VarInt (zig-zag) | `add_sint32` | `try_as_sint32` |
| `sint64` | VarInt (zig-zag) | `add_sint64` | `try_as_sint64` |
| `bool` | VarInt | `add_bool` | `try_as_bool` |
| `enum` | VarInt | `add_enum` | `try_as_int32` |
| `fixed32` | I32 | `add_fixed32` | `as_fixed32` |
| `sfixed32` | I32 | `add_sfixed32` | `as_sfixed32` |
| `float` | I32 | `add_float` | `as_float` |
| `fixed64` | I64 | `add_fixed64` | `as_fixed64` |
| `sfixed64` | I64 | `add_sfixed64` | `as_sfixed64` |
| `double` | I64 | `add_double` | `as_double` |
| `string` | LEN | `add_string` | `try_as_string` |
| `bytes` | LEN | `add_bytes` | `as_bytes` |
| embedded message | LEN | `start_msg` / `end_msg` | `as_sub_msg` |
| packed repeated | LEN | `start_packed` / `end_packed` | -- |

## References

- [Protocol Buffers Documentation - Encoding](https://protobuf.dev/programming-guides/encoding/)

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.
