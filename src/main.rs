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
    let mut pc = 1; // Program counter starts after "Tralalero Tralala"

    while pc < lines.len() - 1 {
        pc = parse_and_execute(pc, &lines, &mut variables);
    }
}

fn get_value(s: &str, variables: &HashMap<String, String>) -> Option<f64> {
    if let Ok(num) = s.parse::<f64>() {
        Some(num)
    } else if let Some(val_str) = variables.get(s) {
        val_str.parse::<f64>().ok()
    } else {
        None
    }
}

fn parse_and_execute(pc: usize, lines: &Vec<&str>, variables: &mut HashMap<String, String>) -> usize {
    let line = lines[pc].trim();
    let mut words = line.split_whitespace();

    if let Some(keyword) = words.next() {
        if keyword == "Biscottini" {
            if let Some(var_name) = words.next() {
                let value = words.collect::<Vec<&str>>().join(" ");
                variables.insert(var_name.to_string(), value.trim_matches('"').to_string());
            }
            return pc + 1;
        } else if keyword == "Matteeeo" {
            let expression = words.collect::<Vec<&str>>().join(" ");
            if expression.starts_with('"') && expression.ends_with('"') {
                println!("{}", expression.trim_matches('"'));
            } else {
                if let Some(value) = variables.get(&expression) {
                    println!("{}", value);
                }
            }
            return pc + 1;
        } else if keyword == "Pinguino" {
            if let (Some(word2), Some(word3), Some(times_str)) = (words.next(), words.next(), words.next()) {
                if word2 == "Arrabiato" && word3 == "Fruti" {
                    if let Ok(times) = times_str.parse::<i32>() {
                        let loop_start = pc + 2; // After "Pinguino..." and "{"
                        let mut loop_end = loop_start;
                        let mut brace_count = 1;

                        for i in loop_start..lines.len() {
                            if lines[i].trim() == "{" {
                                brace_count += 1;
                            }
                            if lines[i].trim() == "}" {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    loop_end = i;
                                    break;
                                }
                            }
                        }

                        for _ in 0..times {
                            let mut inner_pc = loop_start;
                            while inner_pc < loop_end {
                                inner_pc = parse_and_execute(inner_pc, lines, variables);
                            }
                        }
                        return loop_end + 1;
                    }
                }
            }
        } else if keyword == "Chimpanzini" {
            if let (Some(var_name), Some(op1_str), Some(op), Some(op2_str)) = (words.next(), words.next(), words.next(), words.next()) {
                if let (Some(op1), Some(op2)) = (get_value(op1_str, variables), get_value(op2_str, variables)) {
                    let result = match op {
                        "+" => op1 + op2,
                        "-" => op1 - op2,
                        "*" => op1 * op2,
                        "/" => op1 / op2,
                        _ => 0.0,
                    };
                    variables.insert(var_name.to_string(), result.to_string());
                }
            }
            return pc + 1;
        } else if keyword == "Tung" {
            if let (Some(word2), Some(word3), Some(op1_str), Some(op), Some(op2_str)) = (words.next(), words.next(), words.next(), words.next(), words.next()) {
                if word2 == "Tung" && word3 == "Tung" {
                    if let (Some(op1), Some(op2)) = (get_value(op1_str, variables), get_value(op2_str, variables)) {
                        let condition = match op {
                            "==" => op1 == op2,
                            "!=" => op1 != op2,
                            ">" => op1 > op2,
                            "<" => op1 < op2,
                            ">=" => op1 >= op2,
                            "<=" => op1 <= op2,
                            _ => false,
                        };

                        let if_block_start = pc + 2;
                        let mut if_block_end = if_block_start;
                        let mut brace_count = 1;

                        for i in if_block_start..lines.len() {
                            if lines[i].trim() == "{" {
                                brace_count += 1;
                            }
                            if lines[i].trim() == "}" {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    if_block_end = i;
                                    break;
                                }
                            }
                        }

                        let mut next_pc = if_block_end + 1;

                        if condition {
                            let mut inner_pc = if_block_start;
                            while inner_pc < if_block_end {
                                inner_pc = parse_and_execute(inner_pc, lines, variables);
                            }
                        } else {
                            // Check for an else block
                            if let Some(next_line) = lines.get(if_block_end + 1) {
                                let mut next_words = next_line.trim().split_whitespace();
                                if let Some(else_keyword) = next_words.next() {
                                    if else_keyword == "Ballerina" && next_words.next() == Some("Cappuccina") {
                                        let else_block_start = if_block_end + 3;
                                        let mut else_block_end = else_block_start;
                                        let mut brace_count = 1;

                                        for i in else_block_start..lines.len() {
                                            if lines[i].trim() == "{" {
                                                brace_count += 1;
                                            }
                                            if lines[i].trim() == "}" {
                                                brace_count -= 1;
                                                if brace_count == 0 {
                                                    else_block_end = i;
                                                    break;
                                                }
                                            }
                                        }

                                        let mut inner_pc = else_block_start;
                                        while inner_pc < else_block_end {
                                            inner_pc = parse_and_execute(inner_pc, lines, variables);
                                        }
                                        next_pc = else_block_end + 1;
                                    }
                                }
                            }
                        }
                        return next_pc;
                    }
                }
            }
        }
    }
    pc + 1
}
