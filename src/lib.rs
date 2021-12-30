mod codes;
pub mod pgn;
mod psqt;
mod ranking;

use bit_vec::BitVec;
use huffman_compress::Book;
use shakmaty::{Chess, Move, Position};
use std::fmt;

#[derive(Debug)]
pub struct InvalidGameError {}

impl std::error::Error for InvalidGameError {}

impl fmt::Display for InvalidGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The provided game contains invalid moves.")
    }
}

impl std::convert::From<huffman_compress::EncodeError> for InvalidGameError {
    fn from(_: huffman_compress::EncodeError) -> Self {
        Self {}
    }
}

pub fn encode_game(moves: &[Move]) -> Result<BitVec, InvalidGameError> {
    let mut encoder = MoveByMoveEncoder::new();
    for m in moves {
        encoder.add_move(&m)?;
    }
    Ok(encoder.buffer)
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

    pub fn add_move(&mut self, m: &Move) -> Result<(), InvalidGameError> {
        match ranking::move_rank(&self.pos, m) {
            Some(rank) => {
                self.book.encode(&mut self.buffer, &(rank as u8))?;
                self.pos.play_unchecked(m);
            }
            None => return Err(InvalidGameError {}),
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
}
