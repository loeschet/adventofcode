use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp::Ordering;



pub fn solve_day_two_one() {

    // more or less copied from above :)
    let myfile = match File::open("course.txt"){
        Ok(f) => f,
        Err(_) => panic!("Error! Could not open file!"),
    };

    // create BufReader using our file
    let reader = BufReader::new(myfile);

    // initialize values for horizontal position and depth
    let mut horizontal_pos: u32 = 0;
    let mut depth: u32 = 0;

    // Use BufReader to turn read-in values into vector of strings
    // Create vector with commands
    let string_vector: Vec<String> = reader.lines()
                                            .map(|l| l.expect("error parsing line"))
                                            .collect();

    let num_lines: usize = string_vector.len();

    let mut cmd_vector: Vec<String> = Vec::new();
    let mut num_vector: Vec<String> = Vec::new();

    for current_str in string_vector {
        let current_str_vec: Vec<&str> = current_str.split(" ").collect();
        cmd_vector.push(current_str_vec[0].to_string());
        num_vector.push(current_str_vec[1].to_string());
    }

    // Turn vector of strings into vector of integers
    let uint_vector: Vec<u32> = num_vector.iter().flat_map(|x| x.parse()).collect();

    for n in 0..num_lines {
        if cmd_vector[n] == "forward"{
            horizontal_pos += uint_vector[n];
        } else if cmd_vector[n] == "up" {
            depth -= uint_vector[n];
        } else if cmd_vector[n] == "down" {
            depth += uint_vector[n];
        } else {
            panic!("Something is wrong with your input!")
        }
    }

    println!("Final horizontal position: {}", horizontal_pos);
    println!("Final depth: {}", depth);
    println!("Product of depth and horizontal position: {}", depth*horizontal_pos);
}

pub fn solve_day_two_two() {

    // more or less copied from above :)
    let myfile = match File::open("course.txt"){
        Ok(f) => f,
        Err(_) => panic!("Error! Could not open file!"),
    };

    // create BufReader using our file
    let reader = BufReader::new(myfile);

    // initialize values for horizontal position, depth and aim
    let mut horizontal_pos: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    let mut cmd_vector: Vec<String> = Vec::new();
    let mut num_vector: Vec<String> = Vec::new();

    for line in reader.lines() {
        let current_str_vec: Vec<String> = line.expect("parsing error").split(" ").map(|s| s.to_string()).collect();

        cmd_vector.push(current_str_vec[0].to_string());
        num_vector.push(current_str_vec[1].to_string());
    }

    // Turn vector of strings into vector of integers
    let uint_vector: Vec<u32> = num_vector.iter().flat_map(|x| x.parse()).collect();

    for n in 0..cmd_vector.len() {
        if cmd_vector[n] == "forward"{
            horizontal_pos += uint_vector[n];
            depth += aim*uint_vector[n];
        } else if cmd_vector[n] == "up" {
            aim -= uint_vector[n];
        } else if cmd_vector[n] == "down" {
            aim += uint_vector[n];
        } else {
            panic!("Something is wrong with your input!")
        }
    }

    println!("Final horizontal position: {}", horizontal_pos);
    println!("Final depth: {}", depth);
    println!("Product of depth and horizontal position: {}", depth*horizontal_pos);
}