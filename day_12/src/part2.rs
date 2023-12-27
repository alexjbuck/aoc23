#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

fn parse(input: &str) -> impl Iterator<Item = (Vec<Spring>, Vec<usize>)> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs: Vec<Spring> = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("at the disco"),
            })
            .collect();
        let counts: Vec<usize> = counts.split(',').filter_map(|s| s.parse().ok()).collect();

        (springs, counts)
    })
}

pub fn part_1(input: &str) -> u64 {
    parse(input)
        .map(|(springs, counts)| count_possible_arangements(springs, counts))
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    parse(input)
        .map(|(mut springs, mut counts)| {
            springs = springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect();
            counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect();

            count_possible_arangements(springs, counts)
        })
        .sum()
}

fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<usize>) -> u64 {
    // to make the Damaged recursion case simpler
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        };
    }
    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        // Not enough space for remaining numbers
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        // Assume operational
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        // Assume damaged
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}