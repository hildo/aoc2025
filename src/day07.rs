use std::collections::HashSet;

fn count_splits(lines: Vec<&str>) -> u32 {
    // Find the staring point in the first line
    let start_line = lines[0];
    let start_pos = start_line.find('S').unwrap();
    let mut beam_positions: HashSet<usize> = HashSet::new();
    beam_positions.insert(start_pos);

    let mut splits: u32 = 0;
    for row_idx in 1..lines.len() {
        let line = lines[row_idx];
        let positions_copy = beam_positions.clone();
        for col_idx in positions_copy {
            if line.chars().nth(col_idx) == Some('^') {
                beam_positions.remove(&col_idx);
                beam_positions.insert(&col_idx - 1);
                beam_positions.insert(&col_idx + 1);
                splits += 1;
            }
        }
    }
    splits
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part_1() {
        let input: &str = include_str!("../src/resources/day07_simple.txt");
        let lines = input.lines().collect::<Vec<&str>>();
        let splits = count_splits(lines);
        assert_eq!(splits, 21);
    }

    #[test]
    fn test_part_1() {
        let input: &str = include_str!("../src/resources/day07_input.txt");
        let lines = input.lines().collect::<Vec<&str>>();
        let splits = count_splits(lines);
        assert_eq!(splits, 1507);
    }
    
}