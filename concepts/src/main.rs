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

    let tup = (500,6.4,1);
    let (x,y,z) = tup;

    println!("The value of y is {y}");

    let a = [1,2,3,4,5,6];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    // print_labeled_measurement(2, 'cm');
}


fn print_labeled_measurement(value:i32,unit_label:char){
    println!("The measurement is: {value}{unit_label}");
    let number = 3;
    if number !=0 {
        println!("Number was three")
    }
    if number < 5 {
        println!("Condition was true")
    }else{
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");
}

