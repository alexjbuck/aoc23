use regex::Regex;
use itertools::Itertools;

pub fn run() {
    println!("Part 1: {}",process(include_str!("input.txt")))
}

fn process(input: &str) -> usize {
    let schematic: Schematic = Schematic::new(input);
    // dbg!(&schematic);
    schematic.parts().iter().sum()
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Symbol {
    char: String,
    position: Position,
}
#[derive(Debug)]
struct Number {
    value: usize,
    position: Position,
}
#[derive(Debug)]
struct Position {
    row: usize,
    start: usize,
    end: usize,
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        let mut row = 0;
        // Regex match on all consecutive digits
        let number_pattern = Regex::new(r"\d+").unwrap();
        // Regex match on all non-digit and also non period (.) characters
        let symbol_pattern = Regex::new(r"[^\d\.]").unwrap();

        for line in input.lines() {
            // println!("{}", line);
            numbers.extend(
                number_pattern.find_iter(line)
                .map(|m| Number{
                    value: m.as_str().parse::<usize>().unwrap(),
                    position: Position{row, start: m.start(), end: m.end()-1}
                }).collect::<Vec<Number>>()
            );
            symbols.extend(
                symbol_pattern.find_iter(line)
                .map(|m| Symbol{char: String::from(m.as_str()), position: Position{row, start: m.start(), end: m.end()-1}})
                .collect::<Vec<Symbol>>()
            );
            row += 1;
        }
        Schematic{numbers, symbols}
    }

    pub fn parts(&self) -> Vec<usize> {
        self.numbers.iter().cartesian_product(self.symbols.iter()).filter(|pair| adjacent(pair.0,pair.1)).map(|pair| pair.0.value).collect::<Vec<usize>>()
    }
}

fn adjacent(number:&Number,symbol:&Symbol) -> bool {
    let row_valid = number.position.row.abs_diff(symbol.position.row) <= 1;
    let col_valid = number.position.start <= symbol.position.start+1 && symbol.position.end <= number.position.end+1;
    row_valid && col_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 4361);
    }
}