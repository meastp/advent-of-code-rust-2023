advent_of_code::solution!(4);

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Card
{
    winning: Vec<u32>,
    numbers: Vec<u32>,
    count: u32
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card = Card{ winning: Vec::new(), numbers: Vec::new(), count: 0  };
        
        let mut groups = s.split('|');

        // winning
        if let Some(ss) = groups.next()
        {
            for n in ss.trim().split(' ').filter(|&s| !s.is_empty() )
            {
                card.winning.push(n.parse::<u32>().unwrap());
            }
        }
        
        // numbers
        if let Some(ss) = groups.next()
        {
            for n in ss.trim().split(' ').filter(|&s| !s.is_empty() )
            {
                let num = n.parse::<u32>().unwrap();
                card.numbers.push(num);
            }
        }

        Ok(card)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines()
    {
        let mut l = line.split(':');

        if let Some(_heading) = l.next()
        {
            //let _id: u32 = heading.strip_prefix("Card ").unwrap().parse().unwrap();
        }
        
        if let Some(card_input) = l.next()
        {
            let card = Card::from_str(card_input).unwrap();
        
            let base: u32 = 2;
            if card.count > 0
            {
                total += base.pow(card.count - 1);
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines()
    {
        let mut l = line.split(':');

        if let Some(_heading) = l.next()
        {
            //let _id: u32 = heading.strip_prefix("Card ").unwrap().parse().unwrap();
        }
        
        if let Some(card_input) = l.next()
        {
            let card = Card::from_str(card_input).unwrap();
            
            cards.push(card);
        }
    }

    let num_cards = cards.len();

    for i in 0..num_cards
    {
        let mut count = 0;
        for num in & cards[i].numbers
        {
            if cards[i].winning.contains(&num)
            {
                count += 1;
            }
        }


            cards[i].count += 1;
        if count > 0
        {

            let card_count = cards[i].count;

            if i + 1 < num_cards
            {
                let subset = &mut cards[i+1..=std::cmp::min(num_cards, i+count)];

                for card in subset
                {
                    (*card).count += card_count;
                }
            }
        }
    }

    //println!("{:?}", cards);

    let total : u32 = cards.into_iter().map(|card| card.count).sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
