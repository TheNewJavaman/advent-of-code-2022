use itertools::Itertools;

fn main() {
    let elf_sums = include_str!("input/day1.txt")
        .split("\n\n")
        .map(|goodies| {
            goodies
                .split('\n')
                .map(|snack| snack.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev();

    let part1 = elf_sums.clone().max().unwrap();
    println!("1. {part1}");

    let part2: u32 = elf_sums.take(3).sum();
    println!("2. {part2}");
}
