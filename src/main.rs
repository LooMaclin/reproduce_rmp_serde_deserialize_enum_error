extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct  A {
    b: B,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum B {
    First,
    Second
}

fn main() {
    println!("Hello, world!");
    let a = A {
        b: B::First,
    };
    let a = rmp_serde::to_vec(&a).unwrap();
    let a: A = rmp_serde::from_slice(&a).unwrap();
    println!("a: {:?}", a);
}
