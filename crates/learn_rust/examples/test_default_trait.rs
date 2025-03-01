#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    gender: Gender,
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            first_name: "".to_string(),
            last_name: "".to_string(),
            age: 18,
            gender: Gender::Male,
        }
    }
}

fn main() {
    let mut person = Person::default();
    person.first_name = "myths".to_string();
    println!("{:?}", person);

    let alice = Person{
      first_name: "Alice".to_string(),
      last_name: "Wonderland".to_string(),
      gender: Gender::Female,
      ..Person::default()
    };

    println!("{:?}", alice);
}
