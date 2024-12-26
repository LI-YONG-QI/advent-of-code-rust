advent_of_code::solution!(1);

fn build_right_and_left(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    let v = input.lines();

    //Create right and left vectors
    for line in v {
        let mut item = line.split_whitespace();
        left.push(item.next().unwrap().parse::<u64>().unwrap());
        right.push(item.next().unwrap().parse::<u64>().unwrap())
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    let (mut left, mut right) = build_right_and_left(input);

    left.sort();
    right.sort();

    let left_iter = left.iter();
    let mut right_iter = right.iter();

    for l in left_iter {
        let diff = l.abs_diff(right_iter.next().unwrap().clone());
        sum += diff as u64;
    }

    println!("Sum: {}", sum);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    let (left, right) = build_right_and_left(input);

    for l in left.iter() {
        let mul: u64 = right
            .iter()
            .filter(|&n| *n == *l)
            .count()
            .try_into()
            .unwrap();

        sum += l.checked_mul(mul).unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
