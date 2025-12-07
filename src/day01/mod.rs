fn solve1(input: &str) -> i32 {
    let mut dial = 50;
    input.lines().fold(0i32, |acc, line| {
        let (direction, value) = line.split_at(1);
        let value = value
            .parse::<i32>()
            .expect(&format!("Invalid value {value}"));
        match direction {
            "R" => dial = (dial + value).rem_euclid(100),
            "L" => dial = (dial - value).rem_euclid(100),
            _ => panic!("Invlalid direction {direction}"),
        }
        if dial == 0 { acc + 1 } else { acc }
    })
}

fn solve2(input: &str) -> i32 {
    let mut dial = 50;
    input.lines().fold(0i32, |mut acc, line| {
        let (direction, value) = line.split_at(1);
        let value = value
            .parse::<i32>()
            .expect(&format!("Invalid value {value}"));
        match (direction, value) {
            (_, 0) => {}
            ("R", value) => dial += value,
            ("L", value) => dial -= value,
            _ => panic!("Invlalid direction {direction}"),
        }
        acc += dial.div_euclid(100).abs();
        dial = dial.rem_euclid(100);
        acc
    })
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day01/input.txt").expect("Error reading inpu file");
    let result = solve1(&input);
    println!("Day 1 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 1 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day01/sample.txt").expect("Error reading inpu file");
        let result = solve1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day01/sample.txt").expect("Error reading inpu file");
        let result = solve2(&input);
        assert_eq!(result, 6);
    }
}
