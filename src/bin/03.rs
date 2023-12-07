use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum ItemType {
    Dot,
    Symbol,
    Number,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    x: usize,
    y: usize,
    item_type: ItemType,
    val: char,
}
pub fn part_one(input: &str) -> Option<u32> {
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

        for item in line {
            match item.item_type {
                ItemType::Number => {
                    tmp_number.push(item);
                }
                _ => {
                    if !tmp_number.is_empty() {
                        numbers.push(tmp_number.clone());
                        tmp_number.clear();
                    }
                }
            }
        }
        if !tmp_number.is_empty() {
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

pub fn part_two(input: &str) -> Option<u64> {
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

        for item in line {
            match item.item_type {
                ItemType::Number => {
                    tmp_number.push(item);
                }
                _ => {
                    if !tmp_number.is_empty() {
                        numbers.push(tmp_number.clone());
                        tmp_number.clear();
                    }
                }
            }
        }
        if !tmp_number.is_empty() {
            numbers.push(tmp_number.clone());
            tmp_number.clear();
        }
    }

    let mut total: u64 = 0;

    let stars = grid_items.iter().flatten().filter(|item| item.val == '*');

    for star in stars {
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

        let mut pieces: HashSet<Vec<Item>> = HashSet::new();

        println!("star {:?}", star);
        for offset in offsets {
            let outer_x = star.x as i32 + offset.0;
            let outer_y = star.y as i32 + offset.1;

            if outer_x >= 0 && outer_y >= 0 {
                let outer_item = grid_items
                    .iter()
                    .flatten()
                    .find(|item| item.x == outer_x as usize && item.y == outer_y as usize);

                if outer_item.is_some() && ItemType::Number == outer_item.unwrap().item_type {
                    let number = numbers
                        .iter()
                        .find(|nr| nr.contains(outer_item.unwrap()))
                        .unwrap();
                    pieces.insert(number.to_vec());
                }
            }
        }
        println!("pieces {:?}", pieces);
        println!();

        if pieces.len() == 2 {
            let mut product = 1;
            for piece in pieces {
                let nr: u32 = piece
                    .iter()
                    .map(|n| n.val.to_digit(10).unwrap())
                    .reduce(|acc, e| acc * 10 + e)
                    .unwrap();

                product *= nr;
            }

            total += product as u64;
            println!("product {:?}", product);
        }
    }

    println!("total {:?}", total);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
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
            "examples", DAY, 4,
        ));
        assert_eq!(result_2, Some(6756));
    }
}
