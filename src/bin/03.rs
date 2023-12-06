use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
struct Number {
    num: u32,
    idx: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let arr: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    //println!("{:?}", arr);

    for (row_idx, row) in input.lines().enumerate() {
        let mut iter = row.chars().enumerate().peekable();

        let iter_ref = iter.by_ref();

        while iter_ref.peek().is_some() {
            let num_str = iter_ref.take_while_ref(|(_idx, c)| c.is_numeric());
            let (idxs, nums): (Vec<_>, String) = num_str.unzip();
            if !nums.is_empty() {
                let mut has_adjacent_sign = false;
                for idx in idxs {
                    if row_idx > 0 {
                        if has_adjacent_sign == false && idx > 0 {
                            has_adjacent_sign = arr[row_idx - 1][idx - 1] != '.'
                                && !arr[row_idx - 1][idx - 1].is_numeric();
                        }
                        if has_adjacent_sign == false {
                            has_adjacent_sign =
                                arr[row_idx - 1][idx] != '.' && !arr[row_idx - 1][idx].is_numeric();
                        }
                        if has_adjacent_sign == false && idx + 1 < arr[row_idx - 1].len() {
                            has_adjacent_sign = arr[row_idx - 1][idx + 1] != '.'
                                && !arr[row_idx - 1][idx + 1].is_numeric();
                        }
                    }

                    if has_adjacent_sign == false && idx > 0 {
                        has_adjacent_sign =
                            arr[row_idx][idx - 1] != '.' && !arr[row_idx][idx - 1].is_numeric();
                    }
                    if has_adjacent_sign == false && idx + 1 < arr[row_idx].len() {
                        has_adjacent_sign =
                            arr[row_idx][idx + 1] != '.' && !arr[row_idx][idx + 1].is_numeric();
                    }

                    if row_idx + 1 < arr.len() {
                        if has_adjacent_sign == false && idx > 0 {
                            has_adjacent_sign = arr[row_idx + 1][idx - 1] != '.'
                                && !arr[row_idx + 1][idx - 1].is_numeric();
                        }
                        if has_adjacent_sign == false {
                            has_adjacent_sign =
                                arr[row_idx + 1][idx] != '.' && !arr[row_idx + 1][idx].is_numeric();
                        }
                        if has_adjacent_sign == false && idx + 1 < arr[row_idx + 1].len() {
                            has_adjacent_sign = arr[row_idx + 1][idx + 1] != '.'
                                && !arr[row_idx + 1][idx + 1].is_numeric();
                        }
                    }
                }

                if has_adjacent_sign {
                    total += nums.parse::<u32>().unwrap();
                }
            }

            while let Some((_nxt_idx, nxt_c)) = iter_ref.peek() {
                if !nxt_c.is_numeric() {
                    iter_ref.next();
                } else {
                    break;
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let arr: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    //println!("{:?}", arr);

    let mut numbers: Vec<(usize, usize, u32)> = Vec::new();

    for (row_idx, row) in input.lines().enumerate() {
        let mut iter = row.chars().enumerate().peekable();

        let iter_ref = iter.by_ref();

        while iter_ref.peek().is_some() {
            let num_str = iter_ref.take_while_ref(|(_idx, c)| c.is_numeric());
            let (idxs, nums): (Vec<_>, String) = num_str.unzip();
            if !nums.is_empty() {
                let num = nums.parse::<u32>().unwrap();
                for idx in idxs {
                    if row_idx > 0 {
                        if idx > 0
                            && arr[row_idx - 1][idx - 1] != '.'
                            && !arr[row_idx - 1][idx - 1].is_numeric()
                        {
                            numbers.push((row_idx - 1, idx - 1, num));
                        }
                        if arr[row_idx - 1][idx] != '.' && !arr[row_idx - 1][idx].is_numeric() {
                            numbers.push((row_idx - 1, idx, num));
                        }
                        if idx + 1 < arr[row_idx - 1].len()
                            && arr[row_idx - 1][idx + 1] != '.'
                            && !arr[row_idx - 1][idx + 1].is_numeric()
                        {
                            numbers.push((row_idx - 1, idx + 1, num));
                        }
                    }

                    if idx > 0
                        && arr[row_idx][idx - 1] != '.'
                        && !arr[row_idx][idx - 1].is_numeric()
                    {
                        numbers.push((row_idx, idx - 1, num));
                    }
                    if idx + 1 < arr[row_idx].len()
                        && arr[row_idx][idx + 1] != '.'
                        && !arr[row_idx][idx + 1].is_numeric()
                    {
                        numbers.push((row_idx, idx + 1, num));
                    }

                    if row_idx + 1 < arr.len() {
                        if idx > 0
                            && arr[row_idx + 1][idx - 1] != '.'
                            && !arr[row_idx + 1][idx - 1].is_numeric()
                        {
                            numbers.push((row_idx + 1, idx - 1, num));
                        }
                        if arr[row_idx + 1][idx] != '.' && !arr[row_idx + 1][idx].is_numeric() {
                            numbers.push((row_idx + 1, idx, num));
                        }
                        if idx + 1 < arr[row_idx + 1].len()
                            && arr[row_idx + 1][idx + 1] != '.'
                            && !arr[row_idx + 1][idx + 1].is_numeric()
                        {
                            numbers.push((row_idx + 1, idx + 1, num));
                        }
                    }
                }
            }

            while let Some((_nxt_idx, nxt_c)) = iter_ref.peek() {
                if !nxt_c.is_numeric() {
                    iter_ref.next();
                } else {
                    break;
                }
            }
        }
    }

    numbers.sort();

    let groups = numbers
        .into_iter()
        .unique()
        .group_by(|(row_idx, idx, _num)| (row_idx.clone(), idx.clone()));

    for (_key, items) in groups.into_iter() {
        let local_numbers: Vec<u32> = items.map(|(_row_idx, _idx, num)| num).collect();

        let mut tot: u32 = 1;
        if local_numbers.len() == 2 {
            for n in local_numbers.into_iter() {
                tot *= n;
            }
            total += tot;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
