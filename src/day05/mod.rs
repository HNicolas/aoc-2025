fn solve(input: &str) -> usize {
    let (ranges, values) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(
            std::collections::BTreeMap::new(),
            |mut tree, (lower_bound, upper_bound)| {
                let lower_bound = lower_bound.parse::<usize>().unwrap();
                let upper_bound = upper_bound.parse::<usize>().unwrap();
                tree.entry(lower_bound)
                    .and_modify(|old_value| {
                        if *old_value < upper_bound {
                            *old_value = upper_bound;
                        }
                    })
                    .or_insert(upper_bound);
                tree
            },
        );
    let mut result = 0;
    for value in values.lines() {
        let value = value.parse::<usize>().unwrap();
        if ranges
            .range(..=value)
            .any(|(_, &upper_bound)| upper_bound >= value)
        {
            result += 1;
        }
    }
    result
}

fn solve2(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut merged_ranges = Vec::<(usize, usize)>::new();
    for range in ranges.lines() {
        let (lower_bound, upper_bound) = range.split_once('-').unwrap();
        let mut lower_bound = lower_bound.parse::<usize>().unwrap();
        let mut upper_bound = upper_bound.parse::<usize>().unwrap();

        let (non_overlapping, overlapping): (Vec<_>, Vec<_>) = merged_ranges
            .iter()
            .partition(|(l, u)| *l > upper_bound || *u < lower_bound);
        for (l, u) in overlapping {
            if l < lower_bound {
                lower_bound = l;
            }

            if u > upper_bound {
                upper_bound = u;
            }
        }

        merged_ranges = non_overlapping;
        merged_ranges.push((lower_bound, upper_bound));
    }

    merged_ranges
        .iter()
        .fold(0, |acc, &(lower_bound, upper_bound)| {
            acc + upper_bound - lower_bound + 1
        })
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day05/input.txt").expect("Error reading input file");
    let result = solve(&input);
    println!("Day 5 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 5 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day05/sample.txt").expect("Error reading input file");
        let result = solve(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day05/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 14);
    }
}
