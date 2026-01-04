use core::time;
use std::iter::zip;
use std::ops::Index;
use std::{any, fs, usize};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];
    let mut input : Vec<Vec<char>> = Vec::new();

    let mut sum_a : usize = 0;
    let mut sum_b : usize = 0;

    for line in fs::read_to_string(env::current_dir().unwrap().into_os_string().into_string().unwrap() + "/" + rel_path).unwrap().lines().skip(0) {
        
        let vchar : Vec<char> = line
                                .chars()
                                .collect();      

        input.push(vchar);
    }
                        
    let s = input[0].iter().position(|c| *c == 'S').unwrap();


    (sum_a, sum_b) = solve(&input, s, true); 

    println!("part a: {}\npart b: {}", sum_a, sum_b)
}


fn solve(gens : &Vec<Vec<char>>, start : usize , part_1: bool ) -> (usize, usize) {
    let mut curGen : Vec<usize> = vec![0; gens[0].len()];
    let mut splits: usize = 0;
    curGen[start] = 1;

    for g in gens.iter().skip(1) {
        let mut new_gen : Vec<usize> = vec![0; gens[0].len()];

        for i in 0..g.len() {

            if g[i] == '.' {
                new_gen[i] += curGen[i];
            }

            if g[i] == '^' && curGen[i] > 0 {
                splits += 1;
                if i > 0 {
                    new_gen[i - 1] += curGen[i];
                }

                if i + 1 < new_gen.len() {
                    new_gen[i + 1] += curGen[i];
                }

            }

        } 

        curGen = new_gen;

    }

    let timelines : usize = curGen.iter().sum();
    
    return (splits, timelines);
}