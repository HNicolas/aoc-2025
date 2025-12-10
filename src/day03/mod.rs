fn solve(input: &str, digit_number: usize) -> u64 {
    input.lines().fold(0, |acc, line| {
        let mut digits = vec![0u64; digit_number];
        for (joltage_index, joltage) in line
            .chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .enumerate()
        {
            let start_index = if line.len() - joltage_index < digit_number {
                digit_number + joltage_index - line.len()
            } else {
                0
            };
            for digit_index in start_index..digit_number {
                if digits[digit_index] < joltage {
                    digits[digit_index] = joltage;
                    for i in digit_index + 1..digit_number {
                        digits[i] = 0;
                    }
                    break;
                }
            }
        }
        acc + digits.iter().enumerate().fold(0, |acc2, (index, value)| {
            acc2 + value * 10u64.pow(u32::try_from(digit_number - index - 1).unwrap())
        })
    })
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day03/input.txt").expect("Error reading input file");
    let result = solve(&input, 2);
    println!("Day 3 solution 1 is {result}");
    let result = solve(&input, 12);
    println!("Day 3 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day03/sample.txt").expect("Error reading input file");
        let result = solve(&input, 2);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day03/sample.txt").expect("Error reading input file");
        let result = solve(&input, 12);
        assert_eq!(result, 3121910778619);
    }
}
