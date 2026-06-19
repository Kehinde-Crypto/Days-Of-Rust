// fn main(){
//     let mut  x = 5;
//     println!("The value of before  is: {}", x);
//      x  = 10;
//     println!("The value of x After: {}", x);
// }
// fn main(){
//  let mut mutable_variable = 10;
//     println!("Before: {}", mutable_variable);
//     mutable_variable += 5;
//     println!("After: {}", mutable_variable);
// }

// fn main() {
//     let age = 18;

//     if age >= 21 {
//         println!("You can drink alcohol in the INDIA.");
//     } else if age >= 18 {
//         println!("You are an adult, but cannot drink alcohol in the INDIA.");
//     } else {
//         println!("You are a minor.");
//     }
// }


// ## 💻 Exercises - Day 2

// ### ✅ Exercise: Level 1

// 3. Create a mutable variable named `my_height` and assign it your height in centimeters. Update it to a new height.
// 4. Declare a variable `my_name` and assign it your name as a string. Print it to the console.
// 5. Create a variable `is_student` and set it to `true` if you are a student, or `false` otherwise. Print the value.
// 6. Create a variable `birth_year` and calculate your birth year by subtracting your age from the current year (you can use a hardcoded current year, e.g., `2024`). Print the value.

// fn main() {
//      let my_age = 20;
//      println!("My age is: {}", my_age);

//      let mut my_height = 170; // in centimeters
//      my_height = 175; // updated height
//      println!("My height is: {} cm", my_height);

//     let my_name = "Kehinde";
//     println!("My name is: {}", my_name);

//     let birth_year = 2024 - 20; // Assuming age is 20
//     println!("My birth year is: {}", birth_year);
// }



// ### ✅ Exercise: Level 2

// 1. Create variables for each numeric type (integer and float) and print their values:
//    - An integer variable `my_integer` set to any integer value.
//    - A floating-point variable `my_float` set to any float value.
// 2. Declare a boolean variable `is_learning_rust` and set it to `true`. Print the value.
// 3. Create a character variable `favorite_letter` and assign it your favorite letter. Print it.
// 4. Create an array of integers called `my_scores` that holds your last five test scores. Print the entire array.
// fn main() {
//     let my_scores: [i32; 5] = [85, 90, 78, 92, 88];
//     println!("My scores are: {:?}", my_scores);

// 
// number four
//}

// 5. Create a string variable `hobby` and assign it one of your hobbies. Print it, and then concatenate it with another string to create a sentence (e.g., "I enjoy [hobby]!"). Print the complete sentence.
// fn main() {
// let hobby = String::from("reading"); 
// println!("My hobby is: {}", hobby);
// let complete_sentence = format!("I enjoy {}!", hobby);
// println!("{}", complete_sentence);

// }
// ## 🎯 Hands-On Challenge

// Write a program that:

// 1. Asks the user to input a number.
// 2. Uses an `if` statement to check if the number is even or odd.
// 3. Use a `loop` to print numbers from 1 to 5.
// 4. Implement a `match` statement to respond to different days of the week, e.g., "Monday" => "Start of the week!", "Friday" => "Weekend is coming!", etc.

// use std::io;

// fn main() {
//     // 1 & 2: Input a number, check even or odd
//     let mut input = String::new();
//     println!("Enter a number:");
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     let number: i32 = input.trim().parse().expect("Please enter a valid number");

//     if number % 2 == 0 {
//         println!("{} is even", number);
//     } else {
//         println!("{} is odd", number);
//     }

//     // 3: Loop to print 1 to 5
//     let mut count = 1;
//     loop {
//         println!("{}", count);
//         if count == 5 {
//             break;
//         }
//         count += 1;
//     }

//     // 4: Match statement for days of the week
//     let day = "Friday";
//     match day {
//         "Monday"    => println!("Start of the week!"),
//         "Tuesday"   => println!("Keep pushing!"),
//         "Wednesday" => println!("Halfway there!"),
//         "Thursday"  => println!("Almost Friday!"),
//         "Friday"    => println!("Weekend is coming!"),
//         "Saturday"  => println!("Enjoy your weekend!"),
//         "Sunday"    => println!("Rest up!"),
//         _           => println!("Not a valid day"),
//     }
// }


// ## 💻 Exercises - Day 3

// ### ✅ Exercise: Level 1

// 1. Write a program that checks if a number is even or odd using the `if` statement.
// fn main(){
//     for number in 1..=10 {
//         if number % 2 == 0 {
//             println!("{} is even", number);
//         } else {
//             println!("{} is odd", number);
//         }
//     }
// }
// 2. Create a `while` loop that prints numbers from 1 to 10.
// fn main() {
// let mut number = 1;
// while number <= 10 {
//     println!("{}", number);
//     number += 1;
// }
//  }
// 3. Use the `for` loop to iterate over an array of your favorite colors and print each one.
// 4. Create a simple calculator using the `match` statement that performs addition, subtraction, multiplication, or division based on user input.
// 5. Write a program that continuously takes user input until the word "exit" is typed, using a `loop`.

// ### ✅ Exercise: Level 2

// 1. Create a program that calculates the factorial of a given number using a `while` loop.
// 2. Write a program that simulates a countdown timer using a `loop` and breaks when the countdown reaches zero.
// 3. Use the `for` loop to calculate the sum of even numbers from 1 to 50.
// 4. Write a program that reads a string input and uses the `match` statement to respond with different outputs based on the input (e.g., "hello" => "Hi there!", "bye" => "Goodbye!", etc.).
// 5. Implement a program that uses `if` statements inside a `for` loop to print all the odd numbers from 1 to 20.
// 6. Create a small game where the program generates a random number between 1 and 10, and the user has to guess it. Use a `loop` to keep asking until the user gets it right.
