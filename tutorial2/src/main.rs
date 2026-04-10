fn main() {
    //Conditional statement
    let proceed = true;
    if proceed {
        println!("Proceeding with the operation...");
    } else {
        println!("Operation aborted.");
    }

    //Conditional statement that checks heights
    let height = 180;
    if height > 200 {
        println!("You are tall!");
    } else if height > 150 {
        println!("You are of average height");
    } else {
        println!("You are short!");
    }

    //Another Conditional statement
    let mut height = 190;
    height = height - 20;

    let result = if height > 200 {
        "Tall"
    } else if height > 150 {
        "Average"
    } else {
        "Short"
    };

    println!("Result: {}. \nYour height is: {}", result, height);

    //Inline conditional statement
    let age = 26;
    if age >= 18 {
        "you are qualify to vote"
    } else {
        "you are not qualify to vote"
    };

    println!("Age: {}", age);
}
