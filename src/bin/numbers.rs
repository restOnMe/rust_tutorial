use std::ops::{Range, RangeInclusive};

// INTEGERS
fn main() {
    number_1();
    number_2();
    number_3();
    number_4();
    number_5();
    number_6();
    number_7();
    number_8();
    number_9();
    number_10();
    number_11();
}

fn number_1() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;
    let z = 10;
    println!("number_1: value of z: {}", &z);
    println!("Success!");
}

fn number_2() {
    let v: u16 = 38u8 as u16;
    println!("number_2: value of v: {}", &v);
    println!("Success!");
}

fn number_3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("number_3: succesful");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn number_4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("number_4: successful!")
}

fn number_5() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();

    println!(" number_5: v1= {},v2= {}", v1, v2);
}

fn number_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("number_6: successful! v= {}", v);
}

fn number_7() {
    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.01_f64; // float64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("number_7: succesful! type_of(x): {}", type_of(&x));
}

fn number_8() {
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
    assert!(0.1f32 + 0.2f32 == 0.3f32);
    println!("number_8: successful! ")
}

fn number_9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }
    println!("{}", &sum);
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }

    println!("number_9: successful")
}

fn number_10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("number_10: succesful!")
}
fn number_11() {
    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
