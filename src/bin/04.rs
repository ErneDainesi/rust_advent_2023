advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    input.lines().for_each(|l| {
        let line = l.split(':').collect::<Vec<&str>>()[1];
        let numbers = line.split('|').collect::<Vec<&str>>();
        let w_nums = numbers[0].split(' ').collect::<Vec<&str>>();
        let my_nums = numbers[1].split(' ').collect::<Vec<&str>>();
        let res: Vec<u32> = my_nums
            .iter()
            .filter(|n| w_nums.contains(n))
            .map(|n| n.parse::<u32>().unwrap_or(0))
            .filter(|n| *n != 0)
            .collect();
        let mut power: u32 = 0;
        let base: u32 = 2;
        let mut partial = 0;
        res.iter().for_each(|_| {
            partial = base.pow(power);
            power += 1;
        });
        result += partial;
    });
    Some(result)
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
