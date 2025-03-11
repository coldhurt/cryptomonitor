enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn traversal_list(list: List) {
    match list {
        Cons(x, y) => {
            println!("{}", x);
            traversal_list(*y);
        },
        Nil => {}
    }
}


trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    // Use box to calcute size of circular referenced objects, because Box is a smart pointer, it's size is determinted.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Linked List created!");
    traversal_list(list);

    // Dynamic dispatch
    let dog: Box<dyn Animal> = Box::new(Dog);
    let cat: Box<dyn Animal> = Box::new(Cat);

    dog.speak();
    cat.speak();

    {
        let b = Box::new(10);
        println!("b = {}", b);
    } // b is released

    let a = Box::new(5);
    let b = a; // Transfer ownership of a to b

    // Cannot use a any more
    // println!("{}", a);
    println!("b is {}", b);

}
