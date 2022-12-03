fn main() {
    let lines = include_str!("input/day2.txt").split('\n').map(|line| {
        let mut chars = line.chars();
        let col1 = chars.next().unwrap() as u32 - b'A' as u32;
        let col2 = chars.nth(1).unwrap() as u32 - b'X' as u32;
        (col1, col2)
    });

    let part1 = lines
        .clone()
        .map(|(opp, you)| {
            (you + 1)
                + (opp == you) as u32 * 3
                + (opp != you && (opp + 1) % 3 == you) as u32 * 6
        })
        .sum::<u32>();
    println!("1. {part1}");

    let part2 = lines
        .map(|(opp, outcome)| {
            let add = opp + outcome;
            let case_0_or_4 = (add ^ 1) & (add >> 1 ^ 1);
            let case_0 = case_0_or_4 & (add >> 2 ^ 1);
            let you = case_0_or_4 + case_0 * 2 + (case_0_or_4 ^ 1) * add;
            you + outcome * 3
        })
        .sum::<u32>();
    println!("1. {part2}");
}
