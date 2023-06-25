/*
*   Rock, Paper, Scissors
*   A, B, C
*   X, Y, Z
*/

fn shape_points(mve: &str) -> u32 {
    match mve {
        "X" => 1,
        "A" => 1,
        "Y" => 2,
        "B" => 2,
        "Z" => 3,
        "C" => 3,
        _ => todo!(),
    }
}

fn round_points(opponent: &str, me: &str) -> u32 {
    match opponent {
        "A" => match me {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => todo!(),
        },
        "B" => match me {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => todo!(),
        },
        "C" => match me {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn losing_move(mve: &str) -> &str {
    match mve {
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _ => todo!(),
    }
}



fn winning_move(mve: &str) -> &str {
    match mve {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => todo!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points: u32 = 0;

    input
        .lines()
        .for_each(|x| {
            let moves: Vec<&str> = x.split(' ').collect();
            points += shape_points(moves[1]) + round_points(moves[0], moves[1]);
        });

    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points: u32 = 0;

    input
        .lines()
        .for_each(|x| {
            let data: Vec<&str> = x.split(' ').collect();

            match data[1] {
                "X" => {
                    // Lose: 0 + shape
                    points += shape_points(losing_move(data[0]))
                }
                "Y" => {
                    // Draw: 3 + same shape
                    points += 3 + shape_points(data[0])
                }
                "Z" => {
                    // Win: 6 + shape
                    points += 6 + shape_points(winning_move(data[0]))
                }
                _ => todo!(),
            }
        });

    Some(points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
