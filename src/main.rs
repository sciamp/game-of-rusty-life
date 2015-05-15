extern crate rand;
use rand::*;

fn seed(universe: &mut [[bool;100];100]) {
    for i in 0..100 {
        for j in 0..100 {
            universe[i][j] = random::<bool>();
        }
    }
}

fn main() {
    let mut universe: [[bool;100];100] = [[false;100];100];

    seed(&mut universe);

    println!("{}", universe);
}
