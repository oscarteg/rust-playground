fn main() {
    println!("Hello, world!");
}

// Starting position in FEN notation
const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Color {
    BLACK,
    WHITE,
}

pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Pieces;
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

pub enum Action {
    Move,
    Capture,
    AcceptDraw,
    DeclareDraw,
    Resign(Player),
}

const BASE_RATING: i32 = 1200;

pub struct Player {
    name: &'static str,
    rating: i32,
    wins: i32,
    losses: i32,
    draws: i32,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            rating: self.rating,
            wins: self.wins,
            losses: self.losses,
            draws: self.draws,
        }
    }
}

impl Player {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            rating: BASE_RATING,
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn play(&self, _opponent: &Player) {
        todo!()
    }

    fn total_games(&self) -> i32 {
        self.wins + self.losses + self.draws
    }

    /// Returns the performance rating of the player against the given opponent.
    /// The performance rating is calculated using the formula:
    /// (opponent rating + 400 * (wins - losses)) / total games
    /// The performance rating is used to calculate the new rating of the player
    /// after a game.
    /// See https://en.wikipedia.org/wiki/Elo_rating_system#Performance_rating
    /// for more details.
    /// # Examples
    /// ```
    /// let player1 = Player::new();
    /// let player2 = Player::new();
    /// assert_eq!(player1.performance_rating(&player2), 1200);
    /// ```
    /// ```
    /// let player1 = Player::new();
    /// let player2 = Player::new();
    /// player1.wins += 1;
    /// assert_eq!(player1.performance_rating(&player2), 1400);
    /// ```
    fn performance_rating(&self, opponent: &Player) -> i32 {
        (opponent.rating + 400 * (self.wins - self.losses)) / self.total_games()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Board(pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    /// Board for each side
    bb_sides: [Board; 2],
    // BitBoards for all pieces and each side
    bb_pieces: [[Board; 6]; 2],
}

impl Board {
    fn from_fen(fen: &str) -> Self {
        let s = fen.split(' ').collect::<Vec<&str>>();

        let b = s[0];

        let x = b.chars().fold(Board(0), |acc, c| {
            let mut acc = acc;
            if c.is_ascii_digit() {
                acc.0 <<= c.to_digit(10).unwrap();
            } else {
                acc.0 <<= 1;
            }
            acc
        });

        println!("TESTING {:?}", x);
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_fen() {
        // Test case 1: FEN string representing an empty board
        let fen1 = "8/8/8/8/8/8/8/8 w - - 0 1";
        let expected1 = 0; // Assuming 0 represents an empty board
        let board1 = Board::from_fen(fen1);
        assert_eq!(board1.0, expected1);

        // Test case 2: FEN string representing a board with some pieces
        let fen2 = "rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let expected2 = 0x18244281AAFF6600; // Example bitboard representation
        let board2 = Board::from_fen(fen2);
        assert_eq!(board2.0, expected2);

        // Test case 3: FEN string representing a different board
        let fen3 = "4k3/4p3/4P3/8/8/8/8/4K3 w - - 0 1";
        let expected3 = 0x0000000000888800; // Example bitboard representation
        let board3 = Board::from_fen(fen3);
        assert_eq!(board3.0, expected3);

        // Add more test cases as needed
    }
}
