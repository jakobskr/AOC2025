use std::iter::zip;
use std::ops::Index;
use std::{fs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];
    let mut _input : Vec<String> = Vec::new();

    let mut sum_a : u64 = 0;
    let mut sum_b : u64 = 0;

    for line in fs::read_to_string(env::current_dir().unwrap().into_os_string().into_string().unwrap() + "/" + rel_path).unwrap().lines().skip(0) {
        
        let vint : Vec<u64> = line
                                .chars()
                                .map(|c| c.to_digit(10).unwrap() as u64)
                                .collect();      

        sum_a += n_digits(2, &vint); 
        sum_b += n_digits(12, &vint); 
    }

    

    println!("part a: {}\npart b: {}", sum_a, sum_b)
}


fn n_digits(n : usize, list : &Vec<u64>) -> u64 {

    let mut dig_found : usize = 0;

    let len = list.len();
    let mut j = 0;
    
    let mut bank : Vec<u64> = Vec::new();

    while dig_found < n {
        let mut max = 0;
        let mut max_index = 0;
        
        let upper = len - n + dig_found + 1;

        for k in j..upper {
            if list[k] > max {
                max = list[k];
                max_index = k;
            }


        }

        j = max_index + 1;
        dig_found += 1;
        bank.push(max);
        
    }
    
    
    let res = bank.iter().map(|f| f.to_string()).collect::<String>().parse::<u64>().unwrap();
    // println!("{}", res);
    return res;
}