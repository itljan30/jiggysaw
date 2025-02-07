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
    PuzzleNumber(u32),
}

fn get_avg(args: &Vec<Argument>) {
    let mut number_of_puzzles = 1000;
    let mut length = 5;
    let mut height = 5;
    let mut max_shapes = 10;

    for arg in args {
        match arg {
            Argument::Length(val) => length = *val,
            Argument::Height(val) => height = *val,
            Argument::MaxShapes(val) => max_shapes = *val,
            Argument::PuzzleNumber(val) => number_of_puzzles = *val,
        }
    }

    let mut s = Vec::new();
    for i in 0..number_of_puzzles {
        let mut puzzle = Puzzle::get_random(length, height, max_shapes);
        let solutions = puzzle.shuffle().get_solutions(u32::MAX as usize);
        s.push(solutions.len());

        print!("\rProgress: {:>7}/{} : {:>5.2}%",i, number_of_puzzles, i as f32 / number_of_puzzles as f32 * 100.0);
    }
    println!("\rProgress:{:>7}/{} : 100.00%\nAverage solutions with at most {} unique shapes across {} puzzles: {}", number_of_puzzles, number_of_puzzles, max_shapes, number_of_puzzles, s.iter().sum::<usize>() as f32 / s.len() as f32);
}

fn two_solutions(args: &Vec<Argument>) {

}

fn main() {
    let mut functions: Vec<Func> = Vec::new();
    let mut context: Vec<Argument> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "average" => functions.push(Func::Average),
            "twosolutions" => functions.push(Func::TwoSolutions),
            "--length" | "-l" => {
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
            "--height" | "-h" => {
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
            "--max-shapes" | "-m" => {
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
            "--total-puzzles" | "-p" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<u32>() {
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
            a => {
                eprintln!("Invalid command: {}", a);
                std::process::exit(1);
            }
        }
    }

    for function in functions {
        match function {
            Func::Average => get_avg(&context),
            Func::TwoSolutions => two_solutions(&context),
        }
    }
}
