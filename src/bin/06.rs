advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    let mut iter = input.lines();

    if let Some(time) = iter.next()
    {
        times = time.strip_prefix("Time:      ").unwrap().split_ascii_whitespace().filter(|item| !item.is_empty()).map(|item| item.parse::<u32>().unwrap()).collect()
    }

    if let Some(distance) = iter.next()
    {
        distances = distance.strip_prefix("Distance:  ").unwrap().split_ascii_whitespace().filter(|item| !item.is_empty()).map(|item| item.parse::<u32>().unwrap()).collect()
    }

    //println!("{:?}\n{:?}", times, distances);

    let it = times.iter().zip(distances.iter());

    let mut all_above_record : Vec<u32> = Vec::new();

    for (time, record_distance) in it
    {
        let mut above_record: u32 = 0;
        for button_time in 1..*time
        {
            if (*time-button_time)*button_time > *record_distance
            {
                above_record += 1;
            }
        }
        if above_record > 0
        {
            all_above_record.push(above_record);
        }
    }

    //println!("{:?}", all_above_record);

    let mut result: u32 = 1;

    for n in all_above_record
    {
        result *= n;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
        let mut time_str: String = String::new();
        let mut distance_str: String = String::new();
    
        let mut iter = input.lines();
    
        if let Some(time) = iter.next()
        {
            time_str = time.strip_prefix("Time:      ").unwrap().chars().filter(|item| !item.is_whitespace()).collect();
        }
    
        if let Some(distance) = iter.next()
        {
            distance_str = distance.strip_prefix("Distance:  ").unwrap().chars().filter(|item| !item.is_whitespace()).collect();
        }
    
    
        let mut all_above_record : Vec<u64> = Vec::new();
    
        let time: u64 = time_str.parse().ok().unwrap();
        let record_distance: u64 = distance_str.parse().ok().unwrap();

        //println!("{:?}\n{:?}", time, record_distance);
        {
            let mut above_record: u64 = 0;
            for button_time in 1..time
            {
                if (time-button_time)*button_time > record_distance
                {
                    above_record += 1;
                }
            }
            if above_record > 0
            {
                all_above_record.push(above_record);
            }
        }
    
        //println!("{:?}", all_above_record);
    
        let mut result: u32 = 1;
    
        for n in all_above_record
        {
            result *= u32::try_from(n).unwrap();
        }
    
        Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]    
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
