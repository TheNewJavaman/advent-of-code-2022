use itertools::Itertools;

fn main() {
    let input = include_str!("input/day1.txt");
    let elf_sums = input
        .split("\n\n")
        .map(|goodies| {
            goodies
                .split('\n')
                .map(|snack| snack.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev();

    let top1 = elf_sums.clone().max().unwrap();
    println!("1. {top1}");

    let top3: u32 = elf_sums.take(3).sum();
    println!("2. {top3}");
}
