advent_of_code::solution!(4);

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
    None
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
        assert_eq!(result, None);
    }
}
