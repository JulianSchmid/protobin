use protobin::WireVarInt;

fn main() {
    let values = [2, 1, 0, -1, -2];
    for value in values {
        let v = WireVarInt::from_sint32(value);
        println!("{value} => {}", v.raw());
    }
}
