advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
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
        .into()
}

#[derive(Debug)]
struct MatchingNumbers {
    pub idx: usize,
    pub num: usize
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut res: u32 = 0;
    for line in input.lines() {
        let mut idx_nums: Vec<MatchingNumbers> = vec![];
        for (i, num) in numbers.iter().enumerate() {
            for m in line.match_indices(num) {
                idx_nums.push(MatchingNumbers {
                    idx: m.0,
                    num: i + 1
                });
            }
        }
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                idx_nums.push(MatchingNumbers { idx: j, num: c.to_digit(10).unwrap() as usize })
            }
        }
        idx_nums.sort_by_key(|n| n.idx);
        if let (Some(first), Some(last)) = (idx_nums.first(), idx_nums.last()) {
            res += (first.num * 10 + last.num) as u32;
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57346));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(57345));
    }
}
