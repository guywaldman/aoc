pub fn solve_part_02<'a>(input: &'a str) -> String {
    let mut carried_per_elf: Vec<usize> = input
        .split("\n\n")
        .map(|txt| {
            txt.lines()
                .map(|weight| weight.trim().parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect();

    carried_per_elf.sort_by(|a, b| b.cmp(a));

    let sum_max_elves = dbg!(&carried_per_elf[0..3]).iter().sum::<usize>();
    sum_max_elves.to_string()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part_01() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(solve_part_02(input), "45000");
    }
}
