use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

// extract the first numeric digit from a string, convert to int
fn first_number(input: &str) -> char {
    input.chars().find(|c| c.is_digit(10)).unwrap()
}

fn first_number_corrected(input: &str, num_dict: &HashMap<String, &str>, re: &Regex) -> char {
    let replaced = re.replace_all(&input, |caps: &regex::Captures| {
        num_dict.get(caps.get(0).unwrap().as_str()).unwrap()
    });
    replaced.chars().find(|c| c.is_digit(10)).unwrap()
}

// run first_number on each line of a file
fn part_one()->i32 {
    let filename = "input.txt";
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let mut sum = 0;
    for line in contents.lines() {
        let first = first_number(line);
        let last = first_number(&line.chars().rev().collect::<String>());
        let number = format!("{}{}", first, last).parse::<i32>().unwrap();
        sum += number;
        println!("{}{}={}  {}", first, last, number, line);
    }
    println!("sum: {}", sum);
    sum
}


fn part_two()->i32 {
    let filename = "input.txt";
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let num_dict: HashMap<String, &str> = [
        ("one".to_string(), "1"),
        ("two".to_string(), "2"),
        ("three".to_string(), "3"),
        ("four".to_string(), "4"),
        ("five".to_string(), "5"),
        ("six".to_string(), "6"),
        ("seven".to_string(), "7"),
        ("eight".to_string(), "8"),
        ("nine".to_string(), "9"),
        ("zero".to_string(), "0"),
    ].into();
    let num_dict_rev: HashMap<String,&str> = num_dict.iter().map(|(k,&v)| (k.chars().rev().collect(),v)).collect();
    let re = Regex::new(&num_dict.keys().map(|k| k.clone()).collect::<Vec<String>>().join("|")).unwrap();
    let re_rev = Regex::new(&num_dict_rev.keys().map(|k| k.clone()).collect::<Vec<String>>().join("|")).unwrap();

    let mut sum = 0;
    for line in contents.lines() {
        let first = first_number_corrected(line, &num_dict, &re);
        let last = first_number_corrected(&line.chars().rev().collect::<String>(), &num_dict_rev, &re_rev);
        let number = format!("{}{}", first, last).parse::<i32>().unwrap();
        sum += number;
        println!("{}{}={}  {}", first, last, number, line);
    }
    println!("sum: {}", sum);
    sum
}

fn main() {
    let sum1 = part_one();
    println!("*********************************************");
    let sum2 = part_two();
    println!("sum1: {}  sum2: {}", sum1, sum2);
}
