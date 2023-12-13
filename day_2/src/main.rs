use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
    let input = include_str!("./input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}

#[derive(Debug)]
struct Game {
    valid: GameValid,
    id: u32,
    plays: Vec<Play>
}

#[derive(Debug)]
enum GameValid {
    Possible,
    Impossible
}

#[derive(Debug)]
struct Play {
    red: u8,
    green: u8,
    blue: u8,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: 0,
            valid: GameValid::Possible,
            plays: Vec::new()
        }
    }
}

impl Default for Play {
    fn default() -> Self {
        Play {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}


fn part_1(input: &str) -> u32 {
    const CONFIGURATION: (u8,u8,u8) = (12,13,14);
    let lines: Vec<&str> = input.split('\n').collect();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let game_sections: Vec<&str> = line.split(':').collect();
        let mut game: Game = Game::default();

        // Try to parse the game number from the first part
        if let Some(game_number) = game_sections[0].trim().strip_prefix("Game") {
            if let Ok(game_number) = game_number.trim().parse::<u32>() {
                game.id = game_number;
            } else {
                println!("Invalid game number format");
            }
        } else {
            println!("Invalid game number format");
        }

        let plays: Vec<&str> = game_sections[1].split(';').map(|x| x.trim()).collect();
        
        for roll in plays {
            let mut play = Play {
                red: 0,
                green: 0,
                blue: 0,
            };
            let cube_move: Vec<&str> = roll.split(',').map(|x| x.trim()).collect();
            for data in cube_move {
                let details: Vec<&str> = data.split(' ').map(|x| x.trim()).collect();
                let amount = details[0].parse::<u8>().expect("Can not parse amount of cubes.");
                let color = details[1];

                if color == "red" {
                    play.red = amount;
                } else if color == "green" {
                    play.green = amount;
                } else if color == "blue" {
                    play.blue = amount;
                }
            }

            // Check if this play invalidates the game.
            if play.red > CONFIGURATION.0 || play.green > CONFIGURATION.1 || play.blue > CONFIGURATION.2 {
                game.valid = GameValid::Impossible;
                break;
            }

            game.plays.push(play);

        }
        games.push(game);
    }

    dbg!(&games);
    
    games.iter().map(|game| match game.valid {
        GameValid::Possible => game.id,
        GameValid::Impossible => 0
    }).sum::<u32>()
}


fn part_2(input: &str) -> u32 {
    const CONFIGURATION: (u8,u8,u8) = (12,13,14);
    let lines: Vec<&str> = input.split('\n').collect();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let game_sections: Vec<&str> = line.split(':').collect();
        let mut game: Game = Game::default();

        // Try to parse the game number from the first part
        if let Some(game_number) = game_sections[0].trim().strip_prefix("Game") {
            if let Ok(game_number) = game_number.trim().parse::<u32>() {
                game.id = game_number;
            } else {
                println!("Invalid game number format");
            }
        } else {
            println!("Invalid game number format");
        }
        
        let plays: Vec<&str> = game_sections[1].split(';').map(|x| x.trim()).collect();
        
        for roll in plays {
            let mut play = Play {
                red: 0,
                green: 0,
                blue: 0,
            };
            let cube_move: Vec<&str> = roll.split(',').map(|x| x.trim()).collect();
            for data in cube_move {
                let details: Vec<&str> = data.split(' ').map(|x| x.trim()).collect();
                let amount = details[0].parse::<u8>().expect("Can not parse amount of cubes.");
                let color = details[1];

                if color == "red" {
                    play.red = amount;
                } else if color == "green" {
                    play.green = amount;
                } else if color == "blue" {
                    play.blue = amount;
                }
            }

            // Check if this play invalidates the game.
            // if play.red > CONFIGURATION.0 || play.green > CONFIGURATION.1 || play.blue > CONFIGURATION.2 {
            //     game.valid = GameValid::Impossible;
            //     break;
            // }

            game.plays.push(play);

        }
        games.push(game);
    }

    // dbg!(&games);
    
    games.iter().map(|game| {
        let mut max: [u8; 3] = [0,0,0];
        for play in game.plays.iter() {
            if play.red > max[0] {
                max[0] = play.red;
            }
            if play.green > max[1] {
                max[1] = play.green;
            }
            if play.blue > max[2] {
                max[2] = play.blue;
            }
        };
        max.iter().fold(1, |acc: u32, x| acc * x.clone() as u32 )
    }).map(|x| u32::from(x)).sum()
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = include_str!("./part_1_test_input.txt");
        let result = part_1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("./part_2_test_input.txt");
        let result = part_2(input);
        assert_eq!(result, 2286);
    }
}