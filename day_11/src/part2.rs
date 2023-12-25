use itertools::Itertools;

pub fn run() {
    println!("Starting Part 1");
    let input = include_str!("input.txt");
    println!("Part 2: {}", process(input,1000000));
}

fn process(input: &str,scale:usize) -> usize {
    let ex_rows = expansion_rows(input);
    let ex_columns = expansion_columns(input);
    let galaxies = input.lines().enumerate().map(|(r,row)| {
        row.chars().enumerate().filter_map(|(c,cell)| {
            match cell {
                '#' => Some((r,c)),
                _ => None,
            }
        }).collect::<Vec<(usize,usize)>>()
    }).flatten().collect::<Vec<(usize,usize)>>();
    println!("Expansion Rows: {:?}",ex_rows.iter().map(|n| n.to_string()).join(","));
    println!("Expansion Columns: {:?}",ex_columns.iter().map(|n| n.to_string()).join(","));
    galaxies.iter().combinations(2).map(|c| {
        let g1 = c[0];
        let g2 = c[1];
        let min_row = g1.0.min(g2.0);
        let max_row = g1.0.max(g2.0);
        let min_column = g1.1.min(g2.1);
        let max_column = g1.1.max(g2.1);
        let ex_rows = ex_rows.iter().filter(|r| (min_row..max_row).contains(r)).count();
        let ex_columns = ex_columns.iter().filter(|c| (min_column..max_column).contains(c)).count();
        let dist = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1) + ex_rows*(scale-1) + ex_columns*(scale-1);
        dist
    }).sum()
}




fn expansion_rows(input: &str) -> Vec<usize> {
    let rows = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    rows.iter().enumerate().filter_map(|(r, row)| {
        match row.iter().all(|c| *c == '.') {
            true => Some(r),
            false => None,
        }
    }).collect::<Vec<_>>()
}

fn expansion_columns(input: &str) -> Vec<usize> {
    let num_columns = input.lines().next().unwrap().len();
    let columns = (0..num_columns)
        .map(|i| {
            input.lines().map(|l| l.chars().nth(i).unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    columns.iter().enumerate().filter_map(|(c, column)| {
        match column.iter().all(|c| *c == '.') {
            true => Some(c),
            false => None,
        }
    }).collect::<Vec<_>>()
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
    assert_eq!(process(input,2),374);
    assert_eq!(process(input,10),1030);
    assert_eq!(process(input,100),8410);
    }
}