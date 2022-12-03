pub fn solve_part_01<'a>(input: &'a str) -> String {
    let max_carried_per_elf: usize = input
        .split("\n\n")
        .map(|txt| {
            txt.lines()
                .map(|weight| weight.trim().parse::<usize>().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap();

    max_carried_per_elf.to_string()
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
        assert_eq!(solve_part_01(input), "24000");
    }
}
