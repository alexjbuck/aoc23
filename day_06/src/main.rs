fn main() {
    println!("Part 1: {}",part1(include_str!("input.txt")));
    println!("Part 2: {}",part2(include_str!("input.txt")));
}


/// Based on speed = t, distance = d = t*(T-t)
/// Times that beat best distance d are those between the roots of d = t*(T-t)
/// Which is t*T-t^2 = d => t^2 - t*T + d = 0
/// Quadratic quation: t = (T +- sqrt(T^2 - 4*d))/2
fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().map(|d| d.parse::<f64>()).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect::<Vec<f64>>();
    let distances = lines.next().unwrap().split_whitespace().map(|d| d.parse::<f64>()).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect::<Vec<f64>>();
    dbg!(&times);
    dbg!(&distances);
    let combinations = times.iter().zip(distances.iter()).map(|(t,d)| delta(t,d)).product::<f64>() as u32;
    combinations
}

/// Based on speed = t, distance = d = t*(T-t)
/// Times that beat best distance d are those between the roots of d = t*(T-t)
/// Which is t*T-t^2 = d => t^2 - t*T + d = 0
/// Quadratic quation: t = (T +- sqrt(T^2 - 4*d))/2
fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines.next().unwrap()[5..].split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>().join("").parse::<usize>().unwrap();
    let distances = lines.next().unwrap()[9..].split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>().join("").parse::<usize>().unwrap();
    dbg!(&times);
    dbg!(&distances);
    let combinations = delta(&(times as f64), &(distances as f64));
    combinations as u32
}

fn delta(t: &f64, d: &f64) -> f64 {
    let upper = ((t + (t.powf(2.0)-4.0*d).sqrt())/2.0 - 1.0).ceil();
    let lower = ((t - (t.powf(2.0)-4.0*d).sqrt())/2.0 + 1.0).floor();
    dbg!(upper - lower + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(part1("Time:      7  15   30
        Distance:  9  40  200"), 288);
    }
}