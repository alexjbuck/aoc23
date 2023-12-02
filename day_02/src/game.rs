use std::vec;

#[derive(Debug)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug)]
pub struct Draw {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
pub struct Cube {
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

impl Cube {
    pub fn new(color: &str, count: u32) -> Self {
        Cube { color: color.to_owned(), count }
    }

}

impl Draw {
    pub fn new(cubes: Vec<Cube>) -> Self {
        Draw { cubes }
    }
    pub fn power(&self) -> u32 {
        let mut power = 1;
        for cube in self.cubes.iter() {
            power *= cube.count;
        }
        power
    }
}

impl Game {
    pub fn new(id: u32, draws: Vec<Draw>) -> Self {
        Game { id, draws }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn required_cubes(&self) -> Draw {
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

    pub fn is_valid(&self, available_cubes: &Draw) -> bool {
        let required_cubes = self.required_cubes();
        for required_cube in required_cubes.cubes.iter() {
            for available_cube in available_cubes.cubes.iter() {
                if required_cube.color == available_cube.color {
                    if required_cube.count > available_cube.count {
                        return false;
                    }
                }
            }
        }
        true
    }
}