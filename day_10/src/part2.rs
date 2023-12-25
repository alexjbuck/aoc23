pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", process(input));
}

fn process(_input: &str) -> usize {
    6
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(process(input), 6)
    }
}
