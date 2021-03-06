use crate::runner::{Parse, RunMut};

use anyhow::Result;

pub struct Day6 {}

impl Parse<Vec<usize>> for Day6 {
    fn parse_input(input: &str) -> Result<Vec<usize>> {
        Ok(input
            .trim()
            .split(',')
            .filter_map(|n| n.parse::<usize>().ok())
            .fold([0; 9], |mut map, n| {
                map[n] += 1;
                map
            })
            .to_vec())
    }
}

impl RunMut<Vec<usize>, usize> for Day6 {
    fn part_one(input: &mut Vec<usize>) -> Result<usize> {
        Ok(count_fish(input, 80))
    }

    fn part_two(input: &mut Vec<usize>) -> Result<usize> {
        Ok(count_fish(input, 256))
    }
}

// First attemp, not efficient enough for part 2
#[allow(dead_code)]
fn change_fish(input: &mut Vec<u16>, days: u16) {
    (0..days).for_each(|_| {
        let mut new_fishes = Vec::new();
        for fish in input.iter_mut() {
            match fish {
                0 => {
                    *fish = 6;
                    new_fishes.push(8);
                }
                _ => *fish -= 1,
            }
        }
        if !new_fishes.is_empty() {
            input.append(&mut new_fishes);
        }
    });
}

fn count_fish(input: &mut [usize], days: usize) -> usize {
    (0..days).for_each(|_| {
        // Move each count to one less day
        // Making the fish at 0 becoming 8
        input.rotate_left(1);
        // Acumulate the fish that were on 0, now on 8 to 6
        input[6] += input[8];
    });

    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day6.test");

    #[test]
    fn test_part_one() -> Result<()> {
        let mut input = Day6::parse_input(INPUT)?;
        let result = Day6::part_one(&mut input)?;
        assert_eq!(result, 5934);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = Day6::parse_input(INPUT)?;
        let result = Day6::part_two(&mut input)?;
        assert_eq!(result, 26984457539);
        Ok(())
    }

    #[test]
    fn test_count_fish() -> Result<()> {
        let mut input = Day6::parse_input(INPUT)?;
        let count = count_fish(&mut input, 2);
        assert_eq!(count, 6);
        let mut input = Day6::parse_input(INPUT)?;
        let count = count_fish(&mut input, 3);
        assert_eq!(count, 7);
        let mut input = Day6::parse_input(INPUT)?;
        let count = count_fish(&mut input, 18);
        assert_eq!(count, 26);
        Ok(())
    }
}
