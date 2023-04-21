use std::fmt::Display;

fn main() {
    let player1: Player = Player::new();
    let player2: Player = Player::new();

    player1.play(&player2);
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Color {
    BLACK,
    WHITE,
}

const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = FILE_A << 1;
const FILE_C: u64 = FILE_A << 2;
const FILE_D: u64 = FILE_A << 3;
const FILE_E: u64 = FILE_A << 4;
const FILE_F: u64 = FILE_A << 5;
const FILE_G: u64 = FILE_A << 6;
const FILE_H: u64 = FILE_A << 7;

const RANK_1: u64 = 0xFF;
const RANK_2: u64 = RANK_1 << 8;
const RANK_7: u64 = RANK_1 << 48;
const RANK_8: u64 = RANK_1 << 56;

pub enum Action {
    Move,
    Capture,
    AcceptDraw,
    DeclareDraw,
    Resign(Player),
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
///
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

const BASE_RATING: i32 = 1200;

struct Player {
    rating: i32,
    wins: i32,
    losses: i32,
    draws: i32,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            rating: self.rating,
            wins: self.wins,
            losses: self.losses,
            draws: self.draws,
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    /// Board for each side
    bb_sides: [Board; 2],
    // BitBoards for all pieces and each side
    bb_pieces: [[Board; 6]; 2],

    state: State,
}

struct Square {
    file: usize,
    rank: usize,
}

/// Contains castling_rights, move_clocks, en_passant_square if possible and the side to move
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    half_move_counter: u8,
    stm: usize,
}

enum CastlingRights {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}

struct FenError;

impl Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FenError")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
struct Board {
    pieces: [u64; 6],
    occupancy: u64,
    en_passant: u64,
    castling: u64,
    side_to_move: Color,
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            pieces: [0; 6],
            occupancy: 0,
            en_passant: 0,
            castling: 0b1111,
            side_to_move: Color::WHITE,
        };

        // Set up pawns
        board.pieces[0] = RANK_2 | RANK_7;
        // Set up knights
        board.pieces[1] = (FILE_B | FILE_G) & (RANK_1 | RANK_8);
        // Set up bishops
        board.pieces[2] = (FILE_C | FILE_F) & (RANK_1 | RANK_8);
        // Set up rooks
        board.pieces[3] = (FILE_A | FILE_H) & (RANK_1 | RANK_8);
        // Set up queens
        board.pieces[4] = (FILE_D | FILE_E) & (RANK_1 | RANK_8);
        // Set up kings
        board.pieces[5] = (FILE_E) & (RANK_1 | RANK_8);
        // Set up occupancy
        for piece in &board.pieces {
            board.occupancy |= piece;
        }
        board
    }
}

trait Move {
    fn possible_moves(&self) -> Vec<Position>;

    fn is_valid_move(&self, position: Position) -> bool {
        self.possible_moves().contains(&position)
    }
}

struct Game<'a> {
    black: &'a Player,
    white: &'a Player,
}

impl Game {
    fn init(&mut self, black: &Player, white: &Player) -> &Game {
        let game = Self { black, white };
        &game
    }

    fn play_turn(&self, action: Action) {}

    fn fen() {}

    fn pgn() {}
}
