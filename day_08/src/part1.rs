use indicatif::ProgressBar;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", process(input));
}

fn process(input: &str) -> usize {
    // Make a Cycle that loops through the chars of the first line of input
    let mut steps = input.lines().next().unwrap().chars().cycle();
    let weak_nodes = input.lines().skip(2).map(parse_line).collect::<Vec<_>>();
    let mut current: WeakNode = *weak_nodes.iter().find(|n| n.name == "AAA").unwrap();
    let spinner = ProgressBar::new_spinner();
    let mut step = 0;
    while current.name != "ZZZ" {
        spinner.tick();
        let turn = steps.next();
        current = match turn {
            Some('L') => *weak_nodes.iter().find(|n| n.name == current.left).unwrap(),
            Some('R') => *weak_nodes.iter().find(|n| n.name == current.right).unwrap(),
            _ => panic!("Invalid turn"),
        };
        step += 1;
        spinner.set_message(format!("Step {:6} => {}", step, &current.name));
    }
    spinner.finish();
    step
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
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(input), 6)
    }
}
