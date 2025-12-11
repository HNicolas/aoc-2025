fn solve2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut columns = lines[0]
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'S')
        .map(|(i, _)| (i, 1usize))
        .collect::<std::collections::HashMap<_, _>>();

    for line_index in 1..lines.len() {
        let mut next_columns = std::collections::HashMap::new();
        for (&column_index, &count) in columns.iter() {
            if lines[line_index][column_index] == '^' {
                if column_index > 0 {
                    next_columns
                        .entry(column_index - 1)
                        .and_modify(|current_count| *current_count += count)
                        .or_insert(count);
                }
                if column_index < lines[line_index].len() - 1 {
                    next_columns
                        .entry(column_index + 1)
                        .and_modify(|current_count| *current_count += count)
                        .or_insert(count);
                }
            } else {
                next_columns
                    .entry(column_index)
                    .and_modify(|current_count| *current_count += count)
                    .or_insert(count);
            }
        }
        columns = next_columns;
    }
    columns.iter().map(|(_, &value)| value).sum()
}

fn solve(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut columns = lines[0]
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'S')
        .map(|(i, _)| i)
        .collect::<std::collections::HashSet<_>>();

    let mut split_count = 0;
    for line_index in 1..lines.len() {
        let mut next_columns = std::collections::HashSet::new();
        for &column_index in columns.iter() {
            if lines[line_index][column_index] == '^' {
                if column_index > 0 {
                    next_columns.insert(column_index - 1);
                }
                if column_index < lines[line_index].len() - 1 {
                    next_columns.insert(column_index + 1);
                }
                split_count += 1;
            } else {
                next_columns.insert(column_index);
            }
        }
        columns = next_columns;
    }
    split_count
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day07/input.txt").expect("Error reading input file");
    let result = solve(&input);
    println!("Day 7 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 7 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day07/sample.txt").expect("Error reading input file");
        let result = solve(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day07/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 40);
    }
}
