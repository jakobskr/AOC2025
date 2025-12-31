use std::iter::zip;
use std::{fs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];

    let mut sum_a : u64 = 0;
    let mut sum_b : u64 = 0;

   let contents = fs::read_to_string(
    env::current_dir().unwrap().into_os_string().into_string().unwrap()
        + "/" + rel_path
    ).unwrap();

    let mut lines: Vec<&str> = contents.lines().collect();

    let opers : Vec<&str> = lines.pop().unwrap().trim().split(" ").filter(|xd| !xd.is_empty()).collect();

    let vint : Vec<Vec<u64>> = lines.iter().map(|x: &&str| x.trim().split(" ").filter(|xd| !xd.is_empty()).map(|y| y.parse::<u64>().unwrap()).collect()).collect();

    let mut rotated = vec![vec![vint[0][0]; vint.len()]; vint[0].len()];

    for r in 0..vint.len() {
        for c in 0..vint[0].len() {
            rotated[c][r] = vint[r][c];
        }
    }

    // part 2 clown fieste : )
    
    let mut input_chars : Vec<Vec<char>> = Vec::new();

    for l in lines.into_iter() {
        input_chars.push(l.chars().collect());
    }

    let mut rot_input : Vec<Vec<char>> = vec![vec![input_chars[0][0]; input_chars.len()]; input_chars[0].len()];

    for y in 0..rot_input.len() {
        for x in 0..rot_input[0].len() {
            rot_input[y][x] = input_chars[x][y];
        }

    }

    let mut p_chunks2 : Vec<Vec<u64>> = Vec::new();

    let mut temp : Vec<u64> = Vec::new();
    
    for l in rot_input.iter() {

        if l.iter().all(|f| *f == ' ') {
            p_chunks2.push(temp);
            temp = Vec::new();
            continue;
        }
        temp.push( String::from_iter(l.iter()).trim().parse::<u64>().unwrap());
    }  
    
    p_chunks2.push(temp);

    let problems_a : Vec<(&str, Vec<u64>)>  = zip(opers.clone(), rotated).collect(); 
    let problems_b  : Vec<(&str, Vec<u64>)>  = zip(opers.clone(), p_chunks2).collect(); 
    
                        
    sum_a += solve(&problems_a); 
    sum_b += solve(&problems_b); 

    println!("part a: {}\npart b: {}", sum_a, sum_b)
}


fn solve(input: & Vec<(&str, Vec<u64>)>) -> u64 {
    let mut sum: u64 = 0;

    for p in input {
        let mut carry = p.1[0];
        let oper = p.0;
        for x in p.1.iter().skip(1) {
            if oper == "*" {
                carry *= x;
            }

            if oper == "+" {
                carry += x;
            }


        }
        sum += carry;
    }

    return sum;

}

