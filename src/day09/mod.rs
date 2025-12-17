fn solve(input: &str) -> isize {
    let coordinates = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
        .collect::<Vec<_>>();
    let mut max_area = 0;
    for i in 0..coordinates.len() - 1 {
        let (xi, yi) = coordinates[i];
        for j in i + 1..coordinates.len() {
            let (xj, yj) = coordinates[j];
            let area = ((xj - xi).abs() + 1) * ((yj - yi).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn solve2(input: &str) -> isize {
    let vertices = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
        .collect::<Vec<_>>();

    // using a btreemap of btreeset was slower (memory access ?)
    let mut frontier = std::collections::HashSet::new();
    for i in 0..vertices.len() {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % vertices.len()];
        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                frontier.insert((x, y));
            }
        }
    }

    let mut max_area = 0;
    for i in 0..vertices.len() - 1 {
        let (xi, yi) = vertices[i];
        for j in i + 1..vertices.len() {
            let (xj, yj) = vertices[j];
            let area = ((xj - xi).abs() + 1) * ((yj - yi).abs() + 1);

            if area > max_area
                && !frontier.iter().any(|&(x, y)| {
                    x > xi.min(xj) && x < xi.max(xj) && y > yi.min(yj) && y < yi.max(yj)
                })
            {
                max_area = area;
            }
        }
    }
    max_area
}

// not used
fn _is_in_polygon((x, y): (isize, isize), vertices: &Vec<(isize, isize)>) -> bool {
    let mut result = false;
    for i in 0..vertices.len() {
        let (x1, y1) = (vertices[i].0, vertices[i].1);
        let (x2, y2) = (
            vertices[(i + 1) % vertices.len()].0,
            vertices[(i + 1) % vertices.len()].1,
        );

        if y1 == y2 {
            // horizontal edge
            if y != y1 {
                // parallel no crossing
                continue;
            } else {
                // on same line than edge
                if x > x1.max(x2) {
                    // after edge
                    continue;
                } else if x >= x1.min(x2) && x <= x1.max(x2) {
                    // on edge
                    return true;
                } else {
                    let y_prev = vertices[usize::try_from(
                        (isize::try_from(i).unwrap() - 1)
                            .rem_euclid(isize::try_from(vertices.len()).unwrap()),
                    )
                    .unwrap()]
                    .1;
                    let y_next = vertices[(i + 1).rem_euclid(vertices.len())].1;
                    if y > y_prev.min(y_next) && y < y_prev.max(y_next) {
                        // cross only if previous and next edge are not on the same side
                        result = !result;
                    }
                }
            }
        } else {
            // vertical edge x1 == x2
            if x > x1 || y < y1.min(y2) || y > y1.max(y2) {
                // after, lower or upper the edge
                continue;
            } else if x == x1 {
                // on edge
                return true;
            } else {
                // cross (consider crossing on vertex)
                result = !result;
            }
        }
    }
    result
}

pub fn run() {
    let input = std::fs::read_to_string("./src/day09/input.txt").expect("Error reading input file");
    let result = solve(&input);
    println!("Day 9 solution 1 is {result}");
    let result = solve2(&input);
    println!("Day 9 solution 2 is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            std::fs::read_to_string("./src/day09/sample.txt").expect("Error reading input file");
        let result = solve(&input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part_two() {
        let input =
            std::fs::read_to_string("./src/day09/sample.txt").expect("Error reading input file");
        let result = solve2(&input);
        assert_eq!(result, 24);
    }
}
