struct Machine {
    indicator_lights: u16,
    button_wirings: Vec<u16>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let (indicator_lights, rest) = line.split_once("] ").unwrap();
        let indicator_lights =
            indicator_lights
                .chars()
                .skip(1)
                .enumerate()
                .fold(0u16, |acc, (index, value)| match value {
                    '#' => acc + 2u16.pow(u32::try_from(index).unwrap()),
                    _ => acc,
                });

        let (rest, _) = rest.split_once(") {").unwrap();
        let button_wirings = rest
            .split(") ")
            .map(|values| {
                values[1..]
                    .split(',')
                    .map(|number| number.parse::<u16>().unwrap())
                    .fold(0u16, |acc, index| acc + 2u16.pow(index.into()))
            })
            .collect::<Vec<_>>();

        Self {
            indicator_lights,
            button_wirings,
        }
    }

    /// Give index of pressed buttons
    fn solve_indicator_lights(self: &Self) -> Vec<usize> {
        let mut states = vec![(0u16, Vec::<usize>::new())];
        let mut seen = std::collections::HashSet::from([0u16]);

        loop {
            let mut next_states = Vec::new();
            for (light, pressed_buttons) in states.iter() {
                for (button_index, button) in self.button_wirings.iter().enumerate() {
                    let light = light ^ button;

                    if !seen.insert(light) {
                        continue;
                    }

                    let mut pressed_buttons = pressed_buttons.clone();
                    pressed_buttons.push(button_index);

                    if light == self.indicator_lights {
                        return pressed_buttons;
                    }

                    next_states.push((light, pressed_buttons));
                }
            }
            states = next_states;
        }
    }
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| Machine::from_line(line))
        .fold(0, |acc, machine| {
            acc + machine.solve_indicator_lights().len()
        })
}

// there is an equivalent light pattern for the joltage counters (parity odd -> # even -> .)
// find all solutions to solve this pattern using part 1
// subtract pushed buttons from counters
// divide by 2 if not all 0 repeat
// https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
fn solve2(input: &str) -> usize {
    input.lines().fold(0usize, |acc, line| {
        let (buttons, joltage_levels) = line.split_once("] ").unwrap().1.split_once(") {").unwrap();

        let buttons = buttons
            .split(") ")
            .map(|values| {
                values[1..]
                    .split(',')
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let button_wirings = buttons
            .iter()
            .map(|button| {
                button.iter().fold(0u16, |acc, index| {
                    acc + 2u16.pow(u32::try_from(*index).unwrap())
                })
            })
            .collect::<Vec<_>>();

        let mut joltage_levels = joltage_levels[..joltage_levels.len() - 1]
            .split(',')
            .map(|value| value.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        let mut press_count = 0;
        let mut press_factor = 1;

        while joltage_levels
            .iter()
            .any(|joltage_level| *joltage_level > 0)
        {
            println!("{joltage_levels:?}");
            let light_equivalent =
                joltage_levels
                    .iter()
                    .enumerate()
                    .fold(0u16, |acc, (index, value)| {
                        if value % 2 == 0 {
                            acc
                        } else {
                            acc + 2u16.pow(u32::try_from(index).unwrap())
                        }
                    });
            if light_equivalent == 0 {
                joltage_levels = joltage_levels.iter().map(|l| l / 2).collect();
                press_factor *= 2;
                continue;
            }

            let machine = Machine {
                button_wirings: button_wirings.clone(),
                indicator_lights: light_equivalent,
            };

            // TODO: get all solutions to find the best one
            let pressed_buttons = machine.solve_indicator_lights();
            println!("{pressed_buttons:?} {press_factor}");
            press_count += pressed_buttons.len() * press_factor;

            for &button_index in pressed_buttons.iter() {
                for &counter_index in buttons[button_index].iter() {
                    joltage_levels[counter_index] -= 1;
                }
            }
        }

        println!("pressed {press_count} buttons");

        acc + press_count
    })
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day10/input.txt").expect("Error reading input file");
    let timer = std::time::Instant::now();
    let result = solve(&input);
    println!(
        "Day 10 solution 1 is {result} in {}us",
        timer.elapsed().as_micros()
    );
    let timer = std::time::Instant::now();
    let result = solve2(&input);
    println!(
        "Day 10 solution 2 is {result} in {}us",
        timer.elapsed().as_micros()
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
