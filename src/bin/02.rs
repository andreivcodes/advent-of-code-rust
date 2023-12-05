advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines();

    let mut result = 0;

    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    for game_line in games {
        let line_parts: Vec<&str> = game_line.split(':').collect();

        let game_id = line_parts[0].replace("Game ", "").parse::<u32>().unwrap();

        let sets: Vec<&str> = line_parts[1].split(';').collect();

        let mut red_cnt = 0;
        let mut green_cnt = 0;
        let mut blue_cnt = 0;

        let mut tmp_red = 0;
        let mut tmp_green = 0;
        let mut tmp_blue = 0;
        for set in sets {
            let cubes: Vec<&str> = set.split(',').collect();

            for cube in cubes {
                if cube.contains("red") {
                    tmp_red = cube.trim().replace(" red", "").parse::<u64>().unwrap();
                }
                if cube.contains("green") {
                    tmp_green = cube.trim().replace(" green", "").parse::<u64>().unwrap();
                }
                if cube.contains("blue") {
                    tmp_blue = cube.trim().replace(" blue", "").parse::<u64>().unwrap();
                }

                red_cnt = red_cnt.max(tmp_red);
                green_cnt = green_cnt.max(tmp_green);
                blue_cnt = blue_cnt.max(tmp_blue);
            }
        }
        if red_cnt <= red_max && blue_cnt <= blue_max && green_cnt <= green_max {
            result += game_id;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines();

    let mut result = 0;

    for game_line in games {
        let line_parts: Vec<&str> = game_line.split(':').collect();

        let sets: Vec<&str> = line_parts[1].split(';').collect();

        let mut red_cnt = 0;
        let mut green_cnt = 0;
        let mut blue_cnt = 0;

        for set in sets {
            let cubes: Vec<&str> = set.split(',').collect();
            let mut tmp_red = 0;
            let mut tmp_green = 0;
            let mut tmp_blue = 0;

            for cube in cubes {
                if cube.contains("red") {
                    tmp_red = cube.trim().replace(" red", "").parse::<u32>().unwrap();
                }
                if cube.contains("green") {
                    tmp_green = cube.trim().replace(" green", "").parse::<u32>().unwrap();
                }
                if cube.contains("blue") {
                    tmp_blue = cube.trim().replace(" blue", "").parse::<u32>().unwrap();
                }
            }
            red_cnt = red_cnt.max(tmp_red);
            green_cnt = green_cnt.max(tmp_green);
            blue_cnt = blue_cnt.max(tmp_blue);
        }
        result += red_cnt * green_cnt * blue_cnt;
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

        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
