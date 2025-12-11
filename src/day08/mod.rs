fn parse(input: &str) -> Vec<(isize, isize, isize)> {
    input
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|value| value.parse::<isize>().unwrap());
            (
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn compute_distances(
    coordinates: &Vec<(isize, isize, isize)>,
) -> std::collections::BTreeMap<isize, Vec<(usize, usize)>> {
    let mut distances = std::collections::BTreeMap::new();
    for i in 0..coordinates.len() - 1 {
        for j in i + 1..coordinates.len() {
            let distance = (coordinates[i].0 - coordinates[j].0).pow(2)
                + (coordinates[i].1 - coordinates[j].1).pow(2)
                + (coordinates[i].2 - coordinates[j].2).pow(2);
            distances
                .entry(distance)
                .and_modify(|jonction: &mut Vec<(usize, usize)>| jonction.push((i, j)))
                .or_insert(vec![(i, j)]);
        }
    }
    distances
}

fn solve(input: &str, jonction_count: usize) -> usize {
    let coordinates = parse(input);
    let distances = compute_distances(&coordinates);

    let sorted_pairs = distances.iter().flat_map(|(_, pairs)| pairs);
    let mut circuit_ids = vec![0; coordinates.len()];
    let mut circuit_id = 0;
    for pair in sorted_pairs.take(jonction_count) {
        let first_box_circuit = circuit_ids[pair.0];
        let second_box_circuit = circuit_ids[pair.1];
        if first_box_circuit > 0 {
            if second_box_circuit > 0 {
                if first_box_circuit == second_box_circuit {
                    continue;
                }
                // merge circuits
                circuit_ids
                    .iter_mut()
                    .filter(|circuit_id| **circuit_id == second_box_circuit)
                    .for_each(|circuit_id| *circuit_id = first_box_circuit);
            } else {
                // add second box to circuit
                circuit_ids[pair.1] = first_box_circuit;
            }
        } else if second_box_circuit > 0 {
            // add first box to circuit
            circuit_ids[pair.0] = second_box_circuit;
        } else {
            // update both with and increment circuit index
            circuit_id += 1;
            circuit_ids[pair.0] = circuit_id;
            circuit_ids[pair.1] = circuit_id;
        }
    }

    let sizes = circuit_ids
        .iter()
        .filter(|id| **id > 0)
        .fold(
            std::collections::HashMap::with_capacity(circuit_id),
            |mut acc, &id| {
                acc.entry(id).and_modify(|value| *value += 1).or_insert(1);
                acc
            },
        )
        .values()
        .map(|&value| value)
        .collect::<Vec<_>>();

    let mut max_sizes = vec![0usize; 3];
    for mut size in sizes {
        for max_size in max_sizes.iter_mut() {
            if size > *max_size {
                std::mem::swap(max_size, &mut size);
            }
        }
    }

    max_sizes
        .into_iter()
        .reduce(|acc, value| acc * value)
        .unwrap()
}

fn solve2(input: &str) -> isize {
    let coordinates = parse(input);
    let distances = compute_distances(&coordinates);

    let sorted_pairs = distances.iter().flat_map(|(_, pairs)| pairs);
    let mut circuit_ids = vec![0; coordinates.len()];
    let mut circuit_id = 0;
    let mut connected_boxes = 0;
    let mut circuit_count = 0;
    for pair in sorted_pairs {
        let first_box_circuit = circuit_ids[pair.0];
        let second_box_circuit = circuit_ids[pair.1];
        if first_box_circuit > 0 {
            if second_box_circuit > 0 {
                if first_box_circuit == second_box_circuit {
                    continue;
                }
                // merge circuits
                circuit_ids
                    .iter_mut()
                    .filter(|circuit_id| **circuit_id == second_box_circuit)
                    .for_each(|circuit_id| *circuit_id = first_box_circuit);
                circuit_count -= 1;
            } else {
                // add second box to circuit
                circuit_ids[pair.1] = first_box_circuit;
                connected_boxes += 1;
            }
        } else if second_box_circuit > 0 {
            // add first box to circuit
            circuit_ids[pair.0] = second_box_circuit;
            connected_boxes += 1;
        } else {
            // update both with and increment circuit index
            circuit_id += 1;
            circuit_ids[pair.0] = circuit_id;
            circuit_ids[pair.1] = circuit_id;
            circuit_count += 1;
            connected_boxes += 2;
        }

        // check condition
        if connected_boxes >= coordinates.len() && circuit_count == 1 {
            return coordinates[pair.0].0 * coordinates[pair.1].0;
        }
    }
    panic!()
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day08/input.txt").expect("Error reading input file");
    let result = solve(&input, 1000);
    println!("Day 8 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 8 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day08/sample.txt").expect("Error reading input file");
        let result = solve(&input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day08/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 25272);
    }
}
