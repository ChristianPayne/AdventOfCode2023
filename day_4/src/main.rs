use substring::Substring;

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input, (5,8),(10,39), (42,116));
    println!("Part 1: {}", result);
    let input = include_str!("./input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}

#[derive(Debug)]
struct Card {
    card_number: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
    score: i32
}

fn part_1(input: &str, card_number_index: (usize, usize), winning_numbers_index: (usize, usize), my_numbers_index: (usize, usize)) -> i32 {
    let cards_input = input.split('\n');

    let cards: Vec<Card> = cards_input.map(|card_input| {
        let card_number = card_input
            .substring(card_number_index.0, card_number_index.1)
            .trim()
            .parse::<i32>().expect("Failed to parse card number");
        let winning_numbers: Vec<i32> = card_input
            .substring(winning_numbers_index.0, winning_numbers_index.1)
            .split(' ')
            .filter_map(|x| {
                match x.trim().parse::<i32>() {
                    Ok(val) => Some(val),
                    Err(_) => None
                }
            })
            .collect();
        let my_numbers: Vec<i32> = card_input
            .substring(my_numbers_index.0, my_numbers_index.1)
            .split(' ')
            .filter_map(|x| {
                match x.trim().parse::<i32>() {
                    Ok(val) => Some(val),
                    Err(_) => None
                }
            })
            .collect();

        let my_winning_numbers: Vec<&i32> = winning_numbers.iter().filter(|winning| {
            my_numbers.contains(&winning)
        }).collect();

        let score: i32 = match my_winning_numbers.len() {
            0 => 0,
            1 => 1,
            value => i32::from(2).pow(u32::try_from(value - 1).expect("Failed to convert winning numbers length."))
        };

        Card {
            card_number,
            my_numbers,
            winning_numbers,
            score
        }
    }).collect();

    // dbg!(&cards);

    cards.iter().map(|card| card.score).sum()
}

fn part_2(input: &str) -> i32 {
   1
}


#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = include_str!("./part_1_test_input.txt");
        let result = part_1(input,(5,6),(8,22),(25,48));
        assert_eq!(result, 13);
    }

    // #[test]
    fn part_2_test() {
        let input = include_str!("./part_2_test_input.txt");
        let result = part_2(input);
        assert_eq!(result, 467835);
    }
}