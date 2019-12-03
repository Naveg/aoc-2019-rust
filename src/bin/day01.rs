use std::io::{self, BufRead};

fn main() {
    let mut fuel1 = 0;
    let mut fuel2 = 0;

    for line in io::stdin().lock().lines() {
        let mass = line.ok().map_or(0, |s| s.parse().unwrap_or(0));
        let mut fuel = mass / 3 - 2;
        fuel1 += fuel;
        while fuel > 0 {
            fuel2 += fuel;
            fuel = fuel / 3 - 2;
        }
    }
    println!("{}", fuel1);
    println!("{}", fuel2);
}
