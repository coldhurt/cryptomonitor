use alloy::primitives::{Log, address};

fn main (){
  // let log =  Log{
  //   address: address!("0x741100A27c416FB7794b14496518B2463D4086a3"),
  //   data: ()
  // };
  // println!("{}", serde_json::to_string_pretty(&log).unwrap());

  let mut vec1: Vec<u32> = Vec::new();

  vec1.push(32);
  vec1.push(32);
  vec1.push(32);
  vec1.push(32);
  vec1.push(32);

  println!("capacity: {:?} len: {}", vec1.capacity(), vec1.len());
}