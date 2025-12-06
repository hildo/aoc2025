#[derive(Debug, PartialEq)]
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

fn count_from_ranges(ranges: Vec<Range>) -> u64 {
    let mut active_ranges: Vec<Range> = Vec::new();

    for range in ranges {
        if active_ranges.is_empty() {
            active_ranges.push(range);
            continue;
        }
        
        let mut overlapping_indices: Vec<usize> = Vec::new();
        for range_idx in 0..active_ranges.len() {
            let active_range = &active_ranges[range_idx];
            if range.start <= active_range.end && range.end >= active_range.start {
                // Ranges overlap, add to list
                overlapping_indices.push(range_idx);
            }
        }

        if overlapping_indices.is_empty() {
            // No overlaps, just add the range
            active_ranges.push(range);
        } else {
            // Merge overlapping ranges
            let mut new_start = range.start;
            let mut new_end = range.end;
            for idx in &overlapping_indices {
                let overlapping_range = &active_ranges[*idx];
                if overlapping_range.start < new_start {
                    new_start = overlapping_range.start;
                }
                if overlapping_range.end > new_end {
                    new_end = overlapping_range.end;
                }
            }
            // Remove overlapping ranges from active_ranges (in reverse order to preserve indices)
            for idx in overlapping_indices.iter().rev() {
                active_ranges.remove(*idx);
            }
            // Add the merged range
            active_ranges.push(Range { start: new_start, end: new_end });
        }
    }

    let mut count: u64 = 0;
    for range in &active_ranges {
        count += range.end - range.start + 1;
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

    #[test]
    fn test_simple_part_2() {
        let input: &str = include_str!("../src/resources/day05_simple.txt");
        let (ranges, values) = load_data(input);
        let fresh_count = count_from_ranges(ranges);
        assert_eq!(fresh_count, 14);
    }

    #[test]
    fn test_part_2() {
        let input: &str = include_str!("../src/resources/day05_input.txt");
        let (ranges, values) = load_data(input);
        let fresh_count = count_from_ranges(ranges);
        assert_eq!(fresh_count, 367899984917516);
    }

}