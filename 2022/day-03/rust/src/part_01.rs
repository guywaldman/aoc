use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

type Priority = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Item(pub(crate) char);

static PRIORITY_MAP: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, chr)| (chr, i + 1))
        .collect()
});

impl Item {
    pub(crate) fn priority(&self) -> Priority {
        PRIORITY_MAP[&self.0] as Priority
    }
}

pub(crate) struct Rucksack {
    compartment_1: HashSet<Item>,
    compartment_2: HashSet<Item>,
}

impl Rucksack {
    pub(crate) fn new(contents: &str) -> Self {
        let rucksack_items: Vec<Item> = contents.chars().map(|x| Item(x)).collect();
        let mut compartments = rucksack_items.chunks(rucksack_items.len() / 2);
        let compartment_1 = compartments.nth(0).unwrap().into_iter().cloned().collect();
        let compartment_2 = compartments.nth(0).unwrap().into_iter().cloned().collect();
        Rucksack {
            compartment_1,
            compartment_2,
        }
    }

    fn get_first_shared_item(&self) -> Option<Item> {
        for item in self.compartment_1.iter() {
            if self.compartment_2.contains(item) {
                return Some(*item);
            }
        }
        None
    }
}

pub fn solve_part_01<'a>(input: &'a str) -> String {
    input
        .lines()
        .filter_map(|line| {
            Rucksack::new(line)
                .get_first_shared_item()
                .and_then(|item| Some(item.priority() as usize))
        })
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
        assert_eq!(solve_part_01(input), "157");
    }

    #[test]
    fn shared_item() {
        assert_eq!(Rucksack::new("abcXYZ").get_first_shared_item(), None);
        assert_eq!(
            Rucksack::new("abcdXYZa").get_first_shared_item(),
            Some(Item('a'))
        );
        assert_eq!(
            Rucksack::new("abcdXYZd").get_first_shared_item(),
            Some(Item('d'))
        );
    }

    #[test]
    fn can_initialize_rucksack() {
        let input = "abcXYZ";
        let rucksack = Rucksack::new(input);
        assert_eq!(
            rucksack.compartment_1,
            vec![Item('a'), Item('b'), Item('c')].into_iter().collect()
        );
        assert_eq!(
            rucksack.compartment_2,
            vec![Item('X'), Item('Y'), Item('Z')].into_iter().collect()
        );
    }
}
