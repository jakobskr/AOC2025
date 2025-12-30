use std::iter::zip;
use std::{fs};
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let rel_path = &args[1];
    let mut _input : Vec<String> = Vec::new();

    let mut sum : u64 = 0;
    let mut sumb : u64 = 0;

    let p = env::current_dir().unwrap().into_os_string().into_string().unwrap() + rel_path;


    for line in fs::read_to_string(env::current_dir().unwrap().into_os_string().into_string().unwrap() + "/" + rel_path).unwrap().split(',') {
        // println!("{}", line);
        
        let ranges : Vec<&str> = line.split("-").collect();

        let lower : u64 = ranges[0].parse().unwrap();
        let upper : u64 = ranges[1].parse().unwrap();


        for i in lower .. upper + 1 {
            // println!("{}",i);
            
            let l = i.to_string();
            
            
            // part a;
            if l.len() % 2 == 0 {
                
                let d = l.split_at(l.len() / 2);
                let left = d.0.chars();
                let right = d.1.chars();

                let is_equal = zip(left, right).all(|x| x.1 == x.0);

                if is_equal {
                    sum += i;
                }

            }


            let l_vec : Vec<char> = l.chars().collect();
            let mut n : String = String::new();
            
            let start = l_vec[0];
            n.push(start);

            for j in 1 .. l.len() {
                

                if l_vec[j] == start {


                    // println!("happens {:?} {:?} {}",n, i, j );
                    // check if n is complete : )
                    
                    let chunky = l_vec.chunks(n.len());

                    // println!("chunky {:?}", chunky);

                    let all_equal : bool = chunky.skip(1).map(|f| String::from_iter(f)).all(|f: String| f == n);
                    if all_equal {
                        sumb += i;
                        break;
                    }

                }

                
                n.push(l_vec[j]);
                
                           
            
            }

        } 
        
        
        // sum += 5;
        
    }

    println!("part a: {}, part b: {}", sum, sumb);

}
