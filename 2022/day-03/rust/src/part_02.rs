use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

use crate::part_01::{Item, Rucksack};

pub fn solve_part_02<'a>(input: &'a str) -> String {
    let common_items: Vec<Item> = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let mut char_occurrence_map = HashMap::new();
            for (i, line) in lines.enumerate() {
                let bit_set_mask = match i {
                    0 => 0b100,
                    1 => 0b010,
                    2 => 0b001,
                    _ => panic!("Unexpected index in group"),
                };
                for chr in line.chars() {
                    *char_occurrence_map.entry(chr).or_insert(0) |= bit_set_mask;
                }
            }
            let common_char = char_occurrence_map
                .into_iter()
                .find(|(_, count)| *count == 0b111)
                .unwrap();
            Item(common_char.0)
        })
        .collect();
    common_items
        .iter()
        .map(|item| item.priority())
        .sum::<usize>()
        .to_string()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_part_02(input), "70");
    }
}
