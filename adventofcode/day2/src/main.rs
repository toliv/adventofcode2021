use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Processing file {}", filename);
    process_file(filename);
    process_file2(filename);
}

fn process_file(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        let mut y_val:i32 = 0;
        let mut x_val:i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(' ').collect();
                let direction : &str = v[0];
                // Unwrap the Result into an int
                let amount : i32 = v[1].trim().parse().unwrap();
                if direction == "forward"{
                    x_val += amount;
                }
                else{
                    let mut sign : i32 = -1;
                    if direction == "down"{
                        sign = 1;
                    }

                    y_val += sign * amount;
                }
            }
        }
        println!("Part 1: Horizontal : {}, Vertical: {}, Multiplier: {}", x_val, y_val, x_val*y_val);
    }
}

fn process_file2(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        let mut y_val:i32 = 0;
        let mut x_val:i32 = 0;
        let mut aim:i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(' ').collect();
                let direction : &str = v[0];
                // Unwrap the Result into an int
                let amount : i32 = v[1].trim().parse().unwrap();
                if direction == "forward"{
                    x_val += amount;
                    y_val += aim * amount;
                }
                else{
                    let mut sign : i32 = -1;
                    if direction == "down"{
                        sign = 1;
                    }
                    aim += sign * amount;
                }
            }
        }
        println!("Part 2: Horizontal : {}, Vertical: {}, Multiplier: {}", x_val, y_val, x_val*y_val);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}