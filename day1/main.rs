// Day 1
use std::fs::read_to_string;

fn bubble_sort(arr: &mut Vec<i64>) -> &mut Vec<i64> {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..arr.len()-1 {
            if arr[i] < arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    return arr;
}

fn main() {
    
    let mut sum:i64 = 0;
    let mut v = Vec::new();

    for line in read_to_string("data").unwrap().lines() {
        if line == "" {
            v.push(sum);
            sum = 0;   
        }
        else {
            let my_integer:i64 = line.parse().unwrap();
            sum += my_integer;
        }
    }
    
    let result:&mut Vec<i64> = bubble_sort(&mut v);
    println!("Advent of code day 1");
    sum = 0;
    for i in 0..3 {
        let calories = result[i];
        println!("Elf {} carrying {}", i+1, calories);
        sum += calories
    }
    
    println!("Result: {}",sum);
}
