use std::env;
use std::fs;

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

    for line in lines.iter().skip(1).rev().skip(1).rev() {
        parse_and_execute(line);
    }
}

fn parse_and_execute(line: &str) {
    if line.starts_with("Matteeeo bambini gusini") {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() > 1 {
            let string_to_print = parts[1]
                .trim_start_matches("bambini gusini")
                .trim()
                .trim_matches('"');
            println!("{}", string_to_print);
        }
    }
}
