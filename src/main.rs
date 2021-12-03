mod day_one;
mod day_two;
use day_one::{solve_day_one_one, solve_day_one_two};
use day_two::{solve_day_two_one, solve_day_two_two};

fn main() {

    println!("Task for day one!");
    println!("Solving part one!");
    solve_day_one_one();

    println!("Solving part two!");

    solve_day_one_two();

    println!("Task for day two!");
    println!("Solving part one!");
    solve_day_two_one();

    println!("Solving part two!");
    solve_day_two_two();
}
