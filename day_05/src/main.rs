use indicatif::{ProgressBar, ProgressStyle, MultiProgress, ProgressIterator, ParallelProgressIterator};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use std::sync::{Arc, Mutex};

fn main() {
    println!("{}",part1(include_str!("input.txt")));
    println!("{}",part2(include_str!("input.txt")));
}

fn part1(input: &str) -> usize {
    let seeds: Vec<usize> = input
        .split("\n\n")
        .nth(0).unwrap()
        .split(":")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let mappings = input
        .split("\n\n")
        .skip(1)
        .map(|chunk| parse_mapping(chunk))
        .collect::<Vec<Mapping>>();
    seeds.par_iter().progress_count(seeds.len() as u64)
        .map(|seed| process_seed(seed, &mappings))
        .min().unwrap()
}

fn part2(input: &str) -> usize {
    let seeds: Vec<RangeSeed> = input
        .split("\n\n")
        .nth(0).unwrap()
        .split(":")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks_exact(2)
        .map(|chunk| RangeSeed { from: chunk[0], len: chunk[1] })
        .collect();
    
    let mappings = input
        .split("\n\n")
        .skip(1)
        .map(|chunk| parse_mapping(chunk))
        .collect::<Vec<Mapping>>();

    let multi_progress = Arc::new(MultiProgress::new());
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap()
        .progress_chars("#>-");

    let outer_pb = multi_progress.add(ProgressBar::new(seeds.len() as u64));
    outer_pb.set_style(style.clone());

    seeds.par_iter().progress_with(outer_pb)
        .map(|range_seed| {
            let inner_pb = multi_progress.add(ProgressBar::new(range_seed.len as u64));
            inner_pb.set_style(style.clone());
            let inner_pb = Arc::new(Mutex::new(inner_pb));
    
            process_seed_range_seed(range_seed, &mappings, inner_pb.clone())
        })
        .min().unwrap()
}

fn process_seed(seed:&usize, mappings: &Vec<Mapping>) -> usize {
    let mut value:usize = seed.clone();
    // println!("Seed: {}", value);
    for mapping in mappings {
        // println!("Mapping:{} => {}", value, mapping.pipe(value.clone()));
        // dbg!(mapping);
        value = mapping.pipe(value);
    }
    value
}

fn process_seed_range_seed(range_seed:&RangeSeed, mappings: &Vec<Mapping>, pb: Arc<Mutex<ProgressBar>>) -> usize {
    let mut result = Vec::new();
    let range = range_seed.from..range_seed.from+range_seed.len;

    for seed in range.progress_with(pb.lock().unwrap().clone()) {
        result.push(process_seed(&seed, &mappings))
    }
    result.iter().min().unwrap().clone()
}


fn parse_mapping(input: &str) -> Mapping {
    // println!("{}", input);
    let mappings = input.lines().skip(1)
        .map(|line| line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>()
                        .into()
        )
        .collect::<Vec<Pipe>>()
        .into();
    mappings
}


// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), 35);
    }
}

#[derive(Debug)]
struct Pipe {
    from: usize,
    to: usize,
    len: usize,
}

#[derive(Debug)]
struct Mapping {
    pipes: Vec<Pipe>,
}

impl Pipe {
    fn contains(&self, value: usize) -> bool {
        self.from <= value && value <= self.from + self.len
    }
    fn pipe(&self, value: usize) -> Option<usize> {
        if self.contains(value) {
            Some(value - self.from + self.to)
        } else {
            None
        }
    }

}

impl Mapping {
    fn pipe(&self, value: usize) -> usize {
        self.pipes
            .iter()
            .filter_map(|p| p.pipe(value))
            .next().unwrap_or(value)
    }
}

impl From<Vec<usize>> for Pipe {
    fn from(vec: Vec<usize>) -> Self {
        Pipe {
            to: vec[0],
            
            from: vec[1],
            len: vec[2],
        }
    }
}

impl From<Vec<Pipe>> for Mapping {
    fn from(pipes: Vec<Pipe>) -> Self {
        Mapping { pipes }
    }
}

struct RangeSeed {
    from: usize,
    len: usize,
}