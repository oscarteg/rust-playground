fn main() {
    let player1: Player = Player::new();
    let player2: Player = Player::new();

    player1.play(&player2);
}

enum Color {
    BLACK,
    WHITE,
}

/// A piece on a board.
///
/// Every piece has both a color and a position.
/// These, combined with the type of piece it is,
/// determine things like
/// 1. The validity of legal moves
/// 2. The validity of legal attacks
/// 3. Move generation
/// 4. Material and positional value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

const BASE_RATING: i32 = 400;

struct Player {
    rating: i32,
    wins: i32,
    losses: i32,
    draws: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            rating: 1200,
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn play(&self, opponent: &Player) {}

    fn total_games(&self) -> i32 {
        self.wins + self.losses + self.draws
    }

    fn performance_rating(&self, opponent: &Player) -> i32 {
        (opponent.rating + 400 * (self.wins - self.losses)) / self.total_games()
    }
}

struct Game {
    player1: Player,

    player2: Player,
}

#[derive(Debug)]
struct Board {
    field: Type,
}

struct Move {}

impl Game {
    fn play_game() {}

    fn fen() {}

    fn pgn() {}
}
