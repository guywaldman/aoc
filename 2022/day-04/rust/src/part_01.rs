use std::ops::{RangeBounds, RangeInclusive};

use nom::{
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};

fn parse_range(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, range_a_start) = map_res(recognize(digit1), str::parse)(input)?;
    let (input, _) = nom::character::complete::char('-')(input)?;
    let (input, range_a_end) = map_res(recognize(digit1), str::parse)(input)?;
    let (input, _) = nom::character::complete::char(',')(input)?;
    let (input, range_b_start) = map_res(recognize(digit1), str::parse)(input)?;
    let (input, _) = nom::character::complete::char('-')(input)?;
    let (input, range_b_end) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((
        input,
        (range_a_start..=range_a_end, range_b_start..=range_b_end),
    ))
}

fn parse_file(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| parse_range(line.trim()).unwrap().1)
        .collect()
}

pub fn solve_part_01<'a>(input: &'a str) -> String {
    parse_file(input)
        .into_iter()
        .filter_map(|(range_a, range_b)| {
            let is_either_range_contained = (range_a.start() >= range_b.start() && range_a.end() <= range_b.end())
                || (range_b.start() >= range_a.start() && range_b.end() <= range_a.end());
            is_either_range_contained.then_some(())
        })
        .count()
        .to_string()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_input() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        assert_eq!(solve_part_01(input), "2");
    }
}
