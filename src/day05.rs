#[derive(Debug)]
struct Range {
    start: u64,
    end: u64
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
}

fn load_data(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut loading_ranges = true;
    let mut ranges: Vec<Range> = Vec::new();
    let mut values: Vec<u64> = Vec::new();

    input.lines().for_each(|line| {
        if line.trim().is_empty() {
            loading_ranges = false;
        } else if loading_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push(Range { start, end });
        } else {
            let value: u64 = line.parse().unwrap();
            values.push(value);
        }
    });

    (ranges, values)
}

fn count_fresh_ingredients(ranges: &Vec<Range>, values: &Vec<u64>) -> u32 {
    let mut count: u32 = 0;

    for value in values {
        for range in ranges {
            if range.contains(*value) {
                count += 1;
                break;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part_1() {
        let input: &str = include_str!("../src/resources/day05_simple.txt");
        let (ranges, values) = load_data(input);
        let fresh_count = count_fresh_ingredients(&ranges, &values);
        assert_eq!(fresh_count, 3);
    }

    #[test]
    fn test_part_1() {
        let input: &str = include_str!("../src/resources/day05_input.txt");
        let (ranges, values) = load_data(input);
        let fresh_count = count_fresh_ingredients(&ranges, &values);
        assert_eq!(fresh_count, 601);
    }    
}