use std::iter::zip;
use std::ops::Index;
use std::{any, fs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];
    let mut input : Vec<Vec<char>> = Vec::new();

    let mut sum_a : u64 = 0;
    let mut sum_b : u64 = 0;

    for line in fs::read_to_string(env::current_dir().unwrap().into_os_string().into_string().unwrap() + "/" + rel_path).unwrap().lines().skip(0) {
        
        let vchar : Vec<char> = line
                                .chars()
                                .collect();      

        input.push(vchar);
    }
                        
                        
    sum_a += solve(&mut input, true); 
    sum_b += solve(&mut input, false); 

    println!("part a: {}\npart b: {}", sum_a, sum_b)
}

fn solve(input : &mut Vec<Vec<char>>, run_once : bool) -> u64 {

    let mut any_removed = true;
    let mut amount_removed = 0;

    let y_max = input.len();
    let x_max = input[0].len();

    println!("y and x {} {}", y_max, x_max);
    while any_removed {
        any_removed = false;
        
        for y in  0..input.len() {
            for x in 0..input[0].len() {

                if  input[y][x] == '@' && remove_roll(input, run_once, x, y, x_max, y_max) {
                    any_removed = true;
                    amount_removed += 1;
                }                


            } 
        }


        if run_once {break;}
    }


    return amount_removed;
}

fn remove_roll(input : &mut Vec<Vec<char>>, keep : bool, x : usize, y : usize, x_max :usize, y_max : usize) -> bool {
    let mut neighbours : usize = 0;



    // NW
    if x > 0 && y > 0 {
        if input[y - 1][x - 1] == '@' || input[y - 1][x - 1] == 'x' {
            neighbours += 1;
        }
    }

    //N 
    if y > 0 {
        if input[y - 1][x] == '@' || input[y - 1][x] == 'x' {
            neighbours += 1;
        }
    }

    //NE 
    if y > 0 && x + 1 < x_max {
        if input[y - 1][x + 1] == '@' || input[y - 1][x + 1] == 'x' {
            neighbours += 1;
        }
    }

    //E
    if x + 1 < x_max {
        if input[y][x + 1] == '@' || input[y][x + 1] == 'x' {
            neighbours += 1;
        }
    }

    //SE
    if x + 1 < x_max && y + 1 < y_max{
        if input[y + 1][x + 1] == '@' || input[y + 1][x + 1] == 'x'{
            neighbours += 1;
        }
    }

    //S
    if y + 1 < y_max{
        if input[y + 1][x] == '@' || input[y + 1][x] == 'x'{
            neighbours += 1;
        }
    }

    //SW
    if y + 1 < y_max && x > 0{
        if input[y + 1][x - 1] == '@' || input[y + 1][x - 1] == 'x' {
            neighbours += 1;
        }
    }

    //W
    if x > 0{
        if input[y][x - 1] == '@' || input[y][x - 1] == 'x' {
            neighbours += 1;
        }
    }

    if neighbours < 4 {

        if !keep {
            input[y][x] = '.';
        }

        return true;
    }

    return false;
}