extern crate elapsed;
use elapsed::measure_time;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    vec![day1].iter().for_each(|f| {
        let (elapsed, _) = measure_time(f);
        println!("It took {}\n", elapsed)
    });
}

fn get_lines(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).expect("file not found")).lines().map(|x| x.unwrap()).collect()
}

fn day1() {
    println!("Executing day 1..");

    let lines = get_lines("./src/input1.txt");
    let mut frequencies = HashSet::new();
    let mut double_found = false;
    let mut first = true;
    let mut pers_acc = 0;

    while !double_found {
        pers_acc = lines.iter().fold(pers_acc, |acc, line| {
            if !double_found && !frequencies.insert(acc) {
                println!("Double frequency: {}", acc);
                double_found = true;
            }
            let num = line[1..].parse::<i32>().unwrap();
            return match &line[..1] {
                "+" => acc + num,
                _ => acc - num,
            };
        });

        if first {
            println!("result: {}", pers_acc);
            first = false;
        }
    }
}
