use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file to run");
        return;
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Could not read the file");

    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() || lines[0] != "Tralalero Tralala" {
        println!("The program must start with 'Tralalero Tralala'");
        return;
    }

    if lines.last() != Some(&"Bombardiro Crocodilo") {
        println!("The program must end with 'Bombardiro Crocodilo'");
        return;
    }

    let mut variables: HashMap<String, String> = HashMap::new();

    for line in lines.iter().skip(1).rev().skip(1).rev() {
        parse_and_execute(line, &mut variables);
    }
}

fn parse_and_execute(line: &str, variables: &mut HashMap<String, String>) {
    let mut words = line.split_whitespace();
    if let Some(keyword) = words.next() {
        if keyword == "Biscottini" {
            if let Some(var_name) = words.next() {
                let value = words.collect::<Vec<&str>>().join(" ");
                variables.insert(var_name.to_string(), value.trim_matches('"').to_string());
            }
        } else if keyword == "Matteeeo" {
            if let Some(expression) = words.next() {
                if expression.starts_with('"') && expression.ends_with('"') {
                    println!("{}", expression.trim_matches('"'));
                } else {
                    if let Some(value) = variables.get(expression) {
                        println!("{}", value);
                    }
                }
            }
        }
    }
}