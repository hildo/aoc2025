#[derive(Debug)]
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
        println!("Total: {}", total);
        // assert_eq!(total, 4277556);
    }

}