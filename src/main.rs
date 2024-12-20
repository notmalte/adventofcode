use clap::Parser;
use inquire::Confirm;

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;
mod year2021;
mod year2022;
mod year2023;
mod year2024;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: u64,

    #[arg(short, long)]
    day: u64,
}

pub struct Answer {
    pub part1: String,
    pub part2: String,
}

fn run(year: u64, day: u64, input: &str) -> Option<Answer> {
    match year {
        2015 => year2015::run(day, input),
        2016 => year2016::run(day, input),
        2017 => year2017::run(day, input),
        2018 => year2018::run(day, input),
        2019 => year2019::run(day, input),
        2020 => year2020::run(day, input),
        2021 => year2021::run(day, input),
        2022 => year2022::run(day, input),
        2023 => year2023::run(day, input),
        2024 => year2024::run(day, input),
        _ => None,
    }
}

fn malformed(year: &str, day: &str) -> ! {
    panic!("Malformed input for year {} day {}", year, day);
}

fn main() {
    let args = Args::parse();

    let input_path = format!("inputs/{}/{:02}.txt", args.year, args.day);

    let exists = std::path::Path::new(&input_path).exists();

    if !exists {
        println!("Input file {} does not exist", input_path);

        let confirm = Confirm::new("Do you want to create it?")
            .with_default(true)
            .prompt();

        if let Ok(true) = confirm {
            std::fs::write(&input_path, "").unwrap();
            println!("Created input file {}", input_path);
        }

        return;
    }

    let input = std::fs::read_to_string(input_path).unwrap();

    let answer = run(args.year, args.day, &input);

    if let Some(answer) = answer {
        println!("Year {} Day {}", args.year, args.day);

        println!("Part 1: {}", answer.part1);

        println!("Part 2: {}", answer.part2);
    } else {
        println!("No solution found for year {} day {}", args.year, args.day);
    }
}
