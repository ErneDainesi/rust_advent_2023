advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.chars().filter(|c| c.is_numeric()).collect())
            .collect::<Vec<String>>()
            .iter_mut()
            .map(|s| {
                if s.len() > 2 {
                    let new_value = format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap());
                    *s = new_value;
                } else if s.len() == 1 {
                    let first_char: char = s.chars().next().unwrap();
                    let new_value = format!("{}{}", first_char, first_char);
                    *s = new_value;
                }
                return s.parse::<u32>().unwrap();
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
