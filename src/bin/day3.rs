use itertools::Itertools;

fn main() {
    let lines = include_str!("input/day3.txt").split('\n');

    let part1: u32 = lines
        .clone()
        .map(|line| {
            let mut chars = line.chars();
            let half = chars.by_ref().take(line.len() / 2).collect::<Vec<_>>();
            let item = chars.find(|item| half.contains(item)).unwrap() as u32;
            item - 38 - (item >> 5 & 1) * 58
        })
        .sum();
    println!("1. {part1}");

    let part2: u32 = lines
        .tuples::<(&str, &str, &str)>()
        .map(|(elf1, elf2, elf3)| {
            let item = elf1
                .chars()
                .find(|&item| elf2.contains(item) && elf3.contains(item))
                .unwrap() as u32;
            item - 38 - (item >> 5 & 1) * 58
        })
        .sum();
    println!("2. {part2}");
}
