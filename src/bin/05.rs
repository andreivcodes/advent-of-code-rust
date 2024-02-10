advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let groups = input.split("\n\n");

    let seeds: Vec<u32> = lines
        .into_iter()
        .find(|c| c.starts_with("seeds: "))
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    println!("seeds: {seeds:?}");

    // let ss = HashMap::new();
    // let sf = HashMap::new();
    // let fw = HashMap::new();
    // let wl = HashMap::new();
    // let lt = HashMap::new();
    // let th = HashMap::new();
    // let hl = HashMap::new();
    for group in groups {
        if group.contains("seeds:") || group.is_empty() {
            continue;
        }
        let group_lines = group.lines();

        let group_name = group_lines.clone().next().unwrap();

        let group_items: Vec<Vec<u32>> = group
            .split('\n')
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(|l| l.split(' ').map(|c| c.parse::<u32>().unwrap()).collect())
            .collect();

        println!("{group_name} : {group_items:?}");
    }

    Some(0)
}

pub fn part_two(_input: &str) -> Option<u32> {
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
