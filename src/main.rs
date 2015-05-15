extern crate rand;
use rand::*;

use std::thread;

struct Universe {
    matrix: [[bool;100];100]
}

fn seed(universe: &mut Universe) {
    for i in 0..100 {
        for j in 0..100 {
            universe.matrix[i][j] = random::<bool>();
        }
    }
}

fn maybe_get(matrix: &[[bool;100];100], i: i32, j: i32) -> u8 {
    if i < 0 || i > 99 || j < 0 || j > 99 {0}
    else { if matrix[i as usize][j as usize] {1} else {0} }
}

fn step(universe: Universe) -> Universe {
    let mut next = Universe{matrix: [[false;100];100]};

    for i in 0..100 {
        for j in 0..100 {
            let mut count = 0;
            count += maybe_get(&universe.matrix, i-1, j-1);
            count += maybe_get(&universe.matrix, i-1, j);
            count += maybe_get(&universe.matrix, i-1, j+1);
            count += maybe_get(&universe.matrix, i, j-1);
            count += maybe_get(&universe.matrix, i, j+1);
            count += maybe_get(&universe.matrix, i+1, j-1);
            count += maybe_get(&universe.matrix, i+1, j);
            count += maybe_get(&universe.matrix, i+1, j+1);

            next.matrix[i as usize][j as usize] = if universe.matrix[i as usize][j as usize] {
                match count {
                    0 | 1 => false,
                    2 | 3 => true,
                    _ => false
                }
            } else {
                count == 3
            }
        }
    }

    next
}

impl ToString for Universe {
    fn to_string(&self) -> String {
        let mut str_repr = String::new();
        for i in 0..100 {
            for j in 0..100 {
                str_repr.push_str(if (self.matrix[i][j]) {"o"} else {"-"});
                str_repr.push_str(" ");
            }
            str_repr.push_str("\n");
        }

        str_repr
    }
}

fn main() {
    let mut universe = Universe{matrix: [[false;100];100]};

    seed(&mut universe);

    loop {
        println!("{}", universe.to_string());
        universe = step(universe);
        thread::sleep_ms(100)
    }
}
