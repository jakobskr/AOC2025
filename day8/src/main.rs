

use std::{ fs, usize};
use std::env;

#[derive(Debug)]
struct JuncBox {
    x : usize,
    y : usize,
    z : usize
}


struct DSU {
    parent: Vec<usize>,
    size : Vec<usize>
}


impl DSU {

    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), 
            size: vec![1; n]
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
            return self.parent[x];
        }
        return self.parent[x];
    }

    // fn find1(&mut self, x: usize) -> usize {
    //     if self.parent[x] != x {
    //         self.parent[x] = self.find(self.parent[x]);
    //     }
    //     return self.parent[x]
    // }

   fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        // attach smaller tree to larger tree
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.size[rb] = 0;

        true
    }

    fn top_n(&self, n: usize) -> Vec<usize> {
        let mut sizes: Vec<usize> = self.size.iter().cloned().filter(|&sz| sz > 0).collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes.into_iter().take(n).collect()
    }


}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];

    let sum_a : usize;
    let sum_b : usize;


    let mut junc_vec : Vec<JuncBox> = Vec::new();

    for line in fs::read_to_string(env::current_dir().unwrap().into_os_string().into_string().unwrap() + "/" + rel_path).unwrap().lines().skip(0) {
        
        let cords : Vec<&str> = line
                                .split(",")
                                .collect();
                        
        let jb : JuncBox = JuncBox { 
            x: (cords[0].parse::<usize>().unwrap()), 
            y: (cords[1].parse::<usize>().unwrap()), 
            z: (cords[2].parse::<usize>().unwrap()), 
        };                          

        junc_vec.push(jb);
    }
                        
    let mut distances: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..junc_vec.len() {
        for j in (i + 1)..junc_vec.len() {
            let dist = abs_distance(&junc_vec[i], &junc_vec[j]);
            distances.push((i, j, dist));
        }
    }

    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut dsu : DSU = DSU::new(1000);
    

    for i in 0..1000 {
        let (p, q, _) = distances[i];
        
        dsu.union(p, q);
    }
    
    sum_a = dsu.top_n(3).iter().fold(1, |acc, x| x * acc);
    
    let mut q : usize = 0;
    let mut p : usize = 0;

    let mut x = 0;
    while *dsu.size.iter().max().unwrap() < junc_vec.len() {        
        (p, q, _) = distances[x];
        dsu.union(p, q);
        x += 1;

    }

    sum_b = junc_vec[p].x * junc_vec[q].x; 
        
    println!("part a: {}\npart b: {}", sum_a, sum_b);
}


// returns squared distance between q and p,
// we dont care about what the distance is only the difference between other distances.
fn abs_distance(p : &JuncBox, q: &JuncBox) -> usize {

    return (
        (p.x as i64 - q.x as i64).pow(2)
    + (p.y as i64 - q.y as i64).pow(2)
    + (p.z as i64 - q.z as i64).pow(2)
    ) as usize;
}