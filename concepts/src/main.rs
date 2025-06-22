mod method;

fn main() {
    // addition
    let sum = 5 + 20;
    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {y}");

    let a = [1, 2, 3, 4, 5, 6];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // print_labeled_measurement(2, 'cm');
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    let number = 3;
    if number != 0 {
        println!("Number was three")
    }
    if number < 5 {
        println!("Condition was true")
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn structs() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None => None,
        Some(i)=>Some(i+1),
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn mains() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The Area of the rectangle is {} square pixels.",
        area(&rect1)
    )
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
