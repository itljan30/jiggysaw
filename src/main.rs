use rand::{Rng, seq::SliceRandom};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Edge {
    Innie(u32),
    Outie(u32),
    Edge,
}

impl Edge {
    fn get_debug_format(&self) -> String {
        match self {
            Edge::Edge => format!("E"),
            Edge::Innie(val) => format!("I{}", val),
            Edge::Outie(val) => format!("O{}", val),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PieceType {
    Edge,
    Corner,
    Inner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Piece {
    up: Edge,
    right: Edge,
    down: Edge,
    left: Edge,
    piece_type: PieceType,
}

impl Piece {
    fn rotate(&mut self) {
        use std::mem::swap;
        swap(&mut self.up, &mut self.right);
        swap(&mut self.right, &mut self.down);
        swap(&mut self.down, &mut self.left);
    }
}

#[derive(Clone)]
struct Puzzle {
    length: usize,
    height: usize,
    pieces: Vec<Piece>,
}

/*
    Returns a 3 x 3 puzzle with no repeated shapes.
    ▒ => Piece
    E => Edge
    I => Innie
    O => Outie
    # => Shape

         E          E          E          
        E▒▒O1     I1▒▒O2     I2▒▒E    
         I8         O10        O3         

         O8         I10        I3         
        E▒▒O12   I12▒▒O13   I13▒▒E    
         I7         O11        O4         

         O7         I11        I4         
        E▒▒I6     O6▒▒I5     O5▒▒E    
         E          E          E
*/
impl Default for Puzzle {
    fn default() -> Self {
        Self {
            length: 3,
            height: 3,
            pieces: vec![
                Piece{up: Edge::Edge,     right: Edge::Outie(1),  down: Edge::Innie(8), left: Edge::Edge, piece_type: PieceType::Corner}, Piece{up: Edge::Edge,      right: Edge::Outie(2),  down: Edge::Outie(10), left: Edge::Innie(1),  piece_type: PieceType::Edge},  Piece{up: Edge::Edge,     right: Edge::Edge, down: Edge::Outie(3), left: Edge::Innie(2),  piece_type: PieceType::Corner},
                Piece{up: Edge::Outie(8), right: Edge::Outie(12), down: Edge::Innie(7), left: Edge::Edge, piece_type: PieceType::Edge},   Piece{up: Edge::Innie(10), right: Edge::Outie(13), down: Edge::Outie(11), left: Edge::Innie(12), piece_type: PieceType::Inner}, Piece{up: Edge::Innie(3), right: Edge::Edge, down: Edge::Outie(4), left: Edge::Innie(13), piece_type: PieceType::Edge},
                Piece{up: Edge::Outie(7), right: Edge::Innie(6),  down: Edge::Edge,     left: Edge::Edge, piece_type: PieceType::Corner}, Piece{up: Edge::Innie(11), right: Edge::Innie(5),  down: Edge::Edge,      left: Edge::Outie(6),  piece_type: PieceType::Edge},  Piece{up: Edge::Innie(4), right: Edge::Edge, down: Edge::Edge,     left: Edge::Outie(5),  piece_type: PieceType::Corner},
            ]
        }
    }
}

impl std::fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for chunk in self.pieces.chunks(self.length) {
            output.push_str("    ");

            // top row
            for &piece in chunk {
                output.push_str(format!("{:11}", piece.up.get_debug_format()).as_str())
            }
            output.push('\n');

            // mid row
            for &piece in chunk {
                output.push_str(format!("{:>4}▒▒{:4} ", piece.left.get_debug_format(), piece.right.get_debug_format()).as_str());
            }
            output.push('\n');

            // bottom row
            output.push_str("    ");
            for &piece in chunk {
                output.push_str(format!("{:11}", piece.down.get_debug_format()).as_str())
            }
            output.push_str("\n\n");
        }

        write!(f, "{}", output[..output.len() - 2].to_string())
    }
}

impl Puzzle {
    fn get_random(length: usize, height: usize) -> Self {
        Self::default()
    }

    fn get_solutions(&self) -> Option<Vec<Puzzle>> {
        let mut solutions: Vec<Vec<Piece>> = Vec::new();
        let mut pieces: HashMap<PieceType, HashMap<Edge, Vec<Piece>>> = HashMap::new();

        for &piece in self.pieces.iter() {
            let category = pieces.entry(piece.piece_type).or_default();
            for &edge in &[piece.up, piece.right, piece.down, piece.left] {
                category.entry(edge).or_default().push(piece);
            }
        }

        // TODO add error handling to this lol
        let mut starting_piece = pieces.get(&PieceType::Corner)?.get(&Edge::Edge)?[0];
        for &edge in &[starting_piece.up, starting_piece.right, starting_piece.down, starting_piece.left] {
            pieces.entry(starting_piece.piece_type).or_default()
                .entry(edge).or_default()
                .retain(|&p| p != starting_piece);
        }

        while starting_piece.up != Edge::Edge || starting_piece.left != Edge::Edge {
            starting_piece.rotate();
        }

        self.rec_solve(&mut solutions, &mut pieces, &mut vec![starting_piece]);

        Some(solutions.into_iter().map(|solution| {
            Puzzle {
                length: self.length,
                height: self.height,
                pieces: solution,
            }
        }).collect::<Vec<Puzzle>>())
    }

    // TODO use a simple loop instead of recursion for optimization
    fn rec_solve(
        &self, 
        solutions: &mut Vec<Vec<Piece>>, 
        pieces: &mut HashMap<PieceType, HashMap<Edge, Vec<Piece>>>, 
        curr: &mut Vec<Piece>
    ) {
        if curr.len() == self.length * self.height {
            solutions.push(curr.clone());
            return;
        }

        let next_piece_type = self.get_piece_type_from_index(curr.len());

        let mut above_target = None;
        if curr.len() >= self.length {
            above_target = match curr[curr.len() - self.length].down {
                Edge::Innie(val) => Some(Edge::Outie(val)),
                Edge::Outie(val) => Some(Edge::Innie(val)),
                Edge::Edge =>       Some(Edge::Edge),
            };
        }

        let left_target = match curr.last().unwrap().right {
            Edge::Innie(val) => Edge::Outie(val),
            Edge::Outie(val) => Edge::Innie(val),
            Edge::Edge       => Edge::Edge,
        };

        let mut valid_pieces = Vec::new();

        let potential_pieces = pieces.get(&next_piece_type).unwrap().get(&left_target).unwrap();
        for &piece in potential_pieces {
            let mut piece_copy = piece;
            for _ in 0..4 {
                piece_copy.rotate();
                if let Some(up_target) = above_target {
                    if piece_copy.up != up_target {
                        continue;
                    }
                }

                if piece_copy.left != left_target {
                    continue;
                }

                // FIX pieces are being added to valid_pieces more than once, !.contains() is a
                // band-aid fix for now
                if !valid_pieces.contains(&piece_copy) { valid_pieces.push(piece_copy) };
            }
        }

        for piece in valid_pieces {
            for &edge in &[piece.up, piece.right, piece.down, piece.left] {
                pieces.get_mut(&piece.piece_type).unwrap()
                    .get_mut(&edge).unwrap()
                    .retain(|&p| p != piece);
            }
            curr.push(piece);

            self.rec_solve(solutions, pieces, curr);
            if curr.len() > 10 {
                return;
            }

            curr.pop();
            let category = pieces.get_mut(&piece.piece_type).unwrap();
            for &edge in &[piece.up, piece.right, piece.down, piece.left] {
                category.get_mut(&edge).unwrap().push(piece);
            }
        }
    }

    fn get_piece_type_from_index(&self, index: usize) -> PieceType {
        // piece should be a corner piece
        if index == 0 || 
            index == self.length - 1 || 
            index == self.length * (self.height - 1) ||
            index == self.length * self.height - 1
            {
            return PieceType::Corner;
        }

        // piece should be an edge piece
        else if index < self.length - 1 || 
            index % self.length == 0 || 
            (index + 1) % self.length == 0 ||
            index > self.length * (self.height - 1) 
            {
            return PieceType::Edge;
        }

        // piece should be an inner piece
        else {
            return PieceType::Inner;
        }
        
    }

    fn shuffle(&mut self) -> &mut Self {
        let mut rng = rand::rng();

        self.pieces.shuffle(&mut rng);

        for piece in &mut self.pieces {
            let rotations = rng.random_range(0..4);
            for _ in 0..rotations {
                piece.rotate();
            }
        }

        self
    }
}

fn main() {
    let mut puzzle = Puzzle::default();

    println!("--------------------------------");
    println!("Start:\n{:?}", puzzle);
    println!("--------------------------------");
    println!("Shuffled:\n{:?}", puzzle.shuffle());
    println!("--------------------------------");
    if let Some(solutions) = puzzle.get_solutions() {
        for (i, p) in solutions.iter().enumerate() {
            println!("Solution: {}\n{:?}\n--------------------------------", i + 1, p);
        }
    }
}
