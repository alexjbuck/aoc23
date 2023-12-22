use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar};
use num::integer::lcm;
use rayon::prelude::*;
use std::sync::Arc;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", process(input));
}

fn process(input: &str) -> usize {
    // Make a Cycle that loops through the chars of the first line of input
    let weak_nodes = input
        .lines()
        .skip(2)
        .map(parse_line)
        .collect::<Vec<WeakNode>>();
    let mut current: Vec<&WeakNode> = weak_nodes
        .iter()
        .filter(|n| n.name.chars().nth(2).unwrap() == 'A')
        .collect::<Vec<_>>();
    dbg!(current.len());
    dbg!(weak_nodes.len());

    let multi_progress = Arc::new(MultiProgress::new());
    let outer_pb: ProgressBar = multi_progress.add(ProgressBar::new(current.len() as u64));

    current
        .par_iter()
        .progress_with(outer_pb)
        .map(|node| {
            let mut current = node.clone();
            let mut steps = input.lines().next().unwrap().chars().cycle();
            let spinner = multi_progress.add(ProgressBar::new_spinner());
            let mut step = 0;
            while current.name.chars().nth(2).unwrap() != 'Z' {
                spinner.tick();
                let turn = steps.next();
                current = match turn {
                    Some('L') => weak_nodes.iter().find(|n| n.name == current.left).unwrap(),
                    Some('R') => weak_nodes.iter().find(|n| n.name == current.right).unwrap(),
                    _ => panic!("Invalid turn"),
                };
                step += 1;
                spinner.set_message(format!("Step {:6} => {}", step, &current.name));
            }
            spinner.finish();
            step
        })
        .reduce(|| 1, |acc, step| lcm(acc, step))
}

fn not_done(current: &Vec<&WeakNode>) -> bool {
    current
        .iter()
        .any(|n| n.name.chars().nth(2).unwrap() != 'Z')
}

#[derive(Debug, Copy, Clone)]
struct WeakNode<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_line(line: &str) -> WeakNode {
    let name = &line[..3];
    let left = &line[7..10];
    let right = &line[12..15];
    WeakNode { name, left, right }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process(input), 6)
    }
}
