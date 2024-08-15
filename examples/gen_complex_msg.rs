use protobin::builders::{MsgBuilder, MsgScribe};
use std::{fs::File, i32, i64, io::Write, process::ExitCode, u32};

const PROTO_FILE: &'static str = "
syntax = \"proto3\";

package dummy;

message Root {

  message Level2 {
    uint64 a = 1;
    string b = 2;
  }

  message Level1 {
    sint32 a = 1;
    int32 b = 2;
    repeated Level2 c = 3;
    int32 d = 4;
  }

  string name = 1;
  optional int32 id = 2;
  string email = 3;
  repeated Level1 level1 = 4;

  uint32 uint32_min = 5;
  uint32 uint32_zero = 6;
  uint32 uint32_max = 7;

  int32 int32_min = 8;
  int32 int32_zero = 9;
  int32 int32_max = 10;

  sint32 sint32_min = 11;
  sint32 sint32_zero = 12;
  sint32 sint32_max = 13;

  uint64 uint64_min = 14;
  uint64 uint64_zero = 15;
  uint64 uint64_max = 16;

  int64 int64_min = 17;
  int64 int64_zero = 18;
  int64 int64_max = 19;

  sint64 sint64_min = 20;
  sint64 sint64_zero = 21;
  sint64 sint64_max = 22;
}
";

struct Level2 {
    a: u64,
    b: String,
}

struct Level1 {
    a: i32,
    b: i32,
    c: Vec<Level2>,
    d: i32,
}

struct Root {
    name: String,
    id: i32,
    email: String,
    level1: Vec<Level1>,

    uint32_min: u32,
    uint32_zero: u32,
    uint32_max: u32,

    int32_min: i32,
    int32_zero: i32,
    int32_max: i32,

    sint32_min: i32,
    sint32_zero: i32,
    sint32_max: i32,

    uint64_min: u64,
    uint64_zero: u64,
    uint64_max: u64,

    int64_min: i64,
    int64_zero: i64,
    int64_max: i64,

    sint64_min: i64,
    sint64_zero: i64,
    sint64_max: i64,
}

// Sometimes you want to define a custom serialisation function
// without coupling it to the type directly. In this case you can
// define a function that handles the serialisation and use it
// with the builder.

fn ser_level2<S: MsgScribe>(p: &Level2, mut s: S) -> S {
    s.add_uint64(1.try_into().unwrap(), p.a);
    s.add_string(2.try_into().unwrap(), &p.b);
    s
}

fn ser_level1<S: MsgScribe>(p: &Level1, mut s: S) -> S {
    s.add_int32(1.try_into().unwrap(), p.a);
    s.add_int32(2.try_into().unwrap(), p.b);
    for l in &p.c {
        s.start_msg(3.try_into().unwrap());
        s = ser_level2(&l, s);
        s.end_msg(3.try_into().unwrap());
    }
    s.add_sint32(4.try_into().unwrap(), p.d);
    s
}

fn ser_root<S: MsgScribe>(p: &Root, mut s: S) -> S {
    s.add_string(1.try_into().unwrap(), &p.name);
    s.add_int32(2.try_into().unwrap(), p.id);
    s.add_string(3.try_into().unwrap(), &p.email);

    for l in &p.level1 {
        s.start_msg(4.try_into().unwrap());
        s = ser_level1(&l, s);
        s.end_msg(4.try_into().unwrap());
    }

    s.add_uint32(5.try_into().unwrap(), p.uint32_min);
    s.add_uint32(6.try_into().unwrap(), p.uint32_zero);
    s.add_uint32(7.try_into().unwrap(), p.uint32_max);

    s.add_int32(8.try_into().unwrap(), p.int32_min);
    s.add_int32(9.try_into().unwrap(), p.int32_zero);
    s.add_int32(10.try_into().unwrap(), p.int32_max);

    s.add_sint32(11.try_into().unwrap(), p.sint32_min);
    s.add_sint32(12.try_into().unwrap(), p.sint32_zero);
    s.add_sint32(13.try_into().unwrap(), p.sint32_max);

    s.add_uint64(14.try_into().unwrap(), p.uint64_min);
    s.add_uint64(15.try_into().unwrap(), p.uint64_zero);
    s.add_uint64(16.try_into().unwrap(), p.uint64_max);

    s.add_int64(17.try_into().unwrap(), p.int64_min);
    s.add_int64(18.try_into().unwrap(), p.int64_zero);
    s.add_int64(19.try_into().unwrap(), p.int64_max);

    s.add_sint64(20.try_into().unwrap(), p.sint64_min);
    s.add_sint64(21.try_into().unwrap(), p.sint64_zero);
    s.add_sint64(22.try_into().unwrap(), p.sint64_max);

    s
}

fn main() -> ExitCode {
    if std::env::args().len() != 3 {
        eprintln!("Expected 2 arguments: <result_proto_file> <result_msg_binary_file>");
        return ExitCode::FAILURE;
    }

    // create file handles
    let mut proto_file = File::create(std::env::args().nth(1).unwrap()).unwrap();
    let mut binary_file = File::create(std::env::args().nth(2).unwrap()).unwrap();

    // setup a message build (should be re-used, it contains Vec's)
    let mut builder = MsgBuilder::new();

    // generate proto file with the message definition
    proto_file.write_all(PROTO_FILE.as_bytes()).unwrap();
    drop(proto_file);

    // setup the data that will be serialized
    let root = Root {
        name: "Greg".to_owned(),
        id: 1234,
        email: "greg@greg.net".to_owned(),
        level1: vec![
            Level1 {
                a: 1,
                b: 2,
                c: vec![],
                d: 4,
            },
            Level1 {
                a: 1,
                b: 2,
                c: vec![
                    Level2 {
                        a: 1,
                        b: "bstring".to_owned(),
                    },
                    Level2 {
                        a: 3,
                        b: "2ndbstring".to_owned(),
                    },
                ],
                d: 4,
            },
        ],
        uint32_min: u32::MIN,
        uint32_zero: 0,
        uint32_max: u32::MAX,
        int32_min: i32::MIN,
        int32_zero: 0,
        int32_max: i32::MAX,
        sint32_min: i32::MIN,
        sint32_zero: 0,
        sint32_max: i32::MAX,
        uint64_min: u64::MIN,
        uint64_zero: 0,
        uint64_max: u64::MAX,
        int64_min: i64::MIN,
        int64_zero: 0,
        int64_max: i64::MAX,
        sint64_min: i64::MIN,
        sint64_zero: 0,
        sint64_max: i64::MAX,
    };

    // generate message (needs two calls)
    // the first step calculates the internal lengths
    let step2 = ser_root(&root, builder.start(Some(0))).end();
    // the second step serializes the message
    let result = ser_root(&root, step2).end();

    // write to file
    binary_file.write_all(&result).unwrap();
    drop(binary_file);

    ExitCode::SUCCESS
}
