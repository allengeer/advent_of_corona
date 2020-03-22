use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let mut start_idx: usize = 0;
    let mut longest: usize = 0;
    let mut in_streak = false;

    let args = Cli::from_args();
    let input_string = std::fs::read_to_string(&args.path).expect("could not read file");

    let mut numbers: Vec<i32> = input_string.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
    
    for i in 0..numbers.len() {
        if in_streak {
            if (numbers[i] <= numbers[i-1]) || (numbers[i] % 2 + numbers[i-1] % 2 != 1) {
                longest = std::cmp::max(i - start_idx, longest);
                in_streak = false;
                // println!("Streak {} - {} - Longest {}" , start_idx, i-start_idx, longest);
            }
        } else {
            in_streak = true;
            start_idx = i;
        }
    }
    println!("Part 1 Answer: {}", longest);

    numbers.sort();
    numbers.dedup();
    println!("Part 2 Answer: {}" , numbers.len());
}