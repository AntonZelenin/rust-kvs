use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

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

    let mut f = OpenOptions::new().read(true).write(true).open("./examples/t.txt").unwrap();
    f.write(&serde_json::to_string(&a).unwrap().as_bytes());
    f.seek(SeekFrom::Start(0));

    let mut b = String::new();
    f.read_to_string(&mut b);

    let c: Move = serde_json::from_str(&b).unwrap();
    println!("{:?}", c);
}
