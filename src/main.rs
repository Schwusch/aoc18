extern crate elapsed;

mod utils;
mod day1;

use elapsed::measure_time;
use self::day1::*;

fn main() {
    vec![day1].iter().for_each(|f| {
        let (elapsed, _) = measure_time(f);
        println!("It took {}\n", elapsed)
    });
}




