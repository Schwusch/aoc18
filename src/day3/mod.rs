use super::utils::*;
use itertools::Itertools;
use collision::{Aabb2};
use collision::{Discrete};
use cgmath::{Point2};

pub fn day3() {    
    println!("Executing day 3..");
    let lines = get_lines("./src/day3/input.txt");
    let parts = lines.iter().map(|elem| 
        elem.as_str()
            .split_whitespace()
            .map(|x| x.to_string())
            .dropping(2)
            .collect_vec()
        ).collect::<Vec<Vec<String>>>();

    let patty = parts.iter().map(|x| {
        let coords = x.first().unwrap()
            .chars()
            .dropping_back(1)
            .collect::<String>()
            .split(',')
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect_vec();
        let dimens = x.last().unwrap()
            .split('x')
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect_vec();
        ((coords.first().unwrap().clone(), coords.last().unwrap().clone()),
            (dimens.first().unwrap().clone(), dimens.last().unwrap().clone()))
    }).collect_vec();

    let mut matrix = vec![vec![0i32; 1001]; 1001];

    patty.iter()
        .for_each(|instr| {
            let x_offs = (instr.0).0;
            let y_offs = (instr.0).1;
            for x in x_offs + 1..=x_offs + (instr.1).0 {
                for y in y_offs + 1..=y_offs + (instr.1).1 {
                    matrix[x][y] = matrix[x][y] + 1; 
                }
            } 
        });

    println!("part one: {}", matrix.into_iter().kmerge().filter(|x| *x > 1).count());

    let mut rects = patty
        .iter()
        .map(|woop| {
            Aabb2::new(
                Point2::new((woop.0).0, (woop.0).1), 
                Point2::new((woop.0).0 + (woop.1).0, (woop.0).1 + (woop.1).1))
        }).collect_vec();

    let mut count = 0;
    while let Some((first, rest)) = rects.split_first() {
        count = count + 1;
        if rest.iter().find_position(|rec| {
            let (a0, a1) = (rec.min, rec.max);
            let (b0, b1) = (first.min, first.max);

            a1.x > b0.x && a0.x < b1.x && a1.y > b0.y && a0.y < b1.y
        }).is_none() {
            println!("part two: {}", count);
            break;
        }
        rects = rest.to_vec();
    }
}