use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Pet {
  name: String,
  age: u8,
  owner: Owner,
}

#[derive(Serialize, Deserialize, Debug)]
struct Owner {
  name: String,
}

fn main() {
  let pet = Pet {
    name: String::from("Alice"),
    age: 30,
    owner: Owner { name: "Bob".to_string() },
  };

  let serialized = serde_json::to_string(&pet).unwrap();
  println!("Serialized: {}", serialized);

  let deserialized: Pet = serde_json::from_str(&serialized).unwrap();
  println!("Deserialized: {:?}", deserialized);
}