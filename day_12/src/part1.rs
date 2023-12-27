use std::fmt;
use std::collections::HashMap;

pub fn run() {
    println!("Starting Part 1");
    println!("Part 1: {}", process(include_str!("input.txt")));
}

#[derive(Clone,Copy,Eq)]
enum Spring {
    Working,
    Damaged,
    Unknown,
}

struct Row(Vec<Spring>);

impl PartialEq for Spring {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Spring::Working, Spring::Working) => true,
            (Spring::Damaged, Spring::Damaged) => true,
            (Spring::Unknown, _) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Working => write!(f, "."),
            Spring::Damaged => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for spring in self.0.iter() {
            s.push_str(&format!("{:?}", spring));
        }
        write!(f, "{}", s)
    }

}

fn process(input:&str) -> usize {
    input.lines().map(parse_line).map(|input| {
        println!("** Processing {:?} {:?} {:?}", Row(input.0.clone()), &input.1, &input.2);
        let result = arrangements(input,0);
        println!("** Found {} arrangements **", result);
        println!();
        result
    }).sum::<usize>()
}

fn parse_line(line:&str) -> (Vec<Spring>,HashMap<usize,usize>, Vec<usize>) {
    let (row,count) = line.split_once(' ').unwrap();
    let counts = count.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let row = row.chars().map(|c| match c {
        '?' => Spring::Unknown,
        '#' => Spring::Damaged,
        '.' => Spring::Working,
        _ => panic!("Invalid input"),
    }).collect::<Vec<_>>();
    let mapping: HashMap<usize,usize> = HashMap::new();
    (row, mapping, counts)
}

fn arrangements(line: (Vec<Spring>, HashMap<usize,usize>, Vec<usize>),level:usize) -> usize {
    let (row, starts, counts) = line;
    if counts.len() == 0 {
        if row.iter().filter(|s| Spring::Damaged == **s ).count() == starts.values().sum::<usize>() + counts.iter().sum::<usize>() {
            println!("{} oo Found a valid arrangement: {:?}",padding(&level,"."), Row(row.clone()));
            return 1;
        } else {
            println!("{}Too many damaged springs: {:?}",padding(&level,"."), Row(row.clone()));
            return 0;
        }
    }
    let count = counts[0];
    let candidates = 0..=row.len()-count;
    let placements = candidates.filter(|candidate| {
        is_ordered(&starts,candidate) && is_valid(&row, *candidate, count)
    });
    if placements.clone().count() == 0 {
        println!("{}No valid placements with {:?} remaining",padding(&level,"."),counts);
        return 0;
    };
    let combos = placements.map(|start| {
        (place_pattern(&row, start, count),start.clone())
    }).map(|(new_row,start)| {
        let mut starts = starts.clone();
        starts.insert(start,count);
        println!("{}Placing pattern:{} {:?} => {:?} {:?} {:?}",padding(&level,"."),padding(&(6-level)," "), Row(row.clone()), Row(new_row.clone()),starts,counts[1..].to_vec());
        arrangements((new_row.clone() ,starts, counts[1..].to_vec()),level+1)
    }).sum::<usize>();
    println!("{}Found {:?} combos for starts {:?} : {:?}",padding(&level,"."),combos, starts, Row(row.clone()));
    combos
}

fn place_pattern(row: &Vec<Spring>, start: usize, count:usize) -> Vec<Spring> {
    let mut new_row = row.clone();
    if start > 0 {
        new_row[start-1] = Spring::Working;
    }
    for i in start..start+count {
        new_row[i] = Spring::Damaged;
    }
    if start+count < row.len() {
        new_row[start+count] = Spring::Working;
    }
    
    new_row
}

fn is_ordered(starts: &HashMap<usize,usize>, candidate: &usize) -> bool {
    starts.keys().all(|start| start < candidate)
}

fn is_valid(row: &Vec<Spring>, start: usize, count: usize) -> bool {
    let mut match_row = row.clone();
    match_row.insert(0, Spring::Working);
    match_row.push(Spring::Working);
    let pattern:Vec<Spring> = vec![vec![Spring::Working], vec![Spring::Damaged;count], vec![Spring::Working]].into_iter().flatten().collect();
    match_row[start..start+pattern.len()] == pattern
}

fn padding(level:&usize,c:&str) -> String {
    let mut s = String::new();
    for _ in 0..*level {
        s.push_str(c);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process(input), 21);
    }

    #[test]
    fn is_valid_test() {
        let row = vec![Spring::Unknown, Spring::Working, Spring::Working, Spring::Damaged, Spring::Damaged, Spring::Unknown, Spring::Working, Spring::Working, Spring::Working];
        assert_eq!(is_valid(&row, 3, 3), true);
        assert_eq!(is_valid(&row, 3, 4), false);
        assert_eq!(is_valid(&row, 3, 2), true);
        assert_eq!(is_valid(&row, 3, 1), false);
        assert_eq!(is_valid(&row, 4, 2), false);
        assert_eq!(is_valid(&row, 0, 1), true);
    }
}