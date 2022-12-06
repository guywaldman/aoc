type Priority = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item(char);

impl Item {
    fn priority(&self) -> Priority {
        match self.0 {
            'a'..='z' => self.0 as Priority - b'a' + 1,
            'A'..='Z' => self.0 as Priority - b'A' + 27,
            _ => 0,
        }
    }
}

struct Rucksack {
    compartment_1: Vec<Item>,
    compartment_2: Vec<Item>,
}

impl Rucksack {
    fn new(contents: &str) -> Self {
        let rucksack_items: Vec<Item> = contents.chars().map(|x| Item(x)).collect();
        let mut compartments = rucksack_items.chunks(rucksack_items.len() / 2);
        let compartment_1 = compartments.nth(0).unwrap();
        let compartment_2 = compartments.nth(0).unwrap();
        Rucksack {
            compartment_1: compartment_1.to_vec(),
            compartment_2: compartment_2.to_vec(),
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
            vec![Item('a'), Item('b'), Item('c')]
        );
        assert_eq!(
            rucksack.compartment_2,
            vec![Item('X'), Item('Y'), Item('Z')]
        );
    }
}
