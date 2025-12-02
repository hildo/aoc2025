use crate::helpers;

fn is_invalid(input: &str) -> bool {
    if input.len() % 2 != 0 {
        return false;
    }

    let substr_len = input.len() / 2;
    let first_half = &input[0..substr_len].parse::<u32>().unwrap();
    let second_half = &input[substr_len..].parse::<u32>().unwrap();

    return first_half == second_half;
}

fn load_ranges(input_file: &str) -> Vec<String> {
    let mut ranges = Vec::new();
    if let Ok(lines) = helpers::read_lines(input_file) {
        for line in lines {
            if let Ok(range) = line {
                let range_parts: Vec<&str> = range.split(',').collect();
                for part in range_parts {
                    let detailed_range = part.split('-').collect::<Vec<&str>>();
                    let lower_bound: u64 = detailed_range[0].parse().unwrap();
                    let upper_bound: u64 = detailed_range[1].parse().unwrap();
                    let numbers:Vec<u64> = (lower_bound..=upper_bound).collect();
                    for number in numbers {
                        ranges.push(number.to_string());
                    }
                }
            }
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        
        assert_eq!(is_invalid("100"), false);
        assert_eq!(is_invalid("1234"), false);
        assert_eq!(is_invalid("123123"), true);
        assert_eq!(is_invalid("123"), false);
        assert_eq!(is_invalid("11"), true);
        assert_eq!(is_invalid("22"), true);
        assert_eq!(is_invalid("99"), true);
        assert_eq!(is_invalid("1010"), true);
        assert_eq!(is_invalid("1188511885"), true);
        assert_eq!(is_invalid("222222"), true);
        assert_eq!(is_invalid("446446"), true);
        assert_eq!(is_invalid("38593859"), true);
    }

    #[test]
    fn test_simple_part_1() {
        let ranges = load_ranges("./src/resources/day02_simple.txt");
        let invalid_ranges: Vec<String> = ranges.into_iter().filter(|r| is_invalid(r)).collect();
        assert_eq!(invalid_ranges.len(), 8);
        let sum = invalid_ranges.iter().map(|r| r.parse::<i32>().unwrap()).sum::<i32>();
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn test_part_1() {
        let ranges = load_ranges("./src/resources/day02_input.txt");
        let invalid_ranges: Vec<String> = ranges.into_iter().filter(|r| is_invalid(r)).collect();
        let sum = invalid_ranges.iter().map(|r| r.parse::<u64>().unwrap()).sum::<u64>();
        println!("Part 1 sum of invalid ranges: {}", sum);
    }

}