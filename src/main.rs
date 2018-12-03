extern crate elapsed;
extern crate strsim;
extern crate itertools;

mod utils;
mod day1;
mod day2;

use elapsed::measure_time;
use self::day1::*;
use self::day2::*;

fn main() {
    [day1, day2].iter().for_each(|f| {
        let (elapsed, _) = measure_time(f);
        println!("It took {}\n", elapsed)
    });
}




