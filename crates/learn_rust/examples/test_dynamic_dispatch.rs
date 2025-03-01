trait Animal {
  fn make_sound(&self);
}

struct Dog {
  name: String,
}
struct Cat{
  cat_att: String,
}

impl Animal for Dog {
  fn make_sound(&self) {
      println!("Woof! {}", self.name);
  }
}

impl Animal for Cat {
  fn make_sound(&self) {
      println!("Meow! {}", self.cat_att);
  }
}

fn main() {
  // let animals: Vec<Box<dyn Animal>> = vec![
  //     Box::new(Dog{name:"dog".to_string()}),
  //     Box::new(Cat{cat_att:"cat".to_string()}),
  // ];

  // for animal in animals.iter() {
  //     animal.make_sound(); // Calls the correct implementation at runtime
  // }

  let dog = Dog{name:"dog".to_string()};
  let cat = Cat{cat_att:"cat".to_string()};

  sound(&dog);
  sound(&cat);
}

fn sound(sound_animal: &dyn Animal){
  sound_animal.make_sound();
}