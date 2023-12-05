advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().clone();

    let mut result = 0;
    for line in lines.into_iter() {
        let mut iter = line.chars().filter(|c| c.is_ascii_digit());
        let first = iter.next().unwrap().to_digit(10).unwrap();
        let last = match iter.last() {
            Some(l) => l.to_digit(10).unwrap(),
            None => first,
        };
        println!("{}{}", first, last);
        result = result + first * 10 + last;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().clone();

    let mut result = 0;
    for line in lines.into_iter() {
        let mut it = (0..line.len()).filter_map(|index| {
            let reduced_line = &line[index..];
            let result = if reduced_line.starts_with("one") {
                '1'
            } else if reduced_line.starts_with("two") {
                '2'
            } else if reduced_line.starts_with("three") {
                '3'
            } else if reduced_line.starts_with("four") {
                '4'
            } else if reduced_line.starts_with("five") {
                '5'
            } else if reduced_line.starts_with("six") {
                '6'
            } else if reduced_line.starts_with("seven") {
                '7'
            } else if reduced_line.starts_with("eight") {
                '8'
            } else if reduced_line.starts_with("nine") {
                '9'
            } else {
                reduced_line.chars().next().unwrap()
            };

            result.to_digit(10)
        });

        let first = it.next().unwrap();
        let last = match it.last() {
            Some(l) => l,
            None => first,
        };
        result = result + first * 10 + last;
        println!("{}{}", first, last);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
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
