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
    let games = parse_games(input);
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
    let games = parse_games(input);
    let mut power_sum = 0;
    games.iter().for_each(|g| {
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        g.sets.iter().for_each(|s| {
            if s.blue > max_blue {
                max_blue = s.blue;
            }
            if s.green > max_green {
                max_green = s.green;
            }
            if s.red > max_red {
                max_red = s.red;
            }
        });
        power_sum += max_red * max_blue * max_green;
    });
    Some(power_sum)
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut games = vec![];
    for (i, line) in input.lines().enumerate() {
        let line = line.split(':').collect::<Vec<&str>>()[1];
        let mut game = Game {id: i + 1, sets: vec![]};
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
    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2176));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(63700));
    }
}
