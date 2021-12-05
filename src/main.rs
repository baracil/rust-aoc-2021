use crate::days::day_04::main::day04_launch;
use crate::days::day_05::main::day05_launch;
use crate::problem::Part;

mod problem;
mod domain;
mod days;
mod macros;
pub mod tools;


#[warn(dead_code)]
fn main() {
    let launch = day05_launch;

    println!("part 1 : {:?}",launch(Part::Part1));
    println!("part 2 : {:?}",launch(Part::Part2));
}
