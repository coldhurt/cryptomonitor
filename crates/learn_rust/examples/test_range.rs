fn main(){

  let vec1: Vec<i8> = (1..10).map(|x| x*x).collect();

  println!("{:?}", vec1);

  let prime_numbers = (1..100).filter(|x: &i32| {
    let mut is_prime = true;
    if *x <= 1 {
      return false;
    }
    for i in 2..((*x).isqrt() + 1) {
      if x % i == 0 {
        is_prime = false;
        break;
      }
    }
    is_prime
  }).collect::<Vec<i32>>();

  println!("Prime numbers from 1 to 100 {:?}", prime_numbers);
}