pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", process(input));
}

fn process(input: &str) -> isize {
    input.lines().map(handle_line).sum()
}

fn handle_line(line: &str) -> isize {
    let sequence = line.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    find_prev(sequence)
}

fn find_prev(sequence: Vec<isize>) -> isize {
    if sequence.iter().all(|x| x==&0) {
        0
    } else {
        sequence.first().unwrap() - find_prev(diff(&sequence))
    }
}

fn diff(sequence: &Vec<isize>) -> Vec<isize> {
    sequence.windows(2).map(|x| x[1]-x[0]).collect::<Vec<_>>()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input), 2)
    }
}
