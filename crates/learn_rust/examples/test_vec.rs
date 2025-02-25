fn main() {

    capacity_len_test();
    // iter_test();
    // chain_test()
}

fn capacity_len_test() {
    let mut vec1: Vec<u32> = Vec::new();

    vec1.push(32);
    vec1.push(32);
    vec1.push(32);
    vec1.push(32);
    vec1.push(32);
    // insert 11 at index 1
    vec1.splice(1..1, vec![11]);
    // replace number 32 at index 2 with 22
    vec1.splice(2..3, vec![22]);

    println!("capacity: {:?} len: {}", vec1.capacity(), vec1.len());
    println!("{:?}", vec1);
}

fn iter_test() {
    let mut numbers = vec![10, 20, 30];

    for (index, num) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, num);
    }

    for num in numbers.iter_mut() {
        *num = *num * *num;
    }

    for (index, num) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, num);
    }
}

fn chain_test() {
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![5, 6, 7, 8];

    for i in vec1.iter().chain(vec2.iter()) {
        println!("{}", i);
    }
}
