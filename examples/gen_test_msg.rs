use protobin::builders::{MsgBuilder, MsgScribe};
use std::{fs::File, io::Write, process::ExitCode};

const PROTO_FILE: &'static str = "
syntax = \"proto3\";

package dummy;

message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
";

struct Person {
    name: String,
    id: i32,
    email: String,
}

// Sometimes you want to define a custom serialisation function
// without coupling it to the type directly. In this case you can
// define a function that handles the serialisation and use it
// with the builder.
fn ser_person<S: MsgScribe>(p: &Person, mut s: S) -> S::End {
    s.add_string(1.try_into().unwrap(), &p.name);
    s.add_int32(2.try_into().unwrap(), p.id);
    s.add_string(3.try_into().unwrap(), &p.email);
    s.end()
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
    let person = Person {
        name: "Greg".to_owned(),
        id: 1234,
        email: "greg@greg.net".to_owned(),
    };

    // generate message (needs two calls)
    // the first step calculates the internal lengths
    let step2 = ser_person(&person, builder.start(Some(0)));
    // the second step serializes the message
    let result = ser_person(&person, step2);

    // write to file
    binary_file.write_all(&result).unwrap();
    drop(binary_file);
    
    ExitCode::SUCCESS
}