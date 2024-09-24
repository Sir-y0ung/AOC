// Day 4
use std::fs;
use std::collections::HashSet;

fn is_subset(subset: &[String], main_array: &[String]) -> bool {
    let main_set: HashSet<_> = main_array.iter().cloned().collect();
    let subset_set: HashSet<_> = subset.iter().cloned().collect();
    
    subset_set.is_subset(&main_set)
}
fn make_pairs(arr:&Vec<&str>) -> (Vec<String>, Vec<String>) {
    let mut part1:Vec<String> = vec![String::from("")];
    let mut part2:Vec<String> = vec![String::from("")];
    
    for i in 0..2 {
        if i == 0 {
            let a = arr[i].split("-").collect::<Vec<&str>>();
            let b:i32 = a[0].parse().unwrap();
            let c:i32 = a[1].parse().unwrap();
            part1 = (b..=c).step_by(1).collect::<Vec<i32>>().iter().map(|num| num.to_string()).collect::<Vec<String>>();
        }
        else {
            let a = arr[i].split("-").collect::<Vec<&str>>();
            let b = a[0].parse().unwrap();
            let c = a[1].parse().unwrap();
            part2 = (b..=c).step_by(1).collect::<Vec<i32>>().iter().map(|num| num.to_string()).collect::<Vec<String>>();
        }
    }
    (part1, part2)
}

fn check_if_pair(arr:&Vec<&str>) -> bool {
    let (part1, part2) = make_pairs(arr);
    if  is_subset(&part1, &part2)  || is_subset(&part2, &part1){
        return  true;
    } 
    else {
        return false;
    }
}

fn check_if_overlap(arr:&Vec<&str>) -> bool {
    let (part1, part2) = make_pairs(arr);
    for i in part1 {
        if part2.contains(&i) {
            return true
        }
    }
    false
}

fn main() {
    let file_content = fs::read_to_string("D:\\Coddingg\\Rust\\AdventOfCode\\src\\data").expect("Failed to read");
    let mut sum1:u32 = 0;
    let mut sum2:u32 = 0;

    println!("Advent of code day 4");

    for i in file_content.lines() {
        let arr =  i.split(",").collect::<Vec<&str>>();
        if check_if_pair(&arr) {
            sum1 += 1;
        }

        if check_if_overlap(&arr) {
            sum2 += 1;
        }
    }
    println!("Task1: Sum is {}", sum1);
    println!("Task2: Sum is {}", sum2);
}