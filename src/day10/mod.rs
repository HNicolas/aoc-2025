#[derive(Debug)]
struct Machine {
    indicator_light: u16,
    button_wirings: Vec<u16>,
}

impl Machine {
    fn new(line: &str) -> Self {
        let (indicator_light, rest) = line.split_once("] ").unwrap();
        let indicator_light = indicator_light.chars().skip(1).enumerate().fold(
            0u16,
            |acc, (index, value)| match value {
                '#' => acc + 2u16.pow(u32::try_from(index).unwrap()),
                _ => acc,
            },
        );

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
            indicator_light,
            button_wirings,
        }
    }

    fn solve(self: &Self) -> Vec<u16> {
        let mut states = vec![(0u16, Vec::<u16>::new())];
        let mut seen = std::collections::HashSet::from([0u16]);

        loop {
            let mut new_states = Vec::new();
            for (light, pressed_buttons) in states.iter() {
                for button in self.button_wirings.iter() {
                    let light = light ^ button;

                    if !seen.insert(light) {
                        continue;
                    }

                    let mut pressed_buttons = pressed_buttons.clone();
                    pressed_buttons.push(*button);

                    if light == self.indicator_light {
                        return pressed_buttons;
                    }

                    new_states.push((light, pressed_buttons));
                }
            }
            states = new_states;
        }
    }
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| Machine::new(line))
        .fold(0, |acc, machine| acc + machine.solve().len())
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day10/input.txt").expect("Error reading input file");
    let timer = std::time::Instant::now();
    let result = solve(&input);
    println!("Day 10 solution 1 is {result} in {}us", timer.elapsed().as_micros());
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
}
