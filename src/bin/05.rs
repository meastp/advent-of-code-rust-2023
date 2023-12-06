advent_of_code::solution!(5);

use regex::Regex;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Seed {
    seed_id: u64,
    soil_id: Option<u64>,
    fertilizer_id: Option<u64>,
    water_id: Option<u64>,
    light_id: Option<u64>,
    temperature_id: Option<u64>,
    humidity_id: Option<u64>,
    location_id: Option<u64>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.lines();

    let iter_ref = iter.by_ref();

    let mut seeds: Vec<Seed> = Vec::new();

    if let Some(seeds_str) = iter_ref.next() {
        let re = Regex::new(r" [0-9]+").unwrap();

        seeds = re.find_iter(seeds_str).map(|m| Seed {
            seed_id: m.as_str().trim().parse().unwrap(),
            soil_id: None,
            fertilizer_id: None,
            water_id: None,
            light_id: None,
            temperature_id: None,
            humidity_id: None,
            location_id: None,
        }).collect();
    }

    while let Some(_blank) =  iter_ref.next() // blank line
    {
        if let Some(map_type) = iter_ref.next() {
            let mut idx_items: Vec<(u64, u64, u64)> = Vec::new();
            for line in iter_ref.take_while_ref(|line| !line.is_empty())
            {
                let mut items = line.split(' ');
                let items_ref = items.by_ref();
                idx_items.push((items_ref.next().unwrap().parse().unwrap(), items_ref.next().unwrap().parse().unwrap(), items_ref.next().unwrap().parse().unwrap()));
            }
    
            for (dest_start, source_start, len) in idx_items
            {
                for seed in &mut seeds
                {
                    if map_type == "seed-to-soil map:"
                    {
                        if (source_start..source_start+len).contains(&seed.seed_id)
                        {
                            seed.soil_id = Some((seed.seed_id-source_start) + dest_start);
                        }
    
                    }
                    else if map_type == "soil-to-fertilizer map:"
                    {
                        if (source_start..source_start+len).contains(&seed.soil_id.unwrap())
                        {
                            seed.fertilizer_id = Some((seed.soil_id.unwrap()-source_start) + dest_start);
                        }
                    }
                    else if map_type == "fertilizer-to-water map:"
                    {
                        if (source_start..source_start+len).contains(&seed.fertilizer_id.unwrap())
                        {
                            seed.water_id = Some((seed.fertilizer_id.unwrap()-source_start) + dest_start);
                        }
                    }
                    else if map_type == "water-to-light map:"
                    {
                        if (source_start..source_start+len).contains(&seed.water_id.unwrap())
                        {
                            seed.light_id = Some((seed.water_id.unwrap()-source_start) + dest_start);
                        }
                    }
                    else if map_type == "light-to-temperature map:"
                    {
                        if (source_start..source_start+len).contains(&seed.light_id.unwrap())
                        {
                            seed.temperature_id = Some((seed.light_id.unwrap()-source_start) + dest_start);
                        }
                    }
                    else if map_type == "temperature-to-humidity map:"
                    {
                        if (source_start..source_start+len).contains(&seed.temperature_id.unwrap())
                        {
                            seed.humidity_id = Some((seed.temperature_id.unwrap()-source_start) + dest_start);
                        }
                    }
                    else if map_type == "humidity-to-location map:"
                    {
                        if (source_start..source_start+len).contains(&seed.humidity_id.unwrap())
                        {
                            seed.location_id = Some((seed.humidity_id.unwrap()-source_start) + dest_start);
                        }
                    }
                }
            }
            
            for seed in &mut seeds
            {
                if map_type == "seed-to-soil map:"
                {
                    if seed.soil_id.is_none()
                    {
                        seed.soil_id = Some(seed.seed_id);
                    }
                }
                else if map_type == "soil-to-fertilizer map:"
                {
                    if seed.fertilizer_id.is_none()
                    {
                        seed.fertilizer_id = Some(seed.soil_id.unwrap());
                    }
                }
                else if map_type == "fertilizer-to-water map:"
                {
                    if seed.water_id.is_none()
                    {
                        seed.water_id = Some(seed.fertilizer_id.unwrap());
                    }
                }
                else if map_type == "water-to-light map:"
                {
                    if seed.light_id.is_none()
                    {
                        seed.light_id = Some(seed.water_id.unwrap());
                    }
                }
                else if map_type == "light-to-temperature map:"
                {
                    if seed.temperature_id.is_none()
                    {
                        seed.temperature_id = Some(seed.light_id.unwrap());
                    }
                }
                else if map_type == "temperature-to-humidity map:"
                {
                    if seed.humidity_id.is_none()
                    {
                        seed.humidity_id = Some(seed.temperature_id.unwrap());
                    }
                }
                else if map_type == "humidity-to-location map:"
                {
                    if seed.location_id.is_none()
                    {
                        seed.location_id = Some(seed.humidity_id.unwrap());
                    }
                }
            }
    
        }
    }

    //println!("{:?}", seeds);

    u32::try_from(seeds.into_iter().map(|seed| seed.location_id.unwrap()).min().unwrap()).ok()
}


#[derive(Debug, PartialEq, Clone)]
struct SeedRange
{
    rng : Vec<std::ops::Range<u64>>
}

impl SeedRange
{
    fn intersection(&mut self, target_range : std::ops::Range<u64>) -> Vec<std::ops::Range<u64>>
    {

         let mut matched : Vec<std::ops::Range<u64>> = Vec::new();
         let mut unmatched : Vec<std::ops::Range<u64>> = Vec::new();

       // println!("intersection input:\n{:?}", target_range);
        // println!("before intersection:\n{:?}", self.rng);

        for rng in &self.rng
        {
            //for target_range in input_range
            //{
                let matched_rng = std::ops::Range{start: std::cmp::max(rng.start, target_range.start), end: std::cmp::min(rng.end, target_range.end)};
                if matched_rng.start < matched_rng.end
                {
                  //  println!("matched:\n{:?}", matched_rng);
                matched.push(matched_rng);
                }

                let unmatched_before = std::ops::Range{start: rng.start, end: std::cmp::min(rng.end, target_range.start)};
                if unmatched_before.start < unmatched_before.end
                {
                  //  println!("unmatched_before:\n{:?}", unmatched_before);
                unmatched.push(unmatched_before);

                }

                let unmatched_after = std::ops::Range{start: std::cmp::max(rng.start, target_range.end), end: rng.end};
                if unmatched_after.start < unmatched_after.end
                {

                   // println!("unmatched_after:\n{:?}", unmatched_after);
                unmatched.push(unmatched_after);
                }
            //}
        }


        self.rng = unmatched;

        //println!("after intersection:\n{:?}", self.rng);

        matched
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut iter = input.lines();

    let iter_ref = iter.by_ref();

    let mut seeds: Vec<SeedRange> = Vec::new();

    if let Some(seeds_str) = iter_ref.next() {
        let re = Regex::new(r" ([0-9]+) ([0-9]+)").unwrap();

        for m in re.find_iter(seeds_str)
            {
                let mut ranges : Vec<std::ops::Range<u64>> = Vec::new();
                ranges.push(std::ops::Range{ start: m.as_str().split(' ').into_iter().nth(1).unwrap().parse().unwrap(), end : m.as_str().split(' ').into_iter().nth(1).unwrap().parse::<u64>().unwrap() + m.as_str().split(' ').into_iter().nth(2).unwrap().parse::<u64>().unwrap()});
                seeds.push(SeedRange { rng: ranges });
            }
    }

    //println!("first:\n{:?}", seeds);

    while let Some(_blank) =  iter_ref.next() // blank line
    {
        let mut next_seeds: Vec<SeedRange> = Vec::new();

        if let Some(_map_type) = iter_ref.next() {

            //println!("{:?}", map_type);

            let mut idx_items: Vec<(u64, u64, u64)> = Vec::new();
            for line in iter_ref.take_while_ref(|line| !line.is_empty())
            {
                let mut items = line.split(' ');
                let items_ref = items.by_ref();
                idx_items.push((items_ref.next().unwrap().parse().unwrap(), items_ref.next().unwrap().parse().unwrap(), items_ref.next().unwrap().parse().unwrap()));
            }
    
            for (dest_start, source_start, len) in idx_items
            {
                for seed in &mut seeds
                {
                    //if map_type == "seed-to-soil map:"
                    //{
                        let mut result = seed.intersection(source_start..source_start+len);
                        
                        for rng in &mut result
                        {
                            if rng.start < source_start
                            {
                                panic!("source_start larger than rng.start");
                            }

                            let result_len = rng.end - rng.start;

                            if source_start > dest_start
                            {

                                rng.start -= source_start-dest_start;
                                rng.end = rng.start+std::cmp::min(len, result_len);
                            }
                            else 
                            {
                                
                                rng.start += dest_start-source_start;
                                rng.end = rng.start+std::cmp::min(len, result_len);
                            }

                            //println!("source_start: {} dest_start: {} len: {} result_len: {} rng: {:?}", source_start, dest_start, len, result_len, rng);

                        }

                        next_seeds.push(SeedRange{rng: result});
                   // }
                }
            }
            
            for seed in &seeds
            {
                next_seeds.push(seed.clone());
            }
    
        }
        next_seeds.retain(|seed| !seed.rng.is_empty());

        //println!("{:?}", next_seeds);

        //println!("seeds:\n{:?}", seeds);
        seeds = next_seeds;
    }


    u32::try_from(seeds.into_iter().map(|seed| seed.rng.into_iter().map(|rng| rng.start).min().unwrap()).min().unwrap()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
