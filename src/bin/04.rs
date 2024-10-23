use std::collections::HashMap;

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
        if res.len() != 0 {
            result += 2_u32.pow((res.len() as u32) - 1);
        }
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_map: HashMap<usize, u32> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
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

        let card_num = i + 1;
        for mut x in 0..res.len() + 1 {
            x += card_num;
            if card_map.contains_key(&x) && x == card_num {
                *card_map.get_mut(&card_num).unwrap() += 1;
            } else if card_map.contains_key(&x) {
                let reps_card = *card_map.get_mut(&card_num).unwrap();
                *card_map.get_mut(&x).unwrap() += reps_card;
            } else {
                card_map.insert(x, 1);
            }
        }
    }

    println!("final card_map = {:?}", card_map);
    card_map.values().sum::<u32>().into()
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
