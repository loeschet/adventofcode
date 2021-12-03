use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::Error;
use std::cmp::Ordering;
use std::thread::current;

pub fn solve_day_three_one() {

    // more or less copied from above :)
    let myfile = match File::open("binaries.txt"){
        Ok(f) => f,
        Err(_) => panic!("Error! Could not open file!"),
    };

    // create BufReader using our file
    let reader = BufReader::new(myfile);

    let mut bytevec: Vec<u32> = Vec::new();
    let mut bytesize: usize = 0;

    for line in reader.lines() {
        let mut current_vec: Vec<u32> = line.expect("There was an error reading this line").chars().map(|s| s.to_digit(10).expect("parsing error!")).collect();
        if bytesize == 0 {
            bytesize = current_vec.len();
        }
        bytevec.append(&mut current_vec);
    }

    let input_len = (bytevec.len() as f32)/(bytesize as f32);
    println!("Input length: {}", input_len);

    let mut result_vec: Vec<u32> = Vec::new();
    for i in 0..bytesize{
        result_vec.push(0);
    }

    for n in 0..bytevec.len() {
        let current_idx = n % bytesize;
        result_vec[current_idx] += bytevec[n];
    }

    let reference = input_len / 2.0;

    let mut result_gamma: Vec<u32>  = Vec::new();
    let mut result_epsilon: Vec<u32>  = Vec::new();
    for k in 0..bytesize{
        let res_bool = result_vec[k] as f32 > reference;
        if res_bool {
            result_gamma.push(1);
            result_epsilon.push(0);
        } else {
            result_gamma.push(0);
            result_epsilon.push(1);
        }
    }

    let mut decimal_gamma: f32 = 0.0;
    let mut decimal_epsilon: f32 = 0.0;

    println!("Reference value {}", reference);
    println!("Result vector values:");

    for j in 0..result_gamma.len() {
        println!("Result vector at position {}: {}", j, result_vec[j]);
        decimal_gamma += (result_gamma[j] as f32)*(2.0 as f32).powf(((result_gamma.len()-1)-j) as f32);
        decimal_epsilon += (result_epsilon[j] as f32)*(2.0 as f32).powf(((result_epsilon.len()-1)-j) as f32);

    }
    println!("Decimal value of gamma : {}", decimal_gamma);
    println!("Decimal value of epsilon : {}", decimal_epsilon);
    println!("Product : {}", decimal_gamma * decimal_epsilon);
}

pub fn solve_day_three_two() {

    // more or less copied from above :)
    let myfile = match File::open("binaries.txt"){
        Ok(f) => f,
        Err(_) => panic!("Error! Could not open file!"),
    };

    // create BufReader using our file
    let reader = BufReader::new(myfile);

    let mut bytevec: Vec<u32> = Vec::new();
    let mut bytesize: usize = 0;

    for line in reader.lines() {
        let mut current_vec: Vec<u32> = line.expect("There was an error reading this line").chars().map(|s| s.to_digit(10).expect("parsing error!")).collect();
        if bytesize == 0 {
            bytesize = current_vec.len();
        }
        bytevec.append(&mut current_vec);
    }

    let input_len = (bytevec.len() as f32)/(bytesize as f32);
    println!("Input length: {}", input_len);

    let mut result_vec: Vec<u32> = Vec::new();
    for i in 0..bytesize{
        result_vec.push(0);
    }

    for n in 0..bytevec.len() {
        let current_idx = n % bytesize;
        result_vec[current_idx] += bytevec[n];
    }

    let reference = input_len / 2.0;

    let mut ref_oxygen: Vec<u32>  = Vec::new();
    let mut ref_scrubber: Vec<u32>  = Vec::new();
    for k in 0..bytesize{
        if result_vec[k] as f32 >= reference {
            ref_oxygen.push(1);
            ref_scrubber.push(0);
        } else {
            ref_oxygen.push(0);
            ref_scrubber.push(1);
        }
    }

    let mut oxygen_res: Vec<u32> = bytevec.clone();
    let mut scrubber_res: Vec<u32> = bytevec.clone();

    let mut ref_value = ref_oxygen[0];
    for n in 0..bytesize{
        let mut helper_oxy: Vec<u32> = Vec::new();
        let mut bitcount: u32 = 0;

        for k in (n..oxygen_res.len()).step_by(bytesize) {

            if oxygen_res[k] == ref_value {
                helper_oxy.extend_from_slice(&mut oxygen_res[(k-n)..(k+(bytesize-n))])
            }
        }

        oxygen_res = helper_oxy.clone();

        if oxygen_res.len() == bytesize {
            println!("That worked!");
            break;
        }

        for k in (n+1..oxygen_res.len()).step_by(bytesize) {
            bitcount += oxygen_res[k];
        }

        if bitcount as f32 >= (((oxygen_res.len() as f32) / (bytesize as f32)) / 2.0) {
            ref_value = 1;
        } else {
            ref_value = 0;
        }
    }

    println!("reference vector for oxygen {:?}", ref_oxygen);
    println!("result vector for oxygen {:?}", oxygen_res);

    let mut ref_value = ref_scrubber[0];
    for n in 0..bytesize{
        let mut helper_scrub: Vec<u32> = Vec::new();
        let mut bitcount: u32 = 0;

        for k in (n..scrubber_res.len()).step_by(bytesize) {

            if scrubber_res[k] == ref_value{
                helper_scrub.extend_from_slice(&mut scrubber_res[(k-n)..(k+(bytesize-n))])
            }
        }

        scrubber_res = helper_scrub.clone();

        if scrubber_res.len() == bytesize {
            println!("That worked!");
            break;
        }

        for k in (n+1..scrubber_res.len()).step_by(bytesize) {
            bitcount += scrubber_res[k];
        }

        if bitcount as f32 >= (((scrubber_res.len() as f32) / (bytesize as f32)) / 2.0) {
            ref_value = 0;
        } else {
            ref_value = 1;
        }
    }

    println!("reference vector for scrub {:?}", ref_scrubber);
    println!("result vector for scrub {:?}", scrubber_res);


    let mut decimal_oxy: f32 = 0.0;
    let mut decimal_scrub: f32 = 0.0;

    for j in 0..bytesize {
        decimal_oxy += (oxygen_res[j] as f32)*(2.0 as f32).powf(((oxygen_res.len()-1)-j) as f32);
        decimal_scrub += (scrubber_res[j] as f32)*(2.0 as f32).powf(((scrubber_res.len()-1)-j) as f32);
    }

    println!("Decimal value of oxygen : {}", decimal_oxy);
    println!("Decimal value of scrub : {}", decimal_scrub);
    println!("Product : {}", decimal_oxy * decimal_scrub);
}