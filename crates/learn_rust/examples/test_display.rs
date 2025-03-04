use std::fmt::{Display, Formatter, Result};
use std::u8::MAX;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "RGB ({},{},{}) 0x{:X}{:X}{:X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

fn main() {
    let color = Color {
        red: 122,
        green: 17,
        blue: 40,
    };
    println!("{}", color);

    println!("Max of u8 is {}", MAX);
}
