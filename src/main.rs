use rand::{Rng, seq::SliceRandom};
use rand::prelude::IndexedRandom;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Edge {
    Innie(usize),
    Outie(usize),
    Edge,
}

impl Edge {
    fn random(start: usize, end: usize) -> Self {
        let mut rng = rand::rng();
        let options = vec![
            Edge::Innie(rng.random_range(start..end)),
            Edge::Outie(rng.random_range(start..end)),
        ];
        *options.choose(&mut rng).unwrap()
    }

    fn inverse(&self) -> Self {
        match &self {
            Edge::Innie(val) => Edge::Outie(*val),
            Edge::Outie(val) => Edge::Innie(*val),
            Edge::Edge => Edge::Edge,
        }
    }

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

impl Piece {
    fn new(up: Edge, right: Edge, down: Edge, left: Edge) -> Self {
        let mut edge_count = 0;
        for &edge in &[up, right, down, left] {
            if edge == Edge::Edge {
                edge_count += 1;
            }
        }

        Self {
            up,
            right,
            down,
            left,
            piece_type: match edge_count {
                0 => PieceType::Inner,
                1 => PieceType::Edge,
                2 => PieceType::Corner,
                _ => panic!("Piece was generated with more than 2 Edges"),
            }
        }
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
                Piece::new(Edge::Edge,     Edge::Outie(1),  Edge::Innie(8), Edge::Edge), Piece::new(Edge::Edge,      Edge::Outie(2),  Edge::Outie(10), Edge::Innie(1)),  Piece::new(Edge::Edge,     Edge::Edge, Edge::Outie(3), Edge::Innie(2)),
                Piece::new(Edge::Outie(8), Edge::Outie(12), Edge::Innie(7), Edge::Edge), Piece::new(Edge::Innie(10), Edge::Outie(13), Edge::Outie(11), Edge::Innie(12)), Piece::new(Edge::Innie(3), Edge::Edge, Edge::Outie(4), Edge::Innie(13)),
                Piece::new(Edge::Outie(7), Edge::Innie(6),  Edge::Edge,     Edge::Edge), Piece::new(Edge::Innie(11), Edge::Innie(5),  Edge::Edge,      Edge::Outie(6)),  Piece::new(Edge::Innie(4), Edge::Edge, Edge::Edge,     Edge::Outie(5)),
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
    fn get_random(length: usize, height: usize, total_shapes: usize) -> Self {
        const UP: usize = 0;
        const RIGHT: usize = 1;
        const DOWN: usize = 2;
        const LEFT: usize = 3;
        let mut puzzle: Vec<[Option<Edge>; 4]> = vec![[None; 4]; length * height];

        for (i, piece) in puzzle.iter_mut().enumerate() {
            if i < length {
                piece[UP] = Some(Edge::Edge);
            }
            if (i + 1) % length == 0 {
                piece[RIGHT] = Some(Edge::Edge);
            }
            if i >= length * (height - 1) {
                piece[DOWN] = Some(Edge::Edge);
            }
            if i % length == 0 {
                piece[LEFT] = Some(Edge::Edge);
            }
        }

        for i in 0..puzzle.len() {
            if let None = puzzle[i][UP] {
                let edge = Edge::random(0, total_shapes);
                puzzle[i][UP] = Some(edge);
                puzzle[i - length][DOWN] = Some(edge.inverse());
            }
            if let None = puzzle[i][RIGHT] {
                let edge = Edge::random(0, total_shapes);
                puzzle[i][RIGHT] = Some(edge);
                puzzle[i + 1][LEFT] = Some(edge.inverse());
            }
        }

        Self {
            length,
            height,
            pieces: puzzle.iter().map(|&piece|
                Piece::new(
                    piece[UP].unwrap(),
                    piece[RIGHT].unwrap(),
                    piece[DOWN].unwrap(),
                    piece[LEFT].unwrap(),
                )
            ).collect(),
        }
    }

    fn get_solutions(&self, max_solutions: usize) -> Vec<Puzzle> {
        let mut pieces: HashMap<PieceType, HashMap<Edge, Vec<Piece>>> = HashMap::new();

        for &piece in self.pieces.iter() {
            let category = pieces.entry(piece.piece_type).or_default();
            for &edge in &[piece.up, piece.right, piece.down, piece.left] {
                let cat = category.entry(edge).or_default();
                if !cat.contains(&piece) {
                    cat.push(piece);
                }
            }
        }

        let mut starting_piece = pieces.get(&PieceType::Corner).unwrap().get(&Edge::Edge).unwrap()[0];
        while starting_piece.up != Edge::Edge || starting_piece.left != Edge::Edge {
            starting_piece.rotate();
        }

        let mut solutions: Vec<Vec<Piece>> = Vec::new();
        let mut curr: Vec<Piece> = Vec::new();
        let mut stack: Vec<Vec<Piece>> = vec![vec![starting_piece]];

        while !stack.is_empty() {
            if solutions.len() >= max_solutions {
                break;
            }

            if curr.len() == self.length * self.height {
                solutions.push(curr.clone());

                // NOTE pop might cause a bug, but I don't think so
                stack.pop();
            }

            let valid_pieces = stack.last_mut().unwrap();

            if let Some(piece) = valid_pieces.pop() {
                for &edge in &[piece.up, piece.right, piece.down, piece.left] {
                    pieces.get_mut(&piece.piece_type).unwrap()
                        .get_mut(&edge).unwrap()
                        .retain(|&p| p != piece);
                }
                curr.push(piece);
                stack.push(self.get_valid_next_pieces(&curr, &pieces).unwrap_or(Vec::new()));
            }
            else {
                stack.pop();
                let piece = curr.pop().unwrap();
                let category = pieces.get_mut(&piece.piece_type).unwrap();
                for &edge in &[piece.up, piece.right, piece.down, piece.left] {
                    category.get_mut(&edge).unwrap().push(piece);
                }
            }
        }

        solutions.into_iter().map(|solution| {
            Puzzle {
                length: self.length,
                height: self.height,
                pieces: solution,
            }
        }).collect::<Vec<Puzzle>>()
    }

    fn get_valid_next_pieces(
        &self,
        curr: &Vec<Piece>, 
        pieces: &HashMap<PieceType, HashMap<Edge, Vec<Piece>>>,
    ) -> Option<Vec<Piece>> {
        let mut above_target = None;
        if curr.len() >= self.length {
            above_target = Some(curr[curr.len() - self.length].down.inverse());
        }

        let left_target = curr.last().unwrap().right.inverse();

        let mut valid_pieces = Vec::new();

        let next_piece_type = self.get_piece_type_from_index(curr.len());

        let potential_pieces = pieces.get(&next_piece_type)?.get(&left_target)?;
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

                valid_pieces.push(piece_copy);
            }
        }

        Some(valid_pieces)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_piece_type_from_index() {
        let puzzle1 = Puzzle {
            length: 3,
            height: 3,
            pieces: Vec::new(),
        };
        assert_eq!(puzzle1.get_piece_type_from_index(5), PieceType::Edge);
        assert_eq!(puzzle1.get_piece_type_from_index(0), PieceType::Corner);
        assert_eq!(puzzle1.get_piece_type_from_index(4), PieceType::Inner);

        let puzzle2 = Puzzle {
            length: 100,
            height: 100,
            pieces: Vec::new(),
        };
        assert_eq!(puzzle2.get_piece_type_from_index(0), PieceType::Corner);
        assert_eq!(puzzle2.get_piece_type_from_index(5), PieceType::Edge);
        assert_eq!(puzzle2.get_piece_type_from_index(101), PieceType::Inner);
        assert_eq!(puzzle2.get_piece_type_from_index(9999), PieceType::Corner);
        assert_eq!(puzzle2.get_piece_type_from_index(9988), PieceType::Edge);
    }

    #[test]
    fn test_get_solutions() {
        let mut puzzle = Puzzle::get_random(5, 5, 10);
        for _ in 0..1000 {
            let solutions = puzzle.shuffle().get_solutions(10);
            for puzzle in solutions {
                for (i, &piece) in puzzle.pieces.iter().enumerate() {
                    if i > puzzle.length {
                        assert_eq!(puzzle.pieces[i - puzzle.length].down, piece.up.inverse());
                    }

                    if i > 0 {
                        assert_eq!(puzzle.pieces[i - 1].right, piece.left.inverse());
                    }
                }
            }
        }
    }
}
    

fn main() {
    let mut puzzle = Puzzle::get_random(2, 2, 10);

    println!("--------------------------------");
    println!("Start:\n{:?}", puzzle);
    println!("--------------------------------");
    for _ in 0..1000 {
    println!("Shuffled:\n{:?}", puzzle.shuffle());
    println!("--------------------------------");
    let solutions = puzzle.get_solutions(3);
    for (i, p) in solutions.iter().enumerate() {
        println!("Solution: {}\n{:?}\n--------------------------------", i + 1, p);
    }
    }
}
