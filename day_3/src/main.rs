fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input);
    // 6261125 too high
    // 6391179 too high
    println!("Part 1: {}", result);
    // let input = include_str!("./input.txt");
    // let result = part_2(input);
    // println!("Part 2: {}", result);
}

enum XAxis {
    Left,
    Center,
    Right,
}
enum YAxis {
    Up,
    Center,
    Down,
}

fn part_1(input: &str) -> i32 {
    let line_length = input.find('\n').expect("Could not find a new line.");
    let line_length: i32 = line_length.try_into().expect("Error converting width.");
    let line_length: i32 = line_length - 1;
    dbg!(&line_length);
    let input_char_length = input.len();
    let input = input.replace('\n', "");

    let mut answer = 0;

    let mut number_buffer: String = String::new();
    let mut number_has_char_around: bool = false;


    for (index, char_val) in input.chars().enumerate() {
        if char_val.is_digit(10) == false {
            if number_buffer.len() > 0 {
                let parsed_number = number_buffer.parse::<i32>().expect("Failed to parse int.");

                if number_has_char_around == true {
                    println!("{} + {} = {}", &parsed_number,  &answer, &answer + &parsed_number);
                    answer = answer + parsed_number;
                } else {
                    println!("{} did not have chars around it.", &parsed_number);
                }

                number_buffer = String::new();
                number_has_char_around = false;
            }
            continue;
        } else {
            number_buffer.push_str(char_val.to_string().as_str());
            // dbg!(&number_buffer);
            if check_all_positions_around_char(&input, index.try_into().expect("Error converting usize."), line_length).len() > 0 {
                number_has_char_around = true;
            }
        }
    }
    answer
}

fn part_2(input: &str) -> i32 {
    let line_length = input.find('\n').expect("Could not find a new line.");
    let line_length: i32 = line_length.try_into().expect("Error converting width.");
    let line_length: i32 = line_length - 1;
    dbg!(&line_length);
    let input_char_length = input.len();
    let input = input.replace('\n', "");

    let mut answer = 0;

    let mut number_buffer: String = String::new();
    let mut number_has_char_around: bool = false;

    for x in 0..input_char_length {
        if let Some(val) = &input.chars().nth(x) {
            if val.is_digit(10) == false {
                if val.clone() == '*' {

                }
                
                if number_buffer.len() > 0 {
                    let parsed_number = number_buffer.parse::<i32>().expect("Failed to parse int.");

                    if number_has_char_around == true {
                        println!("{} + {} = {}", &parsed_number,  &answer, &answer + &parsed_number);
                        answer = answer + parsed_number;
                    } else {
                        println!("{} did not have chars around it.", &parsed_number);
                    }

                    number_buffer = String::new();
                    number_has_char_around = false;
                }
                continue;
            } else {
                number_buffer.push_str(val.to_string().as_str());
                // dbg!(&number_buffer);
                if check_all_positions_around_char(&input, x.try_into().expect("Error converting usize."), line_length).len() > 0 {
                    number_has_char_around = true;
                }
            }
        } 
    }
    answer
}


fn check_all_positions_around_char(input: &String, index: i32, line_length: i32) -> Vec<char> {
    let places_to_check: [(XAxis, YAxis); 8] = [
        (XAxis::Left, YAxis::Up),
        (XAxis::Center, YAxis::Up),
        (XAxis::Right, YAxis::Up),
        (XAxis::Left, YAxis::Center),
        (XAxis::Right, YAxis::Center),
        (XAxis::Left, YAxis::Down),
        (XAxis::Center, YAxis::Down),
        (XAxis::Right, YAxis::Down),
    ];

    let mut adjacent_chars: Vec<char> = vec![];

    for place_to_check in places_to_check {
        match get_char_at_coord(
            input.clone(), 
            index.try_into().expect("Failed to convert usize into int."), 
            line_length, 
            place_to_check.0, 
            place_to_check.1
        ) {
            None => continue,
            Some(found_char) => {
                if found_char.is_digit(10) == false && found_char.clone() != '.' {
                    // println!("Found the symbol char: {}", &found_char);
                    adjacent_chars.push(found_char)
                } else {
                    // println!("Char is not a number: {}", &found_char);
                }
            },
        }
    }

    adjacent_chars
}

fn get_char_at_coord (input: String, index: i32, line_length: i32, x_axis: XAxis, y_axis: YAxis) -> Option<char> {
    let x_offset: i32 = match x_axis {
        XAxis::Left => -1,
        XAxis::Center => 0,
        XAxis::Right => 1
    };
    let y_offset: i32 = match y_axis {
        YAxis::Up => -1,
        YAxis::Center => 0,
        YAxis::Down => 1
    };

    let index_offset = x_offset + (y_offset * line_length + y_offset);

    // dbg!(&index, &index_offset);

    let final_index = match (index + index_offset).try_into() {
        Ok(val) => val,
        Err(_) => return None
    };

    input.chars().nth(final_index)
}


#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = include_str!("./part_1_test_input.txt");
        let result = part_1(input);
        assert_eq!(result, 4361);
    }

    // #[test]
    fn part_2_test() {
        let input = include_str!("./part_2_test_input.txt");
        let result = part_2(input);
        assert_eq!(result, 467835);
    }

    #[test]
    fn recursive_test() {
        fn fib (n: i64) -> i64 {
            dbg!(&n);
            match n {
                0 => 0,
                1 => 1,
                _ => fib(n - 1) + fib(n - 2)
            }
        }
        
        // A tail recursion implementation of the fibonacci sequence.
        fn fib_tail(n: i64, a: i64, b: i64) -> i64 {
            dbg!(&n);
            match n {
                0 => a,
                1 => b,
                _ => fib_tail(n - 1, b, a + b)
            }
        }

        // dbg!(fib(60));
        // dbg!(fib_tail( 60, 0, 1 ));
    }
}