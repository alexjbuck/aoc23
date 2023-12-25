use itertools::Itertools;

pub fn run() {
    println!("Starting Part 1");
    let input = include_str!("input.txt");
    println!("Part 1: {}", process(input));
}

fn process(input: &str) -> usize {
    let expanded_rows = expand_rows(input);
    let space = expand_columns(expanded_rows.as_str());
    let galaxies = space.lines().enumerate().map(|(r,row)| {
        row.chars().enumerate().filter_map(|(c,cell)| {
            match cell {
                '#' => Some((r,c)),
                _ => None,
            }
        }).collect::<Vec<(usize,usize)>>()
    }).flatten().collect::<Vec<(usize,usize)>>();
    galaxies.iter().combinations(2).map(|c| {
        let g1 = c[0];
        let g2 = c[1];
        g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)
    }).sum()
}



fn expand_rows(input: &str) -> String {
    let mut rows = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let expansion_rows = rows.iter().enumerate().filter_map(|(r, row)| {
        match row.iter().all(|c| *c == '.') {
            true => Some(r),
            false => None,
        }
    }).collect::<Vec<_>>();
    let new_row = vec!['.'; rows[0].len()];
    expansion_rows.iter().enumerate().for_each(|(index,row)| {
        rows.insert(*row+index, new_row.clone());
    });
    rows.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
}

fn expand_columns(input: &str) -> String {
    let num_columns = input.lines().next().unwrap().len();
    let num_rows = input.lines().count();
    let mut columns = (0..num_columns)
        .map(|i| {
            input.lines().map(|l| l.chars().nth(i).unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let expansion_columns = columns.iter().enumerate().filter_map(|(c, column)| {
        match column.iter().all(|c| *c == '.') {
            true => Some(c),
            false => None,
        }
    }).collect::<Vec<_>>();
    let new_column = vec!['.'; columns[0].len()];
    expansion_columns.iter().enumerate().for_each(|(index,column)| {
        columns.insert(*column+index, new_column.clone());
    });
    (0..num_rows).map(|i| columns.iter().map(|r| r.iter().nth(i).unwrap()).collect::<String>()).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(process(input),374);
    }
}