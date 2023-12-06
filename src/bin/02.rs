advent_of_code::solution!(2);

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCubeSetError;

impl FromStr for CubeSet {
    type Err = ParseCubeSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = CubeSet {
            red: None,
            green: None,
            blue: None,
        };

        for ss in s.split(',') {
            let mut iter = ss.trim().split(' ');

            let n: u32 = iter.next().unwrap().parse().unwrap();

            match iter.next() {
                Some("red") => set.red = Some(n),
                Some("green") => set.green = Some(n),
                Some("blue") => set.blue = Some(n),
                name @ _ => {
                    println!("unknown: {}", name.unwrap());
                    return Err(ParseCubeSetError);
                }
            }
        }

        Ok(set)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut l = line.split(':');

        let heading = l.next().unwrap();
        let id: u32 = heading.strip_prefix("Game ").unwrap().parse().unwrap();

        let mut is_ok = true;
        for cubeset in l.next().unwrap().split(';') {
            let cs = CubeSet::from_str(cubeset).unwrap();
            if cs.red > Some(12) || cs.green > Some(13) || cs.blue > Some(14) {
                is_ok = false;
            }
        }

        if is_ok {
            total += id;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut set = CubeSet {
            red: None,
            green: None,
            blue: None,
        };

        let mut l = line.split(':');

        let _heading = l.next().unwrap();
        //let id: u32 = heading.strip_prefix("Game ").unwrap().parse().unwrap();

        for cubeset in l.next().unwrap().split(';') {
            let cs = CubeSet::from_str(cubeset).unwrap();

            match set.red {
                Some(n @ _) if cs.red.is_some() && n < cs.red.unwrap() => set.red = cs.red,
                Some(_) => (),
                None => set.red = cs.red,
            }

            match set.green {
                Some(n @ _) if cs.green.is_some() && n < cs.green.unwrap() => set.green = cs.green,
                Some(_) => (),
                None => set.green = cs.green,
            }

            match set.blue {
                Some(n @ _) if cs.blue.is_some() && n < cs.blue.unwrap() => set.blue = cs.blue,
                Some(_) => (),
                None => set.blue = cs.blue,
            }
        }

        total += set.red.unwrap_or(1) * set.green.unwrap_or(1) * set.blue.unwrap_or(1);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
