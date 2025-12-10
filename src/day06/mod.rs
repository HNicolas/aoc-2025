fn solve(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut results = grid[0]
        .iter()
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for line_index in 1..grid.len() - 1 {
        for operation_index in 0..results.len() {
            let value = grid[line_index][operation_index].parse::<usize>().unwrap();
            match grid[grid.len() - 1][operation_index] {
                "+" => results[operation_index] += value,
                "*" => results[operation_index] *= value,
                _ => panic!(),
            }
        }
    }
    results.iter().sum()
}

pub fn solve2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut column = 0;
    while column < grid[0].len() {
        let operator = grid[grid.len() - 1][column];
        let mut operation_result = String::with_capacity(grid.len() - 1);
        for row in 0..grid.len() - 1 {
            if grid[row][column].is_digit(10) {
                operation_result.push(grid[row][column]);
            }
        }
        let mut operation_result = operation_result.parse::<usize>().unwrap();
        column += 1;

        while column < grid[0].len() {
            let mut value = String::with_capacity(grid.len() - 1);
            for row in 0..grid.len() - 1 {
                if grid[row][column].is_digit(10) {
                    value.push(grid[row][column]);
                }
            }
            column += 1;

            if value.len() == 0 {
                break;
            }

            let value = value.parse::<usize>().unwrap();
            match operator {
                '*' => operation_result *= value,
                '+' => operation_result += value,
                _ => panic!(),
            }
        }

        result += operation_result;
    }
    result
}

pub fn run() {
    let input =
        std::fs::read_to_string("./src/day06/input.txt").expect("Error reading input file");
    let result = solve(&input);
    println!("Day 6 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 6 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day06/sample.txt").expect("Error reading input file");
        let result = solve(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day06/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 3263827);
    }
}
