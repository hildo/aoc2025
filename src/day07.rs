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

fn count_timeline_splits(lines: Vec<&str>) -> u32 {
    // Find the staring point in the first line
    let start_line = lines[0];
    let start_pos = start_line.find('S').unwrap();
    let mut beam_positions_multiverse: Vec<HashSet<usize>> = Vec::new();
    let mut beam_positions: HashSet<usize> = HashSet::new();
    beam_positions.insert(start_pos);
    beam_positions_multiverse.push(beam_positions);

    let mut splits: u32 = 0;
    for row_idx in 1..lines.len() {
        let line = lines[row_idx];
        for verse_idx in 0..beam_positions_multiverse.len() {
            let positions_copy = beam_positions_multiverse[verse_idx].clone();
            let mut new_positions = beam_positions_multiverse[verse_idx].clone();
            for position in positions_copy.iter(){
                if line.chars().nth(*position) != Some('^') {
                    continue;
                }
                new_positions.remove(position);
                let mut left_positions = new_positions.clone();
                left_positions.insert(position - 1);
                beam_positions_multiverse.push(left_positions);
                let mut right_positions = new_positions.clone();
                right_positions.insert(position + 1);
                beam_positions_multiverse.push(right_positions);
                beam_positions_multiverse.remove(verse_idx);
                splits += 2;
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

    #[test]
    fn test_simple_part_2() {
        let input: &str = include_str!("../src/resources/day07_simple.txt");
        let lines = input.lines().collect::<Vec<&str>>();
        let splits = count_timeline_splits(lines);
        assert_eq!(splits, 40);
    }
    
}