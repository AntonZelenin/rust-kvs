use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use rand::Rng;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use bson;

#[derive(Serialize, Deserialize, Debug, FromPrimitive)]
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
    let structs = generate_structs(1000);

    let mut f = OpenOptions::new().read(true).write(true).open("./examples/1000.txt").unwrap();
    structs
        .iter()
        .for_each(|x| {
            if let bson::Bson::Document(document) = bson::to_bson(x).unwrap() {
                bson::encode_document(&mut f, &document).unwrap();
            } else {
                println!("Error converting the BSON object into a document");
            }
        });
    f.seek(SeekFrom::Start(0)).unwrap();

    while let Ok(decoded) = bson::decode_document(&mut f) {
        println!("{:?}", decoded);
    }
}

fn generate_structs(num: usize) -> Vec<Move> {
    let mut rng = rand::thread_rng();
    (0..num)
        .map(|_| {
            Move {
                axis: FromPrimitive::from_i32(rng.gen_range(0, 2)).unwrap(),
                value: rng.gen_range(0, 10),
            }
        })
        .collect()
}
