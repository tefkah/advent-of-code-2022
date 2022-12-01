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
    let (top3, last) = &contents.lines().fold(
        (
            [
                Elf {
                    index: 0,
                    calories: 0,
                },
                Elf {
                    index: 0,
                    calories: 0,
                },
                Elf {
                    index: 0,
                    calories: 0,
                },
            ],
            Elf {
                index: 0,
                calories: 0,
            },
        ),
        |acc, line| -> ([Elf; 3], Elf) {
            let (mut top3, mut curr) = acc;

            if line == "" {
                curr.index += 1;

                top3 = if top3.iter().any(|elf| curr.calories > elf.calories) {
                    [top3[0], top3[1], curr]
                } else {
                    top3
                };

                top3.sort_by(|a, b| b.calories.cmp(&a.calories));

                curr = Elf {
                    index: curr.index,
                    calories: 0,
                };

                return (top3, curr);
            }

            let line_val = line.parse::<i32>();

            if line_val.is_ok() {
                curr.calories += line_val.unwrap();
            }

            return (top3, curr);
        },
    );

    let sumOfCalories = top3.iter().fold(0, |acc, elf| acc + elf.calories);

    println!(
        "The top 3 elves carried this many calories: {}",
        sumOfCalories
    );
    println!("This were the elves: {:?}", top3);

    // dbg!(args);
}

fn read_file(file_path: &str) -> String {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    format!("{contents}\n")
}

#[derive(Debug, Copy, Clone)]
struct Elf {
    index: i32,
    calories: i32,
}
