use alloy::primitives::U256;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[allow(dead_code)]
fn string_to_number() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

#[allow(dead_code)]
// Decimal, binary, hexadecimal, octal conversion
fn decimal_conversion() {
    let num = 255;

    println!("decimal {}", num);
    println!("binary: {:b}", num); // binary
    println!("octal: {:o}", num); // octal
    println!("hexadecimal: {:x}", num); // lowercase hexadecimal
    println!("hexadecimal: {:X}", num); // uppercase hexadecimal

    let binary = "1101"; // binary (13)
    let octal = "755"; // octal (493)
    let hex = "FF"; // hexadecimal (255)

    let decimal_binary = i32::from_str_radix(binary, 2).unwrap();
    let decimal_octal = i32::from_str_radix(octal, 8).unwrap();
    let decimal_hex = i32::from_str_radix(hex, 16).unwrap();

    println!("binary 1101 to decimal: {}", decimal_binary);
    println!("octal 755 to decimal: {}", decimal_octal);
    println!("hexadecimal FF decimal: {}", decimal_hex);
}

fn alloy_conversion() {
    let hex_str = "ff";
    let num = U256::from_str_radix(hex_str, 16).unwrap();

    println!("alloy hex {} to decimal: {}", hex_str, num);
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    decimal_conversion();

    alloy_conversion();
}
