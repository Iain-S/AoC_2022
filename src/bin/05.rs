pub fn part_one(input: &str) -> Option<String> {
    let stacks: Vec<Vec<char>> = Vec::new();

    let starting_state_text = input.split("\n\n").collect::<Vec<&str>>()[0];
    // println!("{:?}", starting_state_text);
    starting_state_text
        .rsplit("\n")
        .map(|line| {
            println!("{:?}", line);
            1
        })
        .sum::<i32>();

    // input.split("\n").map(|line|{
    // line.chunks(4)
    // })

    // Vec of stacks
    // Vec of Vecs
    Some(String::from("A"))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
