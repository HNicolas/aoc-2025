fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_removable_rolls(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut map = std::collections::HashMap::new();
    for (line_index, line) in grid.iter().enumerate() {
        for (char_index, &char) in line.iter().enumerate() {
            if char == '@' {
                map.entry((line_index, char_index)).or_insert(0);
                for i in usize::try_from(0.max(i32::try_from(line_index).unwrap() - 1)).unwrap()
                    ..grid.len().min(line_index + 2)
                {
                    for j in usize::try_from(0.max(i32::try_from(char_index).unwrap() - 1)).unwrap()
                        ..line.len().min(char_index + 2)
                    {
                        if i == line_index && j == char_index || grid[i][j] != '@' {
                            continue;
                        }

                        map.entry((i, j))
                            .and_modify(|value| *value += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }
    map.iter()
        .filter(|&(_, &adjacent)| adjacent < 4)
        .map(|(&coordinates, _)| coordinates)
        .collect()
}

fn solve1(input: &str) -> usize {
    let grid = parse_input(input);
    let removable_rolls = get_removable_rolls(&grid);
    removable_rolls.len()
}

fn solve2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut removed_rolls = 0;
    loop {
        let removable_rolls = get_removable_rolls(&grid);

        if removable_rolls.len() == 0 {
            break;
        }

        removed_rolls += removable_rolls.len();
        for (i, j) in removable_rolls {
            grid[i][j] = '.';
        }
    }
    removed_rolls
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!()
    }
    println!()
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day04/input.txt").expect("Error reading input file");
    let result = solve1(&input);
    println!("Day 4 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 4 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day04/sample.txt").expect("Error reading input file");
        let result = solve1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day04/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 43);
    }
}
