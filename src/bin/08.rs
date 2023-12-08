use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Node<'a>
{
    left: &'a str,
    right: &'a str
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut instr = Vec::new();
    if let Some(lr) = lines.next()
    {
        instr = lr.chars().collect_vec();
    }

    lines.next(); // blank line

    let mut map = HashMap::new();

    while let Some(line) = lines.next()
    {
        let mut iter = line.split('=').into_iter();

        let start = iter.next().unwrap().trim();

        let mut pair_iter = iter.next().unwrap().trim().strip_prefix('(').unwrap().strip_suffix(')').unwrap().split(',').into_iter();

        map.insert(start, Node{left: pair_iter.next().unwrap().trim(), right: pair_iter.next().unwrap().trim()});
    }

    //println!("{:?}", map);

    let mut nsteps: u32 = 0;
    let mut current_node = "AAA";

    let mut instruction_iter = instr.iter().cycle();

    while current_node != "ZZZ"
    {
        match instruction_iter.next().unwrap()
        {
            'L' => current_node = map.get(current_node).unwrap().left,
            'R' => current_node = map.get(current_node).unwrap().right,
            i @ _ => panic!("unkown instruction: {}", i)
        }

        nsteps += 1;
    }

    Some(nsteps)

}

fn gcd(a: u64, b: u64) -> u64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
  }

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut instr = Vec::new();
    if let Some(lr) = lines.next()
    {
        instr = lr.chars().collect_vec();
    }

    lines.next(); // blank line

    let mut map = HashMap::new();

    while let Some(line) = lines.next()
    {
        let mut iter = line.split('=').into_iter();

        let start = iter.next().unwrap().trim();

        let mut pair_iter = iter.next().unwrap().trim().strip_prefix('(').unwrap().strip_suffix(')').unwrap().split(',').into_iter();

        map.insert(start, Node{left: pair_iter.next().unwrap().trim(), right: pair_iter.next().unwrap().trim()});
    }

    //println!("{:?}", map);

    let mut all_nsteps: Vec<u32> = Vec::new();

    let mut current_nodes = map.keys().filter(|k| k.ends_with('A')).map(|v| v.to_string()).collect_vec();

    let mut instruction_iter = instr.iter().cycle();

    //println!("{:?}", current_nodes);

    for current_node in &mut current_nodes
    {
        let mut nsteps = 0;
        while !current_node.ends_with('Z')
        {
            let instr = instruction_iter.next().unwrap();

                match instr
                {
                    'L' => *current_node = map.get(&current_node[..]).unwrap().left.to_string(),
                    'R' => *current_node = map.get(&current_node[..]).unwrap().right.to_string(),
                    i @ _ => panic!("unkown instruction: {}", i)
                }

            //println!("{:?}", current_node);

            nsteps += 1;
        }
        all_nsteps.push(nsteps);
    }

    let ans = all_nsteps.iter().map(|v| u64::from(*v)).fold(1, |ans, x| (x*ans) / gcd(x,ans));

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2,));
        assert_eq!(result, Some(6));
    }
}
