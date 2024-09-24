// Day 5
use regex::Regex;
use std::fs;

fn move_crates(stacks_of_crats: &mut[Vec<&str>; 9], moves:Vec<&str>) {
    let re = Regex::new(r"\d+").unwrap();
    
    for i in moves {
        let mut pos:[usize;3] = [0,0,0];
        let mut j = 0;

        for cap in re.find_iter(i) {
            if let Ok(num) = cap.as_str().parse::<usize>() {
                pos[j] = num;
            } else {
                println!("Failed to parse the number");
            }
            j += 1
        }
        for k in 0..pos[0]  {
            let x = stacks_of_crats[pos[1] - 1].pop();
            stacks_of_crats[pos[2] - 1].push(x.unwrap());
        } 
    }
}

fn smart_move_crates(stacks_of_crats: &mut[Vec<&str>; 9], moves:Vec<&str>) {
    let re = Regex::new(r"\d+").unwrap();
    
    for i in moves {
        let mut pos:[usize;3] = [0,0,0];
        let mut j = 0;

        for cap in re.find_iter(i) {
            if let Ok(num) = cap.as_str().parse::<usize>() {
                pos[j] = num;
            } else {
                println!("Failed to parse the number");
            }
            j += 1
        }
        let mut temp_vec:Vec<&str> = Vec::new();
        for k in 0..pos[0]  {
            let x: Option<&str> = stacks_of_crats[pos[1] - 1].pop();
            temp_vec.push(x.unwrap());
        }
        temp_vec = temp_vec.iter().rev().cloned().collect();
        stacks_of_crats[pos[2] - 1].extend(temp_vec);
    }
}

fn main() {
    let file_content = fs::read_to_string("D:\\Coddingg\\Rust\\AdventOfCode\\src\\data").expect("Failed to read");
    let mut stacks_of_crats:[Vec<&str>; 9] = Default::default();
    let mut moves:Vec<&str> = Vec::new();

    println!("Advent of code day 5");
    for i in file_content.lines() {
        moves.push(i);
    }
    stacks_of_crats[0] = ["F", "T","C","L","R","P","G","Q"].to_vec();
    stacks_of_crats[1] = ["N", "Q","H","W","R","F","S","J"].to_vec();
    stacks_of_crats[2] = ["F", "B","H","W","P","M","Q"].to_vec();
    stacks_of_crats[3] = ["V", "S","T","D","F"].to_vec();
    stacks_of_crats[4] = ["Q", "L","D","W","V","F","Z"].to_vec();
    stacks_of_crats[5] = ["Z", "C","L","S"].to_vec();
    stacks_of_crats[6] = ["Z", "B","M","V","D","F"].to_vec();
    stacks_of_crats[7] = ["T", "J","B"].to_vec();
    stacks_of_crats[8] = ["Q", "N","B","G","L","S","P","H"].to_vec();

    // move_crates(&mut stacks_of_crats, moves);
    // print!("Task1: ");
    // for i in stacks_of_crats {
    //     print!("{}", i.last().unwrap());
    // }

    smart_move_crates(&mut stacks_of_crats, moves);
    print!("Task2: ");
    for i in stacks_of_crats {
        print!("{}", i.last().unwrap());
    } 
    
}