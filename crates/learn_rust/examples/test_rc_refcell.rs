use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_value = Rc::new(RefCell::new(42));

    // 克隆 Rc 引用计数增加
    let a = Rc::clone(&shared_value);
    let b = Rc::clone(&shared_value);

    // 修改数据
    *a.borrow_mut() += 10;

    {
        let c = Rc::clone(&shared_value);
        println!("c: {}", c.borrow());
        *c.borrow_mut() += 30;
    }

    println!("a: {}", a.borrow()); // 输出：82
    println!("b: {}", b.borrow()); // 输出：82

}
