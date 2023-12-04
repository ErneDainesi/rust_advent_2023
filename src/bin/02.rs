advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    pub id: usize,
    pub sets: Vec<GameSet>
}

#[derive(Debug)]
struct GameSet {
    pub blue: u32,
    pub green: u32,
    pub red: u32
}

impl GameSet {
    pub fn is_valid_set(&self) -> bool {
        let (max_red, max_green, max_blue) = (12, 13, 14);
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut games = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut game = Game {id: i + 1, sets: vec![]};
        let game_prefix = format!("Game {}: ", i + 1);
        let line = line.strip_prefix(&game_prefix).unwrap();
        for game_set in line.split(';').collect::<Vec<&str>>() {
            let mut set = GameSet {blue: 0, green: 0, red: 0};
            for dice in game_set.split(',').collect::<Vec<&str>>() {
                let dice_and_amount: Vec<&str> = dice.trim().split(' ').collect();
                let pair: (u32, &str) = (dice_and_amount[0].parse::<u32>().unwrap(), dice_and_amount[1]);
                match pair.1 {
                    "blue" => set.blue += pair.0,
                    "green" => set.green += pair.0,
                    "red" => set.red += pair.0,
                    _ => continue,
                }
            }
            game.sets.push(set);
        }
        games.push(game);
    }
    let mut res = 0;
    for game in games.iter() {
        let mut is_valid_set = true;
        for set in game.sets.iter() {
            if !set.is_valid_set() {
                is_valid_set = false;
            }
        }
        if is_valid_set {
            res += game.id;
        }
    }
    Some(res as u32)
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
