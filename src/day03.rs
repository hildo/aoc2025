use crate::helpers;
use core::str;
use std::cmp::Reverse;

/*
  Integer -> Char 
        0 -> 48
        1 -> 49
        2 -> 50
        3 -> 51
        4 -> 52
        5 -> 53
        6 -> 54
        7 -> 55
        8 -> 56
        9 -> 57
 */
fn find_max_pair(input: &str) -> u32 {
    let bytes = input.as_bytes();

    // First loop.  Find the highest number, stopping at the index
    let mut tens_place_idx: usize = 0;
    let mut tens_place: char = 48 as char; // '0'
    for i in 0..bytes.len() - 1 {
        let current_char = bytes[i] as char;
        if current_char == '9' {
            tens_place_idx = i;
            tens_place = current_char;
            break;  
        }

        if current_char > tens_place {
            tens_place = current_char;
            tens_place_idx = i;
        }
    }

    // Second loop.  Find the highest number after the first highest number
    let mut ones_place: char = 48 as char; // '0'
    for j in tens_place_idx + 1..bytes.len() {
        let current_char = bytes[j] as char;
        if current_char == '9' {
            ones_place = current_char;
            break;  
        }    

        if current_char > ones_place {
            ones_place = current_char;
        }
    }
    tens_place.to_digit(10).unwrap() * 10 + ones_place.to_digit(10).unwrap()
}

fn parse_string_to_numbers(input: &str) -> Vec<u64> {
    input
        .chars()
        .into_iter()
        .map(|char| char.to_string().parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n")
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| parse_string_to_numbers(line))
        .collect()
}

fn compute_max_joltage(input: &Vec<u64>, length: usize) -> u64 {
    let mut joltage: u64 = 0;
    let mut position = 0;
    for digit_counter in (0..length).rev() {
        let (max_position, max_value) = input[position..input.len() - digit_counter]
            .iter()
            .enumerate()
            .min_by_key(|&(_, value)| Reverse(value))
            .unwrap();

        position += max_position + 1;
        joltage += (10 as u64).pow(digit_counter as u32) * max_value;
    }
    joltage
}

fn sum_joltages(input: &str, length: usize) -> u64 {
    parse_input(input)
        .iter()
        .map(|line| compute_max_joltage(line, length))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_pair() {
        assert_eq!(find_max_pair("12345678901234567890"), 99);   
        assert_eq!(find_max_pair("987654321111111"), 98);
        assert_eq!(find_max_pair("811111111111119"), 89);
        assert_eq!(find_max_pair("234234234234278"), 78);
        assert_eq!(find_max_pair("818181911112111"), 92);
    }

    #[test]
    fn test_simple_part_1() {
        if let Ok(lines) = helpers::read_lines("./src/resources/day03_simple.txt") {
            let mut total: u32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    total += find_max_pair(&ip);
                }
            }
            assert_eq!(total, 357);
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(lines) = helpers::read_lines("./src/resources/day03_input.txt") {
            let mut total: u32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    total += find_max_pair(&ip);
                }
            }
            assert_eq!(total, 17376);
        }
    }

    #[test]
    fn test_simple_part_2() {
        let input: &str = include_str!("../src/resources/day03_simple.txt");
        let sum = sum_joltages(input, 12);
        assert_eq!(sum, 3121910778619);
    }

    #[test]
    fn test_part_2() {
        let input: &str = include_str!("../src/resources/day03_input.txt");
        let sum = sum_joltages(input, 12);
        assert_eq!(sum, 172119830406258);
    }

}