advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let positions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (-1, 1), (-1, -1), (1, 0), (1, 1), (1, -1)];
    let mut numbers_found = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut current_num = String::from("");
        let mut has_adyacent = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current_num.push(c);
                for pos in positions.iter() {
                    let position = ((i as i32) + pos.0, ((j as i32) + pos.1));
                    let grid: Vec<&str> = input.lines().collect();
                    // position out of bounds
                    if position.0 < 0 || position.0 > (grid.len() - 1) as i32 || position.1 < 0 || position.1 > (grid[position.0 as usize].len() - 1) as i32 {
                        continue;
                    }
                    let element = grid[position.0 as usize].as_bytes()[position.1 as usize] as char;
                    if element != '.' && !element.is_digit(10) {
                        has_adyacent = true;
                    }
                }
            }
            if !c.is_digit(10) || j == line.len() - 1 {
                if has_adyacent {
                    numbers_found.push(current_num.parse::<u32>().unwrap());
                }
                current_num.clear();
                has_adyacent = false;
            }
        }
    }
    numbers_found.iter().sum::<u32>().into()
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
        assert_eq!(result, Some(540025));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
