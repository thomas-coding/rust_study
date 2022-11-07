const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn compound_type() {
    let dup: (i32, f64, u8) = (500, 6.4, 1);
    let x = dup.1;
    println!("compound_type, The value of x is: {x}");

    let a = [1, 2, 3, 4, 5];
    let y = a[4];
    println!("compound_type, The value of y is: {y}");
}

fn branch() {
    // test if
    let number = 100;
    if number < 20 {
        println!("branch number is too small");
    } else if number < 50 {
        println!("branch number is small");
    } else {
        println!("branch number is big");
    }

    // loop
    let mut number = 5;
    loop {
        println!("branch loop number:{number}");
        number -= 1;
        if number == 0 {
            break;
        }
    }

    // while
    let mut number = 5;
    while number != 0 {
        println!("branch while number:{number}");
        number -= 1;
    }

    // for 0,1,2,3
    for number in 0..4 {
        println!("branch for {number}");
    }
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    let x = x + 10;
    println!("The value of x is: {x}");

    let heart_eyed_cat = '😻';
    println!("cat: {heart_eyed_cat}");
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");

    compound_type();

    let value = add(10, 20);
    println!("The add value is: {value}");

    branch();
}
