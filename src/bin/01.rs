fn get_food_calories(input: &str) -> Vec<u32> {
    input.split('\n')
        .filter_map(|word| word.trim()
            .parse::<u32>()
            .ok())
        .collect()
}

fn get_elf_calories(input: &str) -> Vec<u32> {
    let twice_split: Vec<Vec<u32>> = input.split("\n\n")
        .map(get_food_calories)
        .collect();
    let sums: Vec<u32> = twice_split.iter()
        .map(|x| x.iter().sum())
        .collect();
    sums
}


pub fn part_one(input: &str) -> Option<u32> {
    let sums: Vec<u32> = get_elf_calories(input);
    let max_sum = sums.iter()
        .max();
    max_sum.copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sums: Vec<u32> = get_elf_calories(input);
    sums.sort();
    sums.reverse();
    let top_three_sum: u32 = sums[..3].iter().sum();
    Some(top_three_sum)
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
