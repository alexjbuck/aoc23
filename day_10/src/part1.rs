use std::{rc::Rc, collections::HashMap, cell::RefCell, ops::Deref, cmp::min};

pub fn run() {
    println!("Starting Part 1");
    let input = include_str!("input.txt");
    println!("Part 1: {}", process(input));
}

fn process(input: &str) -> usize {
    println!("Starting to process...");
    let mut grid = Grid::from(input);
    println!("Created Grid");
    let current = grid.current.clone().expect("No current point found.");
    let position = {
        current.borrow().position.clone()
    };
    let count = {
        current.borrow().count.expect("Current node does not have a count.").clone()
    };

    println!("Current Position: {:?} at {:?}",count, position);

    grid.advance();
    while grid.current.clone().expect("No current point found.").borrow().value.as_str() != "S" {
        let current = grid.current.clone().expect("No current point found.");
        let position = {
            current.borrow().position.clone()
        };
        let count = {
            current.borrow().count.expect("Current node does not have a count.").clone()
        };
        println!("Current Position: {:?} at {:?}",count, position);
        grid.advance();
    }
    
    println!("\nGoing backwards\n");

    grid.set_start();
    grid.reverse();
    while grid.current.clone().expect("No current point found.").borrow().value.as_str() != "S" {
        let current = grid.current.clone().expect("No current point found.");
        let position = {
            current.borrow().position.clone()
        };
        let count = {
            current.borrow().count.expect("Current node does not have a count.").clone()
        };
        println!("Current Position: {:?} at {:?}",count, position);
        grid.reverse();
    }
    grid.nodes.iter().filter_map(|(_,node)| match node.borrow().count {
        Some(count) => Some(count),
        None => None,
    }).max().expect("No max found.")
}

type Link<T> = Rc<RefCell<T>>;

// Make a Grid and node type and allow nodes to store their connections to neighbors
#[derive(Debug, Clone)]
struct Grid {
    rows: usize,
    columns: usize,
    nodes: HashMap<Position,Link<Node>>,
    start: Option<Link<Node>>,
    current: Option<Link<Node>>,
    prev: Option<Link<Node>>,
}

#[derive(Debug, Clone)]
struct Node {
    value: String,
    position: Position,
    count: Option<usize>,
    connections: Vec<Link<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Position { x, y }
    }
}
impl From<(usize,usize)> for Position{
    fn from((x,y): (usize, usize)) -> Self {
        Position {x: x as isize, y: y as isize}
    }
}
impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position {x,y }
    }
}
impl Position {
    fn up(&self) -> Self {
        Position::from((self.x, self.y - 1))
    }
    fn down(&self) -> Self {
        Position::from((self.x, self.y + 1))
    }
    fn left(&self) -> Self {
        Position::from((self.x - 1, self.y))
    }
    fn right(&self) -> Self {
        Position::from((self.x + 1, self.y))
    }
}

impl Drop for Grid {
    fn drop(&mut self) {
        self.nodes.iter().for_each(|(_,node)| node.borrow_mut().connections.clear());
    }
}


impl Grid {
    fn new(nodes: HashMap<Position,Link<Node>>, rows:usize, columns:usize) -> Self {
        Grid {
            nodes,
            rows,
            columns,
            start: None,
            current: None,
            prev: None,
        }
    }

    fn connect(& mut self) {
        for (position,node) in self.nodes.iter() {
            let mut node = node.borrow_mut();
            match node.value.as_str() {
                "|" => {
                    if let Some(up) = self.nodes.get(&position.up()) {
                        node.connections.push(up.clone());
                    }
                    if let Some(down) = self.nodes.get(&position.down()) {
                        node.connections.push(down.clone());
                    }
                },
                "-" => {
                    if let Some(left) = self.nodes.get(&position.left()) {
                        node.connections.push(left.clone());
                    }
                    if let Some(right) = self.nodes.get(&position.right()) {
                        node.connections.push(right.clone());
                    }
                },
                "L" => {
                    if let Some(up) = self.nodes.get(&position.up()) {
                        node.connections.push(up.clone());
                    }
                    if let Some(right) = self.nodes.get(&position.right()) {
                        node.connections.push(right.clone());
                    }
                },
                "J" => {
                    if let Some(left) = self.nodes.get(&position.left()) {
                        node.connections.push(left.clone());
                    }
                    if let Some(up) = self.nodes.get(&position.up()) {
                        node.connections.push(up.clone());
                    }
                },
                "F" => {
                    if let Some(down) = self.nodes.get(&position.down()) {
                        node.connections.push(down.clone());
                    }
                    if let Some(right) = self.nodes.get(&position.right()) {
                        node.connections.push(right.clone());
                    }
                },
                "7" => {
                    if let Some(left) = self.nodes.get(&position.left()) {
                        node.connections.push(left.clone());
                    }
                    if let Some(down) = self.nodes.get(&position.down()) {
                        node.connections.push(down.clone());
                    }
                },
                _ => {}
            }
        }
        for (position,node) in self.nodes.iter() {
            let mut node = node.borrow_mut();
            match node.value.as_str() {
                "S" => {
                    if let Some(left) = self.nodes.get(&position.left()) {
                        if matches!(left.deref().borrow().value.as_str(), "-" | "F" | "L") {
                            node.connections.push(left.clone())
                        }
                    }
                    if let Some(right) = self.nodes.get(&position.right()) {
                        if matches!(right.deref().borrow().value.as_str(), "-" | "J" | "7") {
                            node.connections.push(right.clone())
                        }
                    }
                    if let Some(up) = self.nodes.get(&position.up()) {
                        if matches!(up.deref().borrow().value.as_str(), "|" | "7" | "F") {
                            node.connections.push(up.clone())
                        }
                    }
                    if let Some(down) = self.nodes.get(&position.down()) {
                        if matches!(down.deref().borrow().value.as_str(), "|" | "J" | "L") {
                            node.connections.push(down.clone())
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn get_node(&self, position: &Position) -> Option<Link<Node>> {
        self.nodes.get(position).cloned()
    }

    fn get_start(&self) -> Option<Link<Node>> {
        self.nodes.iter().find(|(_,node)| node.borrow().value == "S").map(|(_,node)| node.clone())
    }
    fn set_start(&mut self) {
        self.start = self.get_start();
        if let Some(start) = self.start.clone() {
            start.borrow_mut().count = Some(0);
        }
        self.current = self.start.clone();
        self.prev = self.start.clone();
    }
    fn advance(&mut self) {
        let current = match self.current.clone() {
            Some(current) => current,
            None => self.start.clone().expect("No start found."),
        };
        let prev = match self.prev.clone() {
            Some(prev) => prev,
            None => self.start.clone().expect("No start found."),
        };
        // Set next equal to the first node in the current connections that is not the previous node
        let next = {
            current.borrow().connections.iter()
                .filter(|node| node.borrow().position != prev.borrow().position)
                .next()
                .expect("No next point found. Dead End.")
                .clone()
        };
        {
            let mut next = next.borrow_mut();
            let current = current.borrow();
            match next.count {
                Some(count) => {
                    next.count = Some(min(count, current.count.expect("Current node does not have a count.") + 1));
                },
                None => {
                    next.count = Some(current.count.expect("Current node does not have a count.") + 1);
                }
            }
        }
        self.prev = Some(current.clone());
        self.current = Some(next.clone());
    }
    
    fn reverse(&mut self) {
        let current = match self.current.clone() {
            Some(current) => current,
            None => self.start.clone().expect("No start found."),
        };
        let prev = match self.prev.clone() {
            Some(prev) => prev,
            None => self.start.clone().expect("No start found."),
        };
        // Set next equal to the first node in the current connections that is not the previous node
        let next = {
            current.borrow().connections.iter()
                .filter(|node| node.borrow().position != prev.borrow().position)
                .last()
                .expect("No last point found. Dead End.")
                .clone()
        };
        {
            let mut next = next.borrow_mut();
            let mut current = current.borrow_mut();
            match next.count {
                Some(count) => {
                    next.count = Some(min(count, current.count.expect("Current node does not have a count.") + 1));
                },
                None => {
                    current.count = Some(0);
                    next.count = Some(current.count.expect("Current node does not have a count.") + 1);
                }
            }
        }
        self.prev = Some(current.clone());
        self.current = Some(next.clone());
    }

}

impl From<&str> for Grid{
    fn from(input: &str) -> Self {
        let nodes: HashMap<Position,Link<Node>> = input
            .lines().enumerate()
            .map(|(y, line)| {
                line.chars().enumerate()
                    .map(move |(x,c)| ((x,y).into(), Rc::new(RefCell::new(Node::new(c.to_string(),(x,y).into())))))
            })
            .flatten().collect();
        println!("Finished making {} nodes.", nodes.len());
        let rows = input.lines().count();
        let columns = input.lines().next().unwrap().chars().count();
        let mut grid = Grid::new(nodes,rows,columns);
        grid.set_start();
        grid.connect();
        println!("Finished connecting nodes.");
        grid
    }
}


impl Node {
    fn new(value: String, position: Position) -> Self {
        Node {
            value,
            position,
            count: None,
            connections: vec![],
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        dbg!("HI!");
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        println!("starting test on\n{}", &input);
        assert_eq!(process(input), 8)
    }
}
