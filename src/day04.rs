use std::ops::RangeInclusive;

pub fn solve<ComparatorFn: Fn(RangeInclusive<u32>, RangeInclusive<u32>) -> bool>(
    input: &str,
    comparator: ComparatorFn,
) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| {
            let split: Vec<u32> = line
                .split(|c| c == ',' || c == '-')
                .map(|num| num.parse().unwrap())
                .collect();
            if comparator(split[0]..=split[1], split[2]..=split[3]) {
                acc + 1
            } else {
                acc
            }
        })
}

pub fn range_contains_other(range: RangeInclusive<u32>, other_range: RangeInclusive<u32>) -> bool {
    range.contains(other_range.start()) && range.contains(other_range.end())
        || other_range.contains(range.start()) && other_range.contains(range.end())
}

pub fn ranges_overlap(range: RangeInclusive<u32>, other_range: RangeInclusive<u32>) -> bool {
    range.contains(other_range.start())
        || range.contains(other_range.end())
        || other_range.contains(range.start())
        || other_range.contains(range.end())
}
