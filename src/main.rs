use clap::Parser;

mod year2015;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: u64,

    #[arg(short, long)]
    day: u64,
}

pub struct Answer {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

fn run(year: u64, day: u64, input: String) -> Option<Answer> {
    match year {
        2015 => year2015::run(day, input),
        _ => None,
    }
}

fn malformed() -> ! {
    panic!("Malformed input");
}

fn main() {
    let args = Args::parse();

    let input_path = format!("inputs/{}/{:02}.txt", args.year, args.day);

    let exists = std::path::Path::new(&input_path).exists();

    if !exists {
        println!("Input file {} does not exist", input_path);
        return;
    }

    let input = std::fs::read_to_string(input_path).unwrap();

    let answer = run(args.year, args.day, input);

    if let Some(answer) = answer {
        println!("Year {} Day {}", args.year, args.day);

        println!(
            "Part 1: {}",
            answer.part1.unwrap_or("No solution found".to_string())
        );

        println!(
            "Part 2: {}",
            answer.part2.unwrap_or("No solution found".to_string())
        );
    } else {
        println!("No solution found for year {} day {}", args.year, args.day);
    }
}
