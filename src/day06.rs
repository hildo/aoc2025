use std::u64;

#[derive(Debug,Clone)]
struct Problem {
    values: Vec<u64>,
    operation: String
}

impl Problem {
    fn new() -> Problem {
        Problem {
            values: Vec::new(),
            operation: String::new()
        }
    }

    fn add_value(&mut self, value: u64) {
        self.values.push(value);
    }

    fn set_operation(&mut self, operation: String) {
        self.operation = operation;
    }
    fn execute(&self) -> u64 {
        match self.operation.as_str() {
            "+" => self.values.iter().sum(),
            "*" => self.values.iter().product(),
            _ => 0
        }
    }
}

fn load_data(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    let lines = input.lines();
    let line_count = lines.count();
    for (line_idx, line) in input.lines().enumerate() {
        if line_idx == line_count - 1 {
            // populate Operations
            let operations = line.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();

            operations.iter().enumerate().for_each(|(op_idx, operation)| {
                problems[op_idx].set_operation(operation.to_string());
            })
        } else {
            let split_values = line.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>();
            
            split_values.iter().enumerate().for_each(|(value_idx, value)| {
                if line_idx == 0 {
                    // initialize problems
                    problems.push(Problem::new());
                }
                problems[value_idx].add_value(*value as u64);
            });
        }
    }

    problems
}

fn load_data2(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let line_count = lines.len();

    let operation_line = lines[line_count - 1];
    // find the indexes where there are operations.  These are important columns
    let operation_indices: Vec<usize> = operation_line
        .char_indices()
        .filter(|(_, c)| !c.is_whitespace()) // Filter out whitespace characters
        .map(|(index, _)| index) // Extract only the byte index
        .collect();
    for opertation_counter in 0..operation_indices.len() {
        let operation_idx = operation_indices[opertation_counter];
        let left_edge = operation_idx;
        let right_edge = if opertation_counter + 1 < operation_indices.len() {
            operation_indices[opertation_counter + 1] - 2
        } else {
            operation_line.len() - 1
        };
        
        let mut problem = Problem::new();
        problem.set_operation(operation_line.chars().nth(operation_idx).unwrap().to_string());

        for col_idx in (left_edge..=right_edge).rev() {
            let mut new_value: u64 = 0;
            let mut exp_value = 0;
            for line_idx in (0..line_count - 1).rev() {
                let line = lines[line_idx];
                if line.len() > col_idx {
                    let char_at_pos = line.chars().nth(col_idx).unwrap();
                    if !char_at_pos.is_whitespace() {
                        let digit = char_at_pos.to_digit(10).unwrap() as u64;
                        new_value += digit * 10u64.pow(exp_value);
                        exp_value += 1;
                    }
                }
            }
            problem.add_value(new_value);
        }
        problems.push(problem);
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part_1() {
        let input: &str = include_str!("../src/resources/day06_simple.txt");
        let problems = load_data(input);
        let total: u64 = problems.iter().map(|f | f.execute()).sum();
        assert_eq!(total, 4277556);
    }
        #[test]
    fn test_part_1() {
        let input: &str = include_str!("../src/resources/day06_input.txt");
        let problems = load_data(input);
        let total: u64 = problems.iter().map(|f | f.execute()).sum();
        assert_eq!(total, 6295830249262);
    }

    #[test]
    fn test_simple_part_2() {
        let input: &str = include_str!("../src/resources/day06_simple.txt");
        let mut problems = load_data2(input);
        let total: u64 = problems.iter().map(|f | f.execute()).sum();
        assert_eq!(total, 3263827);
    }    

        #[test]
    fn test_part_2() {
        let input: &str = include_str!("../src/resources/day06_input.txt");
        let problems = load_data2(input);
        let total: u64 = problems.iter().map(|f | f.execute()).sum();
        assert_eq!(total, 9194682052782);
    }

}