fn main() {
    let input = include_str!("./input.txt").split('\n').collect();
    let result = part_1(input);
    println!("Part 1: {}", result);
    let input = include_str!("./input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}

fn part_1(input: Vec<&str>) -> i32 {
    let mut total = 0;
    for line in input {
        let stripped_line: String = strip_letters(line);
        let val_1 = stripped_line.chars().nth(0).unwrap();
        let val_2 = stripped_line.chars().nth(stripped_line.len() - 1).unwrap();
        let parsed_line_value: i32 = format!("{}{}", val_1, val_2).parse().unwrap();

        total = total + parsed_line_value;
    }
    total
}

fn strip_letters(input: &str) -> String {
    input.chars().filter(|&c| c.is_numeric()).collect()
}

fn part_2(input: &str) -> u32 {
    let output =
        input.lines().map(replace_words_with_numbers).sum::<u32>();

    output
}

fn replace_words_with_numbers (line: &str) -> u32 {
    // Solution given from https://github.com/ChristopherBiscardi/advent-of-code/tree/76c5ca80795336e465c1272d99147a069162de56/2023/rust/day-01
    // Needed help on a rust solution.
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

fn check_for_replacements(input: String, string_to_check: &str, replacement: &str) -> String {
    if input.starts_with(string_to_check) {
        return input.replacen(string_to_check, replacement, 1)
    }
    input.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input: Vec<&str> = include_str!("./part_1_test_input.txt").split('\n').collect();
        let result = part_1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("./part_2_test_input.txt");
        let result = part_2(input);
        assert_eq!(result, 281);
    }
}