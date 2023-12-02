use std::vec;

pub fn run() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}
#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}
#[derive(Debug)]
struct Draw {
    cubes: Vec<Cube>,
}
#[derive(Debug)]
struct Cube {
    color: String,
    count: u32,
}

impl From<&str> for Cube {
    fn from(s:&str) -> Self {
        let mut words = s.split_whitespace();
        let count = words.next().unwrap().parse::<u32>().unwrap();
        let color = words.next().unwrap().to_string();
        Cube { color, count }
    }
}

impl From<&str> for Draw {
    fn from(s: &str) -> Self {
        let cubes = s.split(",").map(|s| Cube::from(s.trim())).collect::<Vec<Cube>>();
        Draw { cubes }
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let line = s.lines().nth(0).unwrap();
        let gamestring = line.split(":").nth(0).unwrap();
        let drawstring = line.split(":").nth(1).unwrap();
        let id: u32 = gamestring.split_whitespace().nth(1).unwrap().trim().parse::<u32>().unwrap();
        let draws = drawstring.split(";").map(|s| Draw::from(s.trim())).collect::<Vec<Draw>>();
        Game { id, draws }
    }
}

impl Draw {
    fn power(&self) -> u32 {
        let mut power = 1;
        for cube in self.cubes.iter() {
            power *= cube.count;
        }
        power
    }
}

impl Game {
    fn required_cubes(&self) -> Draw {
        let red = Cube { color: String::from("red"), count: 0 };
        let green = Cube { color: String::from("green"), count: 0 };
        let blue = Cube { color: String::from("blue"), count: 0 };
        let mut required_cubes:Draw = Draw { cubes: vec![red, green, blue] };
        for draw in self.draws.iter() {
            for cube in draw.cubes.iter() {
                for rc in required_cubes.cubes.iter_mut() {
                    if rc.color == cube.color {
                        if rc.count < cube.count {
                            rc.count = cube.count;
                        }
                    }
                }
            }
        }
        required_cubes
    }
}

fn process(input: &str) -> u32 {
    let games = input.lines().map(|l| Game::from(l)).collect::<Vec<Game>>();
    games.iter().map(|g| g.required_cubes().power()).sum()
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
        dbg!(&output); 
        dbg!(process(input));
        assert_eq!(process(input), output);
    }
}