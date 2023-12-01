advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                s.chars().filter(|c| c.is_digit(10)).collect()
            })
            .collect::<Vec<String>>()
            .iter()
            .map(|s| {
                s.chars().next().unwrap().to_digit(10).unwrap() * 10 + s.chars().last().unwrap().to_digit(10).unwrap()
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
