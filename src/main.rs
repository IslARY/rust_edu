#![allow(unused)] //Allows for unused

use core::panic;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);

    // CONST
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.1415;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number!");
    // Shadowing: Multiple variable with same name but different datatypes.
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // Generating Random Number;
    let random_number: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_number);

    // Ordering
    let my_age = 30;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("Can Vote"),
    }
    // For Loop;
    let array_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for val in array_1.iter() {
        println!("Array Values: {}", val);
    }
    // Strings Overview
    let st3 = String::from("A B C B D E F E A G F"); // Converts &str -> String, and is stored in Heap
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("Heap Allocated String from &str: {}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String6: {} length: {}", st6, st6.len());
    st5.clear();
    let st6 = String::from("Just Some");
    let st7 = String::from(" Words");
    let st8 = st6 + &st7; // st6 will cease to exist;
                          // println!("St6: {}", st6); Error
    println!("Concatenated Strings: {}", st8);

    // Vector
    let mut vec1: Vec<i32> = Vec::new();
    vec1 = vec![1, 2, 3, 4, 5];
    vec1.push(6);
    println!("{:#?}", vec1);

    // Hashmap
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Flash", "Barry Allen");
    heroes.insert("Batman", "Bruce Wayne");
    for (k, v) in heroes.iter() {
        println!("{} is {}", k, v);
    }
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Unable to find the hero with that name."),
        }
    }

    // Traits and Structs
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        radius: f32,
    };
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(radius: f32, diameter: f32) -> Circle {
            return Circle { radius };
        }
        fn area(&self) -> f32 {
            return (self.radius / 2.0).powf(2.0) * PI;
        }
    }
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area: {}", rec.area());
    println!("Circ Area: {}", circ.area());

    // Handling Files and Errors
    let path = "File.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error)
        }
    };
    write!(output, "I am Vengeance!").expect("Failed to write to File.");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}
