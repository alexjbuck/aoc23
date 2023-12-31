pub fn run() {
    println!("Part 1: {}",process(include_str!("input.txt")))
}

fn process(input: &str) -> usize {
    let mut counts: Vec<usize> = vec![1;input.lines().count()];
    input.lines().enumerate().for_each(|l| process_line(l.0, l.1,counts.as_mut()));
    counts.iter().sum()
}

fn process_line<'a>(line_number:usize, line: &str, counts: &'a mut Vec<usize>) {
    let winning_numbers = line.split([':','|']).nth(1).unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let picks = line.split([':','|']).nth(2).unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let winners = picks.iter().filter(|x| winning_numbers.contains(x)).count();
    let current_line_weight = counts[line_number];
    counts[line_number+1..line_number+1+winners].iter_mut().for_each(|x| *x+=current_line_weight);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
    }
}