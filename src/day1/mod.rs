use super::utils::*;
use std::collections::BTreeSet;

pub fn day1() {
    println!("Executing day 1..");
    let lines: Vec<i32> = get_lines("./src/day1/input1.txt").iter().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Part one: {}", lines.iter().sum::<i32>());
    let mut frequencies = BTreeSet::new();
    let mut curr_frequency = 0;

    let twin = lines.iter().cycle().find_map(|x| {
        curr_frequency = curr_frequency + x;
        frequencies.replace(curr_frequency)
    }).unwrap();
    println!("Part two: {}", twin);
}