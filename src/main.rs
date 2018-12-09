extern crate elapsed;
extern crate strsim;
extern crate itertools;
extern crate collision;

mod utils;
mod day1;
mod day2;
mod day3;

use elapsed::measure_time;
use self::day1::*;
use self::day2::*;
use self::day3::*;

fn main() {
    [day1, day2, day3].iter().for_each(|f| {
        let (elapsed, _) = measure_time(f);
        println!("It took {}\n", elapsed)
    });
}




