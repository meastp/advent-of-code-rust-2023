use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Node {
    values: Vec<i64>,
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut total = 0i64;

    for line in input.lines() {
        let mut stack: Vec<Node> = Vec::new();

        stack.push(Node {
            values: line
                .split_ascii_whitespace()
                .map(|str| str.parse::<i64>().unwrap())
                .collect_vec(),
        });

        //println!("START\n{:?}", stack);
        while !stack.last().unwrap().values.iter().all(|v| *v == 0i64) {
            stack.push(Node {
                values: stack
                    .last()
                    .unwrap()
                    .values
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec(),
            });
            //println!("{:?}", stack.last());
        }

        let top = stack
            .iter()
            .rev()
            .fold(0i64, |sum, node| sum + node.values.last().unwrap());

        //println!("END: {top}");

        total += top;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut total = 0i64;

    for line in input.lines() {
        let mut stack: Vec<Node> = Vec::new();

        stack.push(Node {
            values: line
                .split_ascii_whitespace()
                .map(|str| str.parse::<i64>().unwrap())
                .collect_vec(),
        });

        //println!("START\n{:?}", stack);
        while !stack.last().unwrap().values.iter().all(|v| *v == 0i64) {
            stack.push(Node {
                values: stack
                    .last()
                    .unwrap()
                    .values
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec(),
            });
            //println!("{:?}", stack.last());
        }

        let top = stack
            .iter()
            .rev()
            .fold(0i64, |sum, node| node.values.iter().next().unwrap() - sum);

        //println!("END: {top}");

        total += top;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
