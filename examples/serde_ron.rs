use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

#[derive(Serialize, Deserialize, Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    axis: Axis,
    value: i32,
}

fn main() {
    let a = Move { axis: Axis::X, value: 3 };
    let mut buf: Vec<u8> = Vec::new();

    // buf.extend_from_slice(serde_json::to_string(&a).unwrap().as_bytes());
    buf.extend_from_slice(ser::to_string(&a).unwrap().as_bytes());
    
    let b = String::from_utf8(buf).unwrap();

    let c: Move = de::from_str(&b).unwrap();
    println!("{:?}", c);
}
