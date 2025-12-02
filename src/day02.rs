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

fn is_invalid_pt2(input: &str) -> bool {

    let len = input.len();
    
    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Check if the length is divisible by the pattern length
        if len % pattern_len == 0 {
            let repetitions = len / pattern_len;
            
            // Need at least 2 repetitions to be invalid
            if repetitions >= 2 {
                let pattern = &input[0..pattern_len];
                
                // Check if the entire ID is made of this pattern repeated
                let is_repeated = (0..repetitions)
                    .all(|i| {
                        let start = i * pattern_len;
                        let end = start + pattern_len;
                        &input[start..end] == pattern
                    });
                
                if is_repeated {
                    return true;
                }
            }
        }
    }
    
    false
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
    fn test_is_valid_2() {
        
        assert_eq!(is_invalid_pt2("100"), false);
        assert_eq!(is_invalid_pt2("1234"), false);
        assert_eq!(is_invalid_pt2("123123"), true);
        assert_eq!(is_invalid_pt2("123"), false);
        assert_eq!(is_invalid_pt2("11"), true);
        assert_eq!(is_invalid_pt2("22"), true);
        assert_eq!(is_invalid_pt2("99"), true);
        assert_eq!(is_invalid_pt2("1010"), true);
        assert_eq!(is_invalid_pt2("1188511885"), true);
        assert_eq!(is_invalid_pt2("222222"), true);
        assert_eq!(is_invalid_pt2("446446"), true);
        assert_eq!(is_invalid_pt2("38593859"), true);
        assert_eq!(is_invalid_pt2("999"), true);
        assert_eq!(is_invalid_pt2("1010"), true);
        assert_eq!(is_invalid_pt2("565656"), true);
        assert_eq!(is_invalid_pt2("824824824"), true);
        assert_eq!(is_invalid_pt2("2121212121"), true);
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
    fn test_simple_part_2() {
        let ranges = load_ranges("./src/resources/day02_simple.txt");
        let invalid_ranges: Vec<String> = ranges.into_iter().filter(|r| is_invalid_pt2(r)).collect();
        assert_eq!(invalid_ranges.len(), 13);
        let sum = invalid_ranges.iter().map(|r| r.parse::<u64>().unwrap()).sum::<u64>();
        assert_eq!(sum, 4174379265);
    }

    #[test]
    fn test_part_1() {
        let ranges = load_ranges("./src/resources/day02_input.txt");
        let invalid_ranges: Vec<String> = ranges.into_iter().filter(|r| is_invalid(r)).collect();
        let sum = invalid_ranges.iter().map(|r| r.parse::<u64>().unwrap()).sum::<u64>();
        println!("Part 1 sum of invalid ranges: {}", sum);
    }

    #[test]
    fn test_part_2() {
        let ranges = load_ranges("./src/resources/day02_input.txt");
        let invalid_ranges: Vec<String> = ranges.into_iter().filter(|r| is_invalid_pt2(r)).collect();
        let sum = invalid_ranges.iter().map(|r| r.parse::<u64>().unwrap()).sum::<u64>();
        println!("Part 2 sum of invalid ranges: {}", sum);
    }

}