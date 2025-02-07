mod puzzle;
    
use crate::puzzle::Puzzle;

enum Func {
    Average,
    TwoSolutions,
}

enum Argument {
    Length(usize),
    Height(usize),
    MaxShapes(usize),
    PuzzleNumber(usize),
    TotalSolutions(usize),
}

fn get_avg(args: &Vec<Argument>) {
    let mut number_of_puzzles = 1000;
    let mut length = 5;
    let mut height = 5;
    let mut max_shapes = 10;
    let mut total_solutions = u32::MAX as usize;

    for arg in args {
        match arg {
            Argument::Length(val) => length = *val,
            Argument::Height(val) => height = *val,
            Argument::MaxShapes(val) => max_shapes = *val,
            Argument::PuzzleNumber(val) => number_of_puzzles = *val,
            Argument::TotalSolutions(val) => total_solutions = *val,
        }
    }

    let mut s = Vec::new();
    for i in 0..number_of_puzzles {
        let mut puzzle = Puzzle::get_random(length, height, max_shapes);
        let solutions = puzzle.shuffle().get_solutions(total_solutions);
        s.push(solutions.len());

        print!("\rProgress: {:>7}/{} : {:>5.2}%",i, number_of_puzzles, i as f32 / number_of_puzzles as f32 * 100.0);
    }
    println!("\rProgress:{:>7}/{} : 100.00%\nAverage solutions with at most {} unique shapes across {} puzzles: {}", number_of_puzzles, number_of_puzzles, max_shapes, number_of_puzzles, s.iter().sum::<usize>() as f32 / s.len() as f32);
}

fn two_solutions(_args: &Vec<Argument>) {
    eprintln!("Not Implemented: twosolutions");
}

fn display_help_info() {
    eprintln!("Commands:
    average              => Generates puzzles and calculates the average number of solutions per puzzle
    twosolutions(TODO)   => Generates puzzles and writes all that are below with only two unique solutions
                                and is below the similarity threshold to a file(TODO add a default file)
Options: 
    --help          | -h => Displays this help page
    --version       | -v => Displays the current version
    --length        | -x => Sets the length of the puzzles generated
    --height        | -y => Sets the height of the puzzles generated
    --max-shapes    | -s => Sets the maximum amount of unique shapes per puzzle
    --max-solutions | -S => Sets the max number of solutions it will search for
    --puzzle-count  | -p => Sets the number of puzzles generated
    --file(TODO)    | -f => Sets the file jiggysaw will write to")
}

fn display_version_info() {
    eprintln!("JiggySaw v0.1.0")
}

fn main() {
    let mut functions: Vec<Func> = Vec::new();
    let mut context: Vec<Argument> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "average" => functions.push(Func::Average),
            "twosolutions" => functions.push(Func::TwoSolutions),
            "--version" | "-v" => {
                display_version_info();
                std::process::exit(0);
            }
            "--help" | "-h" => {
                display_help_info();
                std::process::exit(0);
            }
            "--length" | "-x" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        context.push(Argument::Length(num));
                    }
                    else {
                        eprintln!("Unable to parse value: {} {}", arg, val);
                        std::process::exit(1);
                    }
                }
                else {
                    eprintln!("Value was not given for argument: {}", arg);
                    std::process::exit(1);
                }
            }
            "--height" | "-y" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        context.push(Argument::Height(num));
                    }
                    else {
                        eprintln!("Unable to parse value: {} {}", arg, val);
                        std::process::exit(1);
                    }
                }
                else {
                    eprintln!("Value was not given for argument: {}", arg);
                    std::process::exit(1);
                }
            }
            "--max-shapes" | "-s" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        context.push(Argument::MaxShapes(num));
                    }
                    else {
                        eprintln!("Unable to parse value: {} {}", arg, val);
                        std::process::exit(1);
                    }
                }
                else {
                    eprintln!("Value was not given for argument: {}", arg);
                    std::process::exit(1);
                }
            }
            "--puzzle-count" | "-p" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        context.push(Argument::PuzzleNumber(num));
                    }
                    else {
                        eprintln!("Unable to parse value: {} {}", arg, val);
                        std::process::exit(1);
                    }
                }
                else {
                    eprintln!("Value was not given for argument: {}", arg);
                    std::process::exit(1);
                }
            }
            "--total-solutions" | "-t" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        context.push(Argument::TotalSolutions(num));
                    }
                    else {
                        eprintln!("Unable to parse value: {} {}", arg, val);
                        std::process::exit(1);
                    }
                }
                else {
                    eprintln!("Value was not given for argument: {}", arg);
                    std::process::exit(1);
                }
            }
            a => {
                eprintln!("Invalid command: {}", a);
                std::process::exit(1);
            }
        }
    }

    if functions.len() == 0 {
        display_help_info();
        std::process::exit(0);
    }

    for function in functions {
        match function {
            Func::Average => get_avg(&context),
            Func::TwoSolutions => two_solutions(&context),
        }
    }
}
