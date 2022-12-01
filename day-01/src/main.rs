use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    println!("Reading file: {}", file_path);

    let contents = read_file(file_path);
    let [largest, last, elf] = &contents.lines().fold([0, 0, 0], |acc, line| {
        let [mut largest, mut curr, mut elf_index] = acc;

        if line == "" {
            elf_index += 1;
            if curr > largest {
                largest = curr;
            }
            curr = 0;
            return [largest, curr, elf_index];
        }

        let line_val = line.parse::<i32>();

        if line_val.is_ok() {
            curr += line_val.unwrap();
        }

        return [largest, curr, elf_index];
    });

    println!("Elf {elf} is carrying {largest} calories, get him!");
    // dbg!(args);
}

fn read_file(file_path: &str) -> String {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    format!("{contents}\n")
}
