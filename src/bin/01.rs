pub fn get_elves(input: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<u32>().unwrap())
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .collect();

    elves.sort_unstable_by(|a, b| b.cmp(a));

    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = get_elves(input);

    Some(elves[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = get_elves(input);
    let top3 = elves.iter().take(3).copied().reduce(|a, b| a + b).unwrap();

    Some(top3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
