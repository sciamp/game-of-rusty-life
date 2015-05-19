extern crate rand;

use rand::*;
use std::thread;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

struct Universe {
    matrix: [[bool; WIDTH]; HEIGHT],
    epoch: u32
}

#[inline]
fn empty_matrix() -> [[bool; WIDTH]; HEIGHT] {
    [[false; WIDTH]; HEIGHT]
}

impl Universe {
    fn new_empty() -> Universe {
        Universe { matrix: empty_matrix(), epoch: 0 }
    }

    fn new_random() -> Universe {
        let mut result = Self::new_empty();

        for i in 0..WIDTH {
            for j in 0..WIDTH {
                result.matrix[i][j] = random::<bool>();
            }
        }

        result
    }

    fn maybe_get(&self, i: isize, j: isize) -> u8 {
        if i < 0 || i >= WIDTH as isize || j < 0 || j >= WIDTH as isize {
            0
        } else if self.matrix[i as usize][j as usize] {
            1
        } else {
            0
        }
    }

    fn is_alive(&self, i: isize, j: isize) -> bool {
        self.maybe_get(i, j) > 0
    }

    fn living_cells(&self) -> usize {
        let mut count = 0;

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                if self.matrix[i][j] {
                    count += 1;
                }
            }
        }

        count
    }

    fn step(&mut self) {
        let mut next = empty_matrix();

        for i in 0..(WIDTH as isize) {
            for j in 0..(HEIGHT as isize) {
                let neighbourhood = [
                    (i - 1, j - 1), (i - 1, j), (i - 1, j + 1),
                    (i, j - 1), (i, j + 1),
                    (i + 1, j - 1), (i + 1, j), (i + 1, j + 1),
                ];

                let mut count = 0;
                for &(x, y) in neighbourhood.iter() {
                    count += self.maybe_get(x, y);
                }

                next[i as usize][j as usize] = if self.is_alive(i, j) {
                    count == 2 || count == 3
                } else {
                    count == 3
                }
            }
        }

        self.epoch += 1;

        self.matrix = next;
    }
}

impl ToString for Universe {
    fn to_string(&self) -> String {
        let mut str_repr = String::new();
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                str_repr.push_str(if self.matrix[i][j] { "o" } else { "-" });
            }
            str_repr.push_str("\n");
        }

        str_repr
    }
}

fn clear_screen() {
    print!("\x1B[2J");
}

fn main() {
    let mut universe = Universe::new_random();

    loop {
        clear_screen();
        println!("{}", universe.to_string());
        println!("Epoch: {}", universe.epoch);
        {
            let living_cells = universe.living_cells();
            let total_size = WIDTH * HEIGHT;
            println!("Living cells: {} ({:.2} %)", living_cells,
                (living_cells as f32 / total_size as f32) * 100.);
        }
        thread::sleep_ms(100);
        universe.step();
    }
}
