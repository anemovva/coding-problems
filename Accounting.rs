// https://open.kattis.com/problems/bokforing

use std::collections::HashMap;
use std::io::stdin;
use std::collections::HashSet;

fn main(){

    let mut people: HashMap<i64, i64> = HashMap::new();
    let mut firstline = String::new();
    stdin().read_line(&mut firstline);
    let nums = firstline.split_whitespace().collect::<Vec<_>>();
    let numlines = nums[1].parse::<i64>().unwrap();
    let numpeople = nums[0].parse::<i64>().unwrap();
    
    let mut defaultval = 0;

    for line in 0..numlines{
        let mut line = String::new();
        stdin().read_line(&mut line);
        let command = line.split_whitespace().collect::<Vec<_>>();

        if command[0]=="SET"{
            let i = command[1].parse::<i64>().unwrap();
            let x = command[2].parse::<i64>().unwrap();
            people.insert(i,x);
        }
        else if command[0]=="PRINT"{
            let i = command[1].parse::<i64>().unwrap();
            if (people.contains_key(&i)){
                println!("{}", people.get(&i).unwrap())
            }
            else {println!("{}", defaultval);}
        }
        else {
            // Reset
            let x = command[1].parse::<i64>().unwrap();
            people.clear();
            defaultval = x;
        }
    }

}