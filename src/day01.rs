use crate::helpers;

fn load_combinations_from_file(filename: &str) -> Vec<String> {
    let mut combinations = Vec::new();
    if let Ok(lines) = helpers::read_lines(filename) {
        for line in lines {
            if let Ok(combination) = line {
                combinations.push(combination);
            }
        }
    }
    combinations
}

fn turn_dial(current_position: &i32, combo: &str) -> i32 {
    let mut new_position = *current_position;
    let direction = combo.chars().nth(0).unwrap();
    let mut clicks: i32 = combo[1..].parse().unwrap();
    clicks = clicks % 100;
    match direction {
        'L' => {
            new_position -= clicks;
            if new_position < 0 {
                new_position = 100 - new_position.abs();
            }
        }
        'R' => {
            new_position += clicks;
            if new_position > 99 {
                new_position = new_position - 100;
            }
        }
        _ => {}
    }
    new_position
}

fn turn_dial_2(current_position: &i32, combo: &str) -> (i32, i32) {
    let mut new_position = *current_position;
    let direction = combo.chars().nth(0).unwrap();
    let mut clicks: i32 = combo[1..].parse().unwrap();
    let mut crossing_count = clicks / 100;
    clicks = clicks % 100;
    match direction {
        'L' => {
            new_position -= clicks;
            if new_position < 0 {
                new_position = 100 - new_position.abs();
                if new_position != 0 && *current_position != 0 {
                    crossing_count += 1;
                }
            }
        }
        'R' => {
            new_position += clicks;
            if new_position > 99 {
                new_position = new_position - 100;
                if new_position != 0 && *current_position != 0 {
                    crossing_count += 1;
                }
            }
        }
        _ => {}
    }
    (new_position, crossing_count)
}

fn find_zeros(starting_point: i32, combinations: &Vec<String>) -> i32 {
    let mut count = 0;
    let mut position = starting_point;
    for combo in combinations {
        position = turn_dial(&position, combo);
        if position == 0 {
            count += 1;
        }
    }
    count
}

fn find_zeros_2(starting_point: i32, combinations: &Vec<String>) -> i32 {
    let mut count = 0;
    let mut position = starting_point;
    for combo in combinations {
        let result = turn_dial_2(&position, combo);
        if result.0 == 0 {
            count += 1;
        }
        count += result.1;
        position = result.0;
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_dial() {
        let mut position = 0;
        position = turn_dial(&position, "R10");
        assert_eq!(position, 10);
        position = turn_dial(&position, "L20");
        assert_eq!(position, 90);
        position = turn_dial(&position, "R15");
        assert_eq!(position, 5);
        position = turn_dial(&position, "L10");
        assert_eq!(position, 95);
    }

    #[test]
    fn test_day01_simple() {
        // Example test case for day01 module
        let combinations = load_combinations_from_file("./src/resources/day01_simple.txt");
        let zeros = find_zeros(50, &combinations);
        assert_eq!( zeros, 3);
    }

    #[test]
    fn test_day01_2_simple() {
        // Example test case for day01 module
        let combinations = load_combinations_from_file("./src/resources/day01_simple.txt");
        let zeros = find_zeros_2(50, &combinations);
        assert_eq!( zeros, 6);
    }


    #[test]
    fn test_turn_dial_2() {
        let res = turn_dial_2(&0, "R150");
        assert_eq!(res, (50, 1));
    }

    #[test]
    fn test_day01() {
        // Example test case for day01 module
        let combinations = load_combinations_from_file("./src/resources/day01_input.txt");
        let zeros = find_zeros(50, &combinations);
        println!("Number of zeros: {}", zeros);
        // assert_eq!( zeros, 3);
    }    

    #[test]
    fn test_day01_part2() {
        // Example test case for day01 module
        let combinations = load_combinations_from_file("./src/resources/day01_input.txt");
        let zeros = find_zeros_2(50, &combinations);
        println!("Number of zeros: {}", zeros);
        // assert_eq!( zeros, 3);
    }    

    
    #[test]
    fn test_modulo() {
        let value = -43 % 100;
        assert_eq!(value, -43);
    }
}