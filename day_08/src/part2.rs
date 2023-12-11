use indicatif::ProgressBar;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", process(input));
}

fn process(input: &str) -> usize {
    // Make a Cycle that loops through the chars of the first line of input
    let mut steps = input.lines().next().unwrap().chars().cycle();
    let weak_nodes = input
        .lines()
        .skip(2)
        .map(parse_line)
        .collect::<Vec<WeakNode>>();
    let mut current: Vec<&WeakNode> = weak_nodes
        .iter()
        .filter(|n| n.name.chars().nth(2).unwrap() == 'A')
        .collect::<Vec<_>>();
    dbg!(&current.len());
    dbg!(&weak_nodes.len());
    let spinner = ProgressBar::new_spinner();
    let mut step = 0;
    while not_done(&current) {
        spinner.tick();
        let turn = steps.next();
        current = current
            .iter()
            .map(|node| match turn {
                Some('L') => weak_nodes.iter().find(|n| n.name == node.left).unwrap(),
                Some('R') => weak_nodes.iter().find(|n| n.name == node.right).unwrap(),
                _ => panic!("Invalid turn"),
            })
            .collect::<Vec<&WeakNode>>();
        let final_chars = current
            .iter()
            .map(|n| n.name.chars().nth(2).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("");
        step += 1;
        spinner.set_message(format!("Step {:6} => {}", step, final_chars));
    }
    spinner.finish();
    step
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
