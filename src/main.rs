#[allow(unused_variables)]
fn main() {
    define_intialization();
    define_mutability();
    define_scope();
    define_shadowing();
    define_destructuring();
    define_destructuring_assignemnts();
}

fn define_intialization() {
    let a: i32 = 10;
    let _b: i32;
    println!("Value of a is {}", a)
}

fn define_mutability() {
    let mut a: i32 = 10;
    a += 20;

    assert_eq!(a, 30);
    println!("Success define_mutbility()")
}

fn define_scope() {
    let a: i32 = 20;
    let b: i32 = 40;
    // The "Inner" and "Outer" are just for naming conventions
    {
        println!("Inner Scope: value of a is {} and b is {}", a, b);
    }
    println!("Outer Scope: value of a is {} and b is {}", a, b);
}

fn define_shadowing() {
    let a: i32 = 20;
    {
        // shadowing
        let a = 50;
        assert_eq!(a, 50);
        println!("Inner scope a {}", a);
    }
    println!("Before shadowing {}", a);
    let a = 42;
    println!("After shadowing {}", a);

    let y = 4;
    let y = "I can also be bound to text";

    println!("y is {}", y);
}

fn define_destructuring() {
    let (mut a, b) = (1, 2);
    a += 3;

    assert_eq!(a, 4);
    assert_eq!(b, 2);
    println!("Success define_destructuring()")
}

fn define_destructuring_assignemnts() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);
    println!("Success define_destructuring_assignemnts()")
}
