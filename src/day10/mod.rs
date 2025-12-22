use std::collections::HashMap;

struct Machine {
    lights: u16,
    buttons: Vec<Vec<usize>>,
    counters: Vec<u16>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let (lights, rest) = line.split_once("] ").unwrap();
        let lights = lights[1..]
            .chars()
            .enumerate()
            .fold(0u16, |acc, (index, c)| {
                acc + if c == '#' { 1 << index } else { 0 }
            });

        let (buttons, counters) = rest.split_once(" {").unwrap();
        let buttons = buttons
            .split_ascii_whitespace()
            .map(|button| {
                button
                    .trim_matches(['(', ')'])
                    .split(',')
                    .map(|value| value.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let counters = counters[..counters.len() - 1]
            .split(',')
            .map(|value| value.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        Machine {
            lights,
            buttons,
            counters,
        }
    }
}

fn get_button_value(button: &[usize]) -> u16 {
    button.iter().fold(0u16, |acc, value| acc + (1 << *value))
}

fn get_valid_combinations<'a>(
    lights: &u16,
    buttons: &'a Vec<Vec<usize>>,
) -> Vec<Vec<&'a Vec<usize>>> {
    let button_values = buttons
        .iter()
        .map(|button| get_button_value(button))
        .collect::<Vec<_>>();

    let mut valid_combinations = vec![];

    // all valid solutions are a combination of some buttons pressed at most once
    for n in 0..2u16.pow(u32::try_from(buttons.len()).unwrap()) {
        let mut pressed_buttons = vec![];
        let mut result = 0;
        for i in 0..buttons.len() {
            if (n >> i) & 1 == 1 {
                result ^= button_values[i];
                pressed_buttons.push(&buttons[i]);
            }
        }
        if result == *lights {
            valid_combinations.push(pressed_buttons);
        }
    }

    valid_combinations
}

fn counters_to_lights(counters: &Vec<u16>) -> u16 {
    counters
        .iter()
        .enumerate()
        .fold(0u16, |acc, (index, value)| {
            acc + if value % 2 == 0 { 0 } else { 1 << index }
        })
}

fn solve_counters_cached(
    counters: &Vec<u16>,
    buttons: &Vec<Vec<usize>>,
    cache: &mut HashMap<Vec<u16>, Option<usize>>,
) -> Option<usize> {
    if let Some(&result) = cache.get(counters) {
        return result;
    }

    if counters.iter().all(|counter| *counter == 0) {
        cache.insert(counters.clone(), Some(0));
        return Some(0);
    }

    let lights = counters_to_lights(&counters);
    let combinations = get_valid_combinations(&lights, buttons);
    let mut solutions = vec![];
    'combination: for combination in combinations {
        let mut next_counters = counters.clone();
        for button in combination.iter() {
            for &counter_index in button.iter() {
                if next_counters[counter_index] > 0 {
                    next_counters[counter_index] -= 1;
                } else {
                    // invalid combination, go to the next one
                    continue 'combination;
                }
            }
        }

        if next_counters.iter().all(|value| *value % 2 == 0) {
            let halved: Vec<u16> = next_counters.iter().map(|value| *value / 2).collect();
            if let Some(solution) = solve_counters_cached(&halved, buttons, cache) {
                solutions.push(combination.len() + 2 * solution);
            }
        }
    }

    let result = solutions.iter().min().copied();
    cache.insert(counters.clone(), result);
    result
}

fn solve_counters(counters: &Vec<u16>, buttons: &Vec<Vec<usize>>) -> Option<usize> {
    let mut cache = HashMap::new();
    solve_counters_cached(counters, buttons, &mut cache)
}

/// This solution can be improved by starting with solutions with few button press
/// and stopping iteration after first find
fn solve(input: &str) -> usize {
    let machines = input.lines().map(|line| Machine::parse(line));
    let mut result = 0;
    for machine in machines {
        let combinations = get_valid_combinations(&machine.lights, &machine.buttons);
        result += combinations
            .iter()
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len();
    }
    result
}

fn solve2(input: &str) -> usize {
    input.lines().fold(0usize, |acc, line| {
        let machine = Machine::parse(line);
        let results = solve_counters(&machine.counters, &machine.buttons);
        acc + results.unwrap()
    })
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day10/input.txt").expect("Error reading input file");
    let timer = std::time::Instant::now();
    let result = solve(&input);
    println!(
        "Day 10 solution 1 is {result} {}ms",
        timer.elapsed().as_millis()
    );
    let timer = std::time::Instant::now();
    let result = solve2(&input);
    println!(
        "Day 10 solution 2 is {result} {}ms",
        timer.elapsed().as_millis()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day10/sample.txt").expect("Error reading input file");
        let result = solve(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day10/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 33);
    }
}
