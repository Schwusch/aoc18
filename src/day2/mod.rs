use super::utils::*;
use itertools::Itertools;

pub fn day2() {
    println!("Executing day 2..");

    let groups = get_lines("./src/day2/input.txt")
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| (x, x))
                .into_group_map()
                .into_iter()
                .map(|(_, r)| r.into_iter().collect())
                .collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();

    println!("Part one: {}", count(&groups, 2) * count(&groups, 3));
}

fn count(group: &Vec<Vec<String>>, occ: usize) -> usize {
    return group
        .into_iter()
        .filter(|x|
            x.into_iter()
                .filter(|x| x.len() == occ)
                .count() > 0
        )
        .count();
}