advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        let all_digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        let combined = format!(
            "{}{}",
            all_digits.first().unwrap(),
            all_digits.last().unwrap()
        );

        let num: u32 = combined.parse().unwrap();

        total += num;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for mut line in input.lines() {
        let mut first: &str = "A";
        let mut has_first = false;
        let mut last: &str = "A";

        while !line.is_empty() {
            if line.starts_with("one") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "1";
                } else {
                    last = "1";
                }
            } else if line.starts_with("two") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "2";
                } else {
                    last = "2";
                }
            } else if line.starts_with("three") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "3";
                } else {
                    last = "3";
                }
            } else if line.starts_with("four") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "4";
                } else {
                    last = "4";
                }
            } else if line.starts_with("five") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "5";
                } else {
                    last = "5";
                }
            } else if line.starts_with("six") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "6";
                } else {
                    last = "6";
                }
            } else if line.starts_with("seven") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "7";
                } else {
                    last = "7";
                }
            } else if line.starts_with("eight") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "8";
                } else {
                    last = "8";
                }
            } else if line.starts_with("nine") {
                line = &line[1..];
                if !has_first {
                    has_first = true;
                    first = "9";
                } else {
                    last = "9";
                }
            } else if line.chars().next().unwrap_or('A').is_numeric() {
                if !has_first {
                    has_first = true;
                    first = &line[0..1];
                } else {
                    last = &line[0..1];
                }
                line = &line[1..];
            } else if !line.is_empty() {
                line = &line[1..];
            }
        }

        if last == "A" {
            last = first;
        }

        let combined = format!("{}{}", first, last);

        let num: u32 = combined.parse().unwrap();

        total += num;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
