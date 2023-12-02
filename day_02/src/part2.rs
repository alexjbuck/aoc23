use crate::game::Game;

pub fn run() {
    println!("Part 2: {}",process(include_str!("input.txt")))
}


fn process(input: &str) -> u32 {
    input.lines().map(|l| Game::from(l).required_cubes().power()).sum()
}  


// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input :&str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = 2286;
        assert_eq!(process(input), output);
    }
}