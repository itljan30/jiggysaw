mod puzzle;
    
use crate::puzzle::Puzzle;

fn main() {
    let shapes = 3;
    let mut s = Vec::new();
    for i in 0..1000000 {
        let mut puzzle = Puzzle::get_random(5, 5, shapes);
        let solutions = puzzle.shuffle().get_solutions(u32::MAX as usize);
        s.push(solutions.len());

        print!("\rProgress: {:>5.2}%", i as f32 / 1000000.0 * 100.0);
        // println!("{}", solutions.len());
        // if solutions.len() == 2 {
        //     for (i, p) in solutions.iter().enumerate() {
        //         println!("Solution: {}\n{:?}\n--------------------------------", i + 1, p);
        //     }
        // }
    }

    println!("\rProgress: 100.00%\nAverage solutions with at most {} unique shapes: {}", shapes, s.iter().sum::<usize>() as f32 / s.len() as f32);
}
