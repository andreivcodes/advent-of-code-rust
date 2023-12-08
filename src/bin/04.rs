use std::slice::Windows;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    elf_numbers: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input.lines();

    let mut total = 0;
    for card in cards {
        let card_name = card.split(':').next().unwrap();
        let lists = card.split(':').nth(1).unwrap();
        println!("{card_name:?} - {lists:?}");

        let winning_list = lists.split('|').next().unwrap();
        let numbers_list = lists.split('|').nth(1).unwrap();

        println!("   winning: {winning_list} \n   numbers_list:{numbers_list}");

        let winning_numbers: Vec<i32> = winning_list
            .trim()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let elf_numbers: Vec<i32> = numbers_list
            .trim()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        println!("   winning_numbers: {winning_numbers:?} \n   elf_numbers: {elf_numbers:?}");

        let mut card_points = 0;

        for winning_number in winning_numbers {
            if elf_numbers.contains(&winning_number) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }

        println!("card points: {card_points}");

        total += card_points;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines();

    let mut initial_cards: Vec<Card> = vec![];

    for card in cards {
        let card_name = card.split(':').next().unwrap();

        let card_number_string = card.split(':').next().unwrap().replace("Card", "").clone();

        println!("{card_number_string}");

        let lists = card.split(':').nth(1).unwrap();
        println!("{card_name:?} - {lists:?}");

        let winning_list = lists.split('|').next().unwrap();
        let numbers_list = lists.split('|').nth(1).unwrap();

        println!("   winning: {winning_list} \n   numbers_list:{numbers_list}");

        let winning_numbers: Vec<u32> = winning_list
            .trim()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let elf_numbers: Vec<u32> = numbers_list
            .trim()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        initial_cards.push(Card {
            card_number: card_number_string.trim().parse::<u32>().unwrap(),
            winning_numbers: winning_numbers.clone(),
            elf_numbers: elf_numbers.clone(),
        });

        println!("   winning_numbers: {winning_numbers:?} \n   elf_numbers: {elf_numbers:?}");
    }

    println!("initial cards: {initial_cards:?}");
    let result = process_cards(initial_cards.clone(), initial_cards.clone());

    Some(result)
}

fn process_cards(cards: Vec<Card>, original_cards: Vec<Card>) -> u32 {
    println!();
    let mut total: u32 = 0;
    let cards_clone = cards.clone();

    total += cards.len() as u32;
    for card in cards {
        let mut wins = 0;
        for winning_number in card.winning_numbers {
            if card.elf_numbers.contains(&winning_number) {
                wins += 1;
            }
        }

        let new_cards: Vec<Card> = original_cards
            .clone()
            .into_iter()
            .skip_while(|c| c.card_number != card.card_number)
            .skip(1)
            .take(wins)
            .collect();

        println!(
            "card: {} - wins: {wins} - total: {} - new cards: {:?}",
            card.card_number, total, new_cards
        );
        println!("{:?}", cards_clone);
        total += process_cards(new_cards.clone(), original_cards.clone());
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }
}
