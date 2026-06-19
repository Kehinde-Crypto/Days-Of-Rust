<div align="center">
  <h1>🦀 30 Days Of Rust: Day 3 - Control Flow 🚀</h1>
  <a href="https://www.linkedin.com/in/het-patel-8b110525a/?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=android_app" target="_blank">
    <img src="https://img.shields.io/badge/style--5eba00.svg?label=LinkedIn&logo=linkedin&style=social" alt="LinkedIn" />
  </a><a href="https://github.com/Hunterdii" target="_blank">
    <img src="https://img.shields.io/badge/Follow%20me%20on-GitHub-blue?style=flat-square&logo=github" alt="Follow me on GitHub" />
</a>

<sub><h4><i>Author:
  <a href="https://www.linkedin.com/in/het-patel-8b110525a/?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=android_app" target="_blank">Het Patel</a></h4></i>
<small> October, 2024</small>
</sub>

</div>

[<< Day 2](../02_Variables%2C%20DataTypes/02_variables_data_types.md) | [Day 4 >>](../04_Functions/04_functions.md)

![30DaysOfRust](https://github.com/user-attachments/assets/a1083fb3-3eec-4d1e-b93a-fa4d7a99f180)

- [📘 Day 3 - Control Flow](#-day-3---control-flow)
  - [👋 Welcome](#-welcome)
  - [🔍 Overview](#-overview)
  - [🛠 Environment Setup](#-environment-setup)
  - [📖 Control Flow in Rust](#-control-flow-in-rust)
    - [🧩 The `if` Statement](#-the-if-statement)
    - [🔄 The `else` and `else if` Statements](#-the-else-and-else-if-statements)
    - [🔁 The `loop` Statement](#-the-loop-statement)
    - [🔄 The `while` Loop](#-the-while-loop)
    - [🔁 The `for` Loop](#-the-for-loop)
    - [🛑 Control Flow with `match`](#-control-flow-with-match)
  - [🎯 Hands-On Challenge](#-hands-on-challenge)
  - [💻 Exercises - Day 3](#-exercises---day-3)
    - [✅ Exercise: Level 1](#-exercise-level-1)
    - [✅ Exercise: Level 2](#-exercise-level-2)
    - [🎥 Helpful Video References](#-helpful-video-references)
  - [📝 Day 3 Summary](#-day-3-summary)

---

# 📘 Day 3 - Control Flow

## 👋 Welcome

Welcome to **Day 3** of the 30 Days of Rust! 🎉 Today, we will explore **Control Flow** in Rust. Understanding control flow is essential for directing the flow of your program based on conditions, repeating actions, or managing various scenarios effectively. Let’s dive into it! 🚀

**Congratulations!** 🎉 You've taken the first step in your journey to master the _30 Days of Rust_ programming challenge. In this challenge, you will learn the fundamentals of Rust and how to harness its power to write efficient, fast, and safe code. By the end of this journey, you'll have gained a solid understanding of Rust's core concepts and best practices, helping you become a confident Rustacean. 🦀


Feel free to join the [30 Days of Rust](https://discord.gg/dy4gAhng) community on Discord, where you can interact with others, ask questions, and share your progress!

## 🔍 Overview

In Rust, control flow allows you to:
- Make decisions using conditional statements.
- Repeat actions using different types of loops.
- Handle multiple scenarios efficiently.

We will cover:
- `if`, `else`, and `else if` statements
- Various loop mechanisms: `loop`, `while`, and `for`
- The powerful `match` statement for pattern matching

## 🛠 Environment Setup

Ensure that you have your Rust environment set up correctly from Day 1. If you haven’t installed Rust yet, please refer to the setup instructions from [Day 1](../README.md#-environment-setup).



## 📖 Control Flow in Rust

### 🧩 The `if` Statement

The `if` statement lets you execute a block of code only if a specified condition is true.

**Example:**

```rust
fn main() {
    let number = 10;

    if number > 5 {
        println!("The number is greater than 5");
    }
}
```

**Output:**

```
The number is greater than 5
```

### 🔄 The `else` and `else if` Statements

Use `else` and `else if` to add alternative conditions.

**Example:**

```rust
fn main() {
    let age = 18;

    if age >= 21 {
        println!("You can drink alcohol in the INDIA.");
    } else if age >= 18 {
        println!("You are an adult, but cannot drink alcohol in the INDIA.");
    } else {
        println!("You are a minor.");
    }
}
```

**Output:**

```
You are an adult, but cannot drink alcohol in the INDIA.
```

### 🔁 The `loop` Statement

The `loop` statement repeatedly executes a block of code. To break out, use the `break` keyword.

**Example:**

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }
}
```

**Output:**

```
Breaking the loop at count: 3
```

### 🔄 The `while` Loop

The `while` loop continues to run while a condition is true.

**Example:**

```rust
fn main() {
    let mut num = 1;

    while num <= 5 {
        println!("Number is: {}", num);
        num += 1;
    }
}
```

**Output:**

```
Number is: 1
Number is: 2
Number is: 3
Number is: 4
Number is: 5
```

### 🔁 The `for` Loop

The `for` loop is useful for iterating over a collection or a range of numbers.

**Example:**

```rust
fn main() {
    for  num in 1..4 {
        println!("Num: {}", num);
    }
}
```

**Output:**

```
Num: 1
Num: 2
Num: 3
```

### 🛑 Control Flow with `match`

The `match` statement lets you handle multiple scenarios by pattern matching.

**Example:**

```rust
fn main() {
    let traffic_light = "green";

    match traffic_light {
        "green" => println!("Go"),
        "yellow" => println!("Slow down"),
        "red" => println!("Stop"),
        _ => println!("Invalid color"),
    }
}
```

**Output:**

```
Go
```

## 🎯 Hands-On Challenge

Write a program that:

1. Asks the user to input a number.
2. Uses an `if` statement to check if the number is even or odd.
3. Use a `loop` to print numbers from 1 to 5.
4. Implement a `match` statement to respond to different days of the week, e.g., "Monday" => "Start of the week!", "Friday" => "Weekend is coming!", etc.


## 💻 Exercises - Day 3

### ✅ Exercise: Level 1

1. Write a program that checks if a number is even or odd using the `if` statement.
2. Create a `while` loop that prints numbers from 1 to 10.
3. Use the `for` loop to iterate over an array of your favorite colors and print each one.
4. Create a simple calculator using the `match` statement that performs addition, subtraction, multiplication, or division based on user input.
5. Write a program that continuously takes user input until the word "exit" is typed, using a `loop`.

### ✅ Exercise: Level 2

1. Create a program that calculates the factorial of a given number using a `while` loop.
2. Write a program that simulates a countdown timer using a `loop` and breaks when the countdown reaches zero.
3. Use the `for` loop to calculate the sum of even numbers from 1 to 50.
4. Write a program that reads a string input and uses the `match` statement to respond with different outputs based on the input (e.g., "hello" => "Hi there!", "bye" => "Goodbye!", etc.).
5. Implement a program that uses `if` statements inside a `for` loop to print all the odd numbers from 1 to 20.
6. Create a small game where the program generates a random number between 1 and 10, and the user has to guess it. Use a `loop` to keep asking until the user gets it right.

## 🎥 Helpful Video References

- [Control Flow in Rust - Rust Programming Tutorial(Language : Hindi)](https://www.youtube.com/watch?v=i4kkNYom9VM)
- [Control Flow Statements & Conditional Expressions(Language : English)](https://youtu.be/3XpxkJXggHA?si=0J1W-NqQJN1Cg2--)
- [Rust Loops Explained](https://www.youtube.com/watch?v=NGR-q8_RXAk)

## 📝 Day 3 Summary

- Learned about using `if`, `else`, and `else if` for conditional checks.
- Explored `loop`, `while`, and `for` loops to repeat actions.
- Discovered the `match` statement for effective control flow management.

🌟 _Great job on completing Day 3! Keep practicing, and get ready for Day 3 where we will explore Functions in Rust!_

Thank you for joining **Day 3** of the 30 Days of Rust challenge! If you found this helpful, don’t forget to <img src="https://github.com/user-attachments/assets/35f6838c-52f5-4e48-8a98-c5203f8c57e3" style="width:20px; color: #FFD700" alt="Star GIF"> star this repository, share it with your friends, and stay tuned for more exciting lessons ahead!


**Stay Connected**  
📧 **Email**: [Hunterdii](mailto:hunterdii9879@gmail.com)  
🐦 **Twitter**: [@HetPate94938685](https://twitter.com/HetPate94938685)  
🌐 **Website**: [Working On It(Temporary)](https://hunterdii.github.io/Portfolio-Temporary/)

[<< Day 2](../02_Variables%2C%20DataTypes/02_variables_data_types.md) | [Day 4 >>](../04_Functions/04_functions.md)

---
