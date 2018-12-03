use super::utils::*;
use itertools::Itertools;
use strsim::levenshtein;

pub fn day2() {
    println!("Executing day 2..");
    let lines = get_lines("./src/day2/input.txt");
    let groups = lines
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

    let mut coll = &lines[..];

    while let Some((first, rest)) = coll.split_first() {
        if let Some((one, two)) = rest.into_iter()
            .find_map(|x|
                if levenshtein(first, x) == 1 {
                    Some((first, x))
                } else {
                    None
                }) {

            let union = one
                .chars()
                .zip(two.chars())
                .filter(|(m, n)| m == n)
                .map(|(m ,_)| m)
                .collect::<String>();
            println!("part two: {}", union);
            break;
        }

        coll = rest;
    }
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