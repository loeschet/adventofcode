// use for interacting with files
use std::fs::File;
// First, we bring std::io::prelude::* into scope, as this is a shortcut to
// import many useful things like std::io::Read, str::io::Write etc. The
// BufReader has a useful method called lines() which returns an iterator
// over the lines of a file without the final /n whitespace at the end.
use std::io::{prelude::*, BufReader};

// useful for comparing two values
use std::cmp::Ordering;

pub fn solve_day_one_one() {

    // open file containing depths plus some error handling
    // File::open() returns Result object that can be either Ok or Err type,
    // depending on success or failure to read the file.
    // Ok type contains the actual value (in this case the file), while
    // Err contains error object. 
    let myfile = match File::open("depths.txt"){
        Ok(f) => f,
        Err(_) => panic!("Error! Could not open file!"),
    };

    // create BufReader using our file
    let reader = BufReader::new(myfile);

    // boolean for first iteration
    let mut first = true;
    
    // this keeps track of how often the depth increased
    let mut tracker: u32 = 0;

    // define variables to contain current depth value and newly measured value
    let mut current_depth: u32 = 0;
    let mut new_depth: u32;

    //loop over iterator
    for line in reader.lines() {

        // parse line to integer plus some error handling.
        new_depth = match line.expect("There was an error reading this line!").parse() {
            Ok(num) => num,
            Err(_) => panic!("Error! Could not parse line to number!"),
        };

        if first {
            println!("{} (N/A - no previous measurement)", new_depth);
            current_depth = new_depth;
            first = false;
        } else {
            match current_depth.cmp(&new_depth) {
                Ordering::Less => {
                    println!("{} (increased)", new_depth);
                    tracker = tracker + 1;
                },
                Ordering::Greater => println!("{} (decreased)", new_depth),
                Ordering::Equal => println!("{} (stayed the same)", new_depth),
        }
        current_depth = new_depth;
    }
}

    println!("Depth increased {} times!", tracker);

}

pub fn solve_day_one_two() {

    // more or less copied from above :)
    let myfile = match File::open("depths.txt"){
        Ok(f) => f,
        Err(_) => panic!("Error! Could not open file!"),
    };

    // create BufReader using our file
    let reader = BufReader::new(myfile);

    // boolean for first iteration
    let mut first = true;
    
    // this keeps track of how often the sum increased
    let mut tracker: u32 = 0;

    // define variables to contain current sum and newly measured sum
    let mut current_result: u32 = 0;
    let mut new_result: u32;

    // Use BufReader to turn read-in values into vector of strings
    let str_vector: Vec<String> = reader.lines()
                                         .map(|l| l.expect("error parsing line"))
                                         .collect();

    // Turn vector of strings into vector of integers
    let uint_vector: Vec<u32> = str_vector.iter().flat_map(|x| x.parse()).collect();

    for n in 1..=uint_vector.len()-2 {
        new_result = uint_vector[n-1..=n+1].iter().sum();

        if first {
            println!("{} (N/A - no previous measurement)", new_result);
            current_result = new_result;
            first = false;
        } else {
            match current_result.cmp(&new_result) {
                Ordering::Less => {
                    println!("{} (increased)", new_result);
                    tracker = tracker + 1;
                },
                Ordering::Greater => println!("{} (decreased)", new_result),
                Ordering::Equal => println!("{} (stayed the same)", new_result),
        }
        current_result = new_result;
        }
    }

    println!("Depth increased {} times!", tracker);

}
