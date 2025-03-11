macro_rules! repeat {
    ($num:expr, $str:expr) => {
        for _ in 0..$num {
            println!($str);
        }
    };
}


fn main() {
    repeat!(3, "Hello");
}
