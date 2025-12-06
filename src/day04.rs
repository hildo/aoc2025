
fn parse_input(input: &str) -> Vec<&str> {
    input
        .split("\n")
        // .into_iter()
        // .filter(|line| line.len() > 0)
        // .map(|line| parse_string_to_numbers(line))
        .collect()
}

fn count_accessible(input: &str) -> u64 {
    fn count_char_occurrences(s: &str, target_char: char) -> usize {
        s.chars().filter(|&c| c == target_char).count()
    }    

    let lines = parse_input(input);
    let grid_width = lines[0].len();

    let mut count: u64 = 0;
    for row_idx  in 0..lines.len() {
        for (col_idx, c) in lines[row_idx].char_indices() {
            if c == '@' {
                let left_bound = if col_idx >= 1 { col_idx - 1 } else { 0 };
                let right_bound = if col_idx + 1 < grid_width { col_idx + 1 } else { grid_width - 1 };
                let mut surrounding_count= 0;
                // test the 8 positions adjecent to this position
                if row_idx > 0 {
                    let above_row = lines[row_idx - 1];
                    surrounding_count += count_char_occurrences(&above_row[left_bound..=right_bound], '@');
                }
                surrounding_count += count_char_occurrences(&lines[row_idx][left_bound..=right_bound], '@') - 1; // exclude self
                if row_idx + 1 < lines.len() {
                    let below_row = lines[row_idx + 1];
                    surrounding_count += count_char_occurrences(&below_row[left_bound..=right_bound], '@');
                }
                if surrounding_count < 4 {
                    count += 1;
                }
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
        let input: &str = include_str!("../src/resources/day04_simple.txt");
        let accessible_count = count_accessible(input);
        assert_eq!(accessible_count, 13);
    }

    #[test]
    fn test_part_1() {
        let input: &str = include_str!("../src/resources/day04_input.txt");
        let accessible_count = count_accessible(input);
        println!("Part 1: Accessible positions count = {}", accessible_count);
    }    
}