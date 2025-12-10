fn solve1(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .fold(0, |mut acc, (lower_bound, upper_bound)| {
            let lower_bound = if lower_bound.len() % 2 == 0 {
                let (first, second) = lower_bound.split_at(lower_bound.len() / 2);
                let first = first.parse::<u64>().unwrap();
                let second = second.parse::<u64>().unwrap();
                if first >= second { first } else { first + 1 }
            } else {
                10u64.pow(((lower_bound.len() - 1) / 2).try_into().unwrap())
            };
            let upper_bound = if upper_bound.len() % 2 == 0 {
                let (first, second) = upper_bound.split_at(upper_bound.len() / 2);
                let first = first.parse::<u64>().unwrap();
                let second = second.parse::<u64>().unwrap();
                if first <= second { first } else { first - 1 }
            } else {
                10u64.pow(((upper_bound.len() - 1) / 2).try_into().unwrap()) - 1
            };

            for i in lower_bound..=upper_bound {
                acc += format!("{i}{i}").parse::<u64>().unwrap();
            }
            acc
        })
}

fn solve2(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .fold(0, |mut acc, (lower_bound, upper_bound)| {
            // set with invalid numbers seen for this range
            let mut seen = std::collections::HashSet::new();
            // from 1 char to the max number of chars to have at least one repetition
            for i in 1u32..=u32::try_from(upper_bound.len() / 2).unwrap() {
                // all numbers with exactly i chars
                for j in 10u32.pow(i - 1)..10u32.pow(i) {
                    // we have at least 2 repetions for invalid numbers
                    for k in 2usize.. {
                        let value = j
                            .to_string()
                            .repeat(k)
                            .parse::<u64>()
                            .unwrap();
                        if value > upper_bound.parse::<u64>().unwrap() {
                            break;
                        }

                        if value < lower_bound.parse::<u64>().unwrap() {
                            continue;
                        }

                        if seen.insert(value) {
                            acc += value;
                        }
                    }
                }
            }

            acc
        })
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day02/input.txt").expect("Error reading input file");
    let result = solve1(&input);
    println!("Day 2 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 2 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day02/sample.txt").expect("Error reading input file");
        let result = solve1(&input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day02/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 4174379265);
    }
}
