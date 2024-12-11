use std::collections::HashMap;
use advent_of_code::get_nums_usize;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let nums = get_nums_usize(input);
    let days = 80;
    let mut memo = HashMap::new();
    let count = nums.iter().map(|&n| count_fish(n,days,&mut memo)).sum();
    Some(count)
}

fn count_fish(fish_timer: usize, rem_days: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(v) = memo.get(&(fish_timer, rem_days)) {
        return *v;
    }
    let count = if rem_days == 0 {
          1
    } else if fish_timer == 0  {
        count_fish(6, rem_days - 1, memo) + count_fish(8, rem_days -1, memo)
    } else {
        count_fish(fish_timer - 1, rem_days -1, memo)
    };

    memo.insert((fish_timer, rem_days), count);
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    let nums = get_nums_usize(input);
    let days = 256;
    let mut memo = HashMap::new();
    let count = nums.iter().map(|&n| count_fish(n,days,&mut memo)).sum();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5934));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26984457539));
    }
}
