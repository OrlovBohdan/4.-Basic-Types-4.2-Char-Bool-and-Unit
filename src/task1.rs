use std::mem::size_of_val;
#[test]
/*// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3);

    println!("Success!");
} */
// Make it work

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}

/*Проблема в твоём коде заключается в предположении о размере типа char в Rust.
В Rust символ (char) представляет собой значение Unicode, которое всегда занимает 4 байта, независимо от того, какой это символ.

Функция size_of_val всегда вернёт 4 для типа char, даже для символов,
которые в других кодировках могут занимать меньше или больше места (например, в UTF-8).*/