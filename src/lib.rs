mod codes;
mod pgn;
mod psqt;
mod ranking;

use bit_vec::BitVec;
use huffman_compress::Book;
use shakmaty::san::{ParseSanError, SanError};
use shakmaty::{Chess, Move, Position};
use std::fmt;

#[derive(Debug)]
pub struct GameEncodeError {
    pub kind: GameEncodeErrorKind,
    pub explanation: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameEncodeErrorKind {
    IoError,
    HuffmanEncodeError,
    SanError,
    ParseSanError,
    IllegalMove,
}

impl std::error::Error for GameEncodeError {}

impl fmt::Display for GameEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to encode chess game: {}", &self.explanation)
    }
}

impl From<huffman_compress::EncodeError> for GameEncodeError {
    fn from(inner: huffman_compress::EncodeError) -> Self {
        Self {
            kind: GameEncodeErrorKind::HuffmanEncodeError,
            explanation: format!("Error during Huffman encoding: {}", inner),
        }
    }
}

impl From<std::io::Error> for GameEncodeError {
    fn from(inner: std::io::Error) -> Self {
        Self {
            kind: GameEncodeErrorKind::IoError,
            explanation: format!("I/O Error: {}", inner),
        }
    }
}

impl From<SanError> for GameEncodeError {
    fn from(inner: SanError) -> Self {
        Self {
            kind: GameEncodeErrorKind::SanError,
            explanation: format!("Illegal or ambiguous SAN: {}", inner),
        }
    }
}

impl From<ParseSanError> for GameEncodeError {
    fn from(inner: ParseSanError) -> Self {
        Self {
            kind: GameEncodeErrorKind::SanError,
            explanation: format!("Unable to parse SAN: {}", inner),
        }
    }
}

pub fn encode_game(moves: &[Move]) -> Result<BitVec, GameEncodeError> {
    let mut encoder = MoveByMoveEncoder::new();
    for m in moves {
        encoder.add_move(&m)?;
    }
    Ok(encoder.buffer)
}

pub fn encode_pgn<T: AsRef<str>>(pgn: T) -> Result<BitVec, GameEncodeError> {
    let mut reader = pgn_reader::BufferedReader::new_cursor(&pgn.as_ref()[..]);

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(BitVec::new()))?;
    Ok(bits)
}

pub fn encode_pgn_file<P: AsRef<std::path::Path>>(path: P) -> Result<BitVec, GameEncodeError> {
    let file = std::fs::File::open(path)?;
    let mut reader = pgn_reader::BufferedReader::new(file);

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(BitVec::new()))?;
    Ok(bits)
}

pub fn decode_game(bits: &BitVec) -> (Vec<Move>, Chess) {
    let (_, tree) = codes::code_from_lichess_weights();
    let ranks = tree.decoder(bits, 256);
    let mut moves = vec![];
    let mut pos = Chess::default();
    for rank in ranks {
        let m = ranking::from_position(&pos).remove(rank as usize);
        pos = pos.play(&m).unwrap();
        moves.push(m);
    }
    (moves, pos)
}

pub fn decode_move_by_move<T: MoveByMoveDecoder>(bits: &BitVec, decoder: &mut T) {
    let (_, tree) = codes::code_from_lichess_weights();
    let ranks = tree.decoder(bits, 256);
    let mut pos = Chess::default();
    for rank in ranks {
        let m = ranking::from_position(&pos).remove(rank as usize);
        pos = pos.play(&m).unwrap();
        decoder.decoded_move(m, &pos);
    }
}

pub struct MoveByMoveEncoder {
    book: Book<u8>,
    pos: Chess,
    pub buffer: BitVec,
}

impl MoveByMoveEncoder {
    pub fn new() -> Self {
        let (book, _) = codes::code_from_lichess_weights();
        Self {
            book,
            pos: Chess::default(),
            buffer: BitVec::new(),
        }
    }

    pub fn add_move(&mut self, m: &Move) -> Result<(), GameEncodeError> {
        match ranking::move_rank(&self.pos, m) {
            Some(rank) => {
                self.book.encode(&mut self.buffer, &(rank as u8))?;
                self.pos.play_unchecked(m);
            }
            None => {
                return Err(GameEncodeError {
                    kind: GameEncodeErrorKind::IllegalMove,
                    explanation: format!("Illegal move {}", m),
                })
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.pos = Chess::default();
        self.buffer.clear();
    }
}

impl Default for MoveByMoveEncoder {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MoveByMoveDecoder {
    fn decoded_move(&mut self, mv: Move, position: &Chess);
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::{Role, Square};

    #[test]
    fn encode_decode_consistency() {
        let moves = vec![
            Move::Normal {
                role: Role::Pawn,
                from: Square::D2,
                to: Square::D4,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Pawn,
                from: Square::E7,
                to: Square::E5,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Pawn,
                from: Square::D4,
                to: Square::E5,
                capture: Some(Role::Pawn),
                promotion: None,
            },
            Move::Normal {
                role: Role::King,
                from: Square::E8,
                to: Square::E7,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Queen,
                from: Square::D1,
                to: Square::D2,
                capture: None,
                promotion: None,
            },
        ];

        let encoded = encode_game(&moves).unwrap();
        let decoded = decode_game(&encoded).0;
        assert_eq!(decoded, moves);
    }

    #[test]
    fn expected_error() {
        let moves = vec![Move::Normal {
            role: Role::Pawn,
            from: Square::E2,
            to: Square::E5,
            capture: None,
            promotion: None,
        }];
        assert!(encode_game(&moves).is_err());
    }

    struct TestDecoder {
        moves: Vec<Move>,
    }

    impl MoveByMoveDecoder for TestDecoder {
        fn decoded_move(&mut self, mv: Move, _position: &Chess) {
            self.moves.push(mv);
        }
    }

    #[test]
    fn encode_decode_consistency_move_by_move() {
        let moves = vec![
            Move::Normal {
                role: Role::Pawn,
                from: Square::D2,
                to: Square::D4,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Pawn,
                from: Square::E7,
                to: Square::E5,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Pawn,
                from: Square::D4,
                to: Square::E5,
                capture: Some(Role::Pawn),
                promotion: None,
            },
            Move::Normal {
                role: Role::King,
                from: Square::E8,
                to: Square::E7,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Queen,
                from: Square::D1,
                to: Square::D2,
                capture: None,
                promotion: None,
            },
        ];

        let encoded = encode_game(&moves).unwrap();

        let mut decoder = TestDecoder { moves: vec![] };
        decode_move_by_move(&encoded, &mut decoder);
        assert_eq!(decoder.moves, moves);
    }

    #[test]
    fn encode_decode_consistency_pgn() {
        let moves = vec![
            Move::Normal {
                role: Role::Pawn,
                from: Square::D2,
                to: Square::D4,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Pawn,
                from: Square::E7,
                to: Square::E5,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Pawn,
                from: Square::D4,
                to: Square::E5,
                capture: Some(Role::Pawn),
                promotion: None,
            },
            Move::Normal {
                role: Role::King,
                from: Square::E8,
                to: Square::E7,
                capture: None,
                promotion: None,
            },
            Move::Normal {
                role: Role::Queen,
                from: Square::D1,
                to: Square::D2,
                capture: None,
                promotion: None,
            },
        ];

        let encoded = encode_pgn("1. d4 e5 2. dxe5 Ke7 3. Qd2").unwrap();
        let decoded = decode_game(&encoded).0;
        assert_eq!(decoded, moves);
    }
}
