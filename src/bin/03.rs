use std::{char, usize};

advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy, PartialEq)]
enum ItemType {
    Dot,
    Symbol,
    Number,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    x: usize,
    y: usize,
    item_type: ItemType,
    val: char,
}
pub fn part_one(input: &str) -> Option<u32> {
    let _total = 0;

    let grid_items: Vec<Vec<Item>> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, character)| Item {
                    x,
                    y,
                    item_type: match character {
                        '.' => ItemType::Dot,
                        c if c.is_ascii_digit() => ItemType::Number,
                        _ => ItemType::Symbol,
                    },
                    val: character,
                })
                .collect()
        })
        .collect();

    let mut numbers: Vec<Vec<Item>> = vec![];

    for line in grid_items.clone() {
        let mut tmp_number: Vec<Item> = vec![];

        println!("{:?}", line.iter().map(|e| e.val).collect::<Vec<char>>());

        for item in line {
            match item.item_type {
                ItemType::Number => {
                    tmp_number.push(item);
                }
                _ => {
                    if !tmp_number.is_empty() {
                        println!("{:?}", tmp_number);
                        numbers.push(tmp_number.clone());
                        tmp_number.clear();
                    }
                }
            }
        }
        if !tmp_number.is_empty() {
            println!("{:?}", tmp_number);
            numbers.push(tmp_number.clone());
            tmp_number.clear();
        }
    }

    let mut total = 0;
    for number in numbers {
        let nr: u32 = number
            .iter()
            .map(|n| n.val.to_digit(10).unwrap())
            .reduce(|acc, e| acc * 10 + e)
            .unwrap();

        println!("{:?}", nr);

        let offsets = vec![
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut valid = false;
        for offset in offsets {
            for digit in number.clone() {
                let outer_x = digit.x as i32 + offset.0;
                let outer_y = digit.y as i32 + offset.1;

                if outer_x >= 0 && outer_y >= 0 {
                    let outer_item = grid_items
                        .iter()
                        .flatten()
                        .find(|item| (item.x == outer_x as usize) && (item.y == outer_y as usize));

                    if outer_item.is_some() && ItemType::Symbol == outer_item.unwrap().item_type {
                        valid = true;
                    }
                }
            }
        }
        if valid {
            total += nr;
        }
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));

        let result_2 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_2, Some(925));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(467835));

        let result_2 = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result_2, Some(6756));
    }
}
