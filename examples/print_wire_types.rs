use std::io::Read;

use protobin::decode::MsgDecoder;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: Unexpected number of command line arguments.");
        eprintln!();
        eprintln!("Usage: print_wire_types <file>");
        return;
    }

    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    drop(file);

    let mut msg_decoder = MsgDecoder::new(&data);
    while let Some(re) = msg_decoder.next() {
        let value = re.unwrap();
        println!("{}: {:?}", value.field_number.value(), value.value);
    }
}
