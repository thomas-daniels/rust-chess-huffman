mod codes;
mod psqt;
mod ranking;

use bit_vec::BitVec;
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
    let mut buffer = BitVec::new();
    let (book, _) = codes::code_from_lichess_weights();
    let mut pos = Chess::default();
    for m in moves {
        match ranking::move_rank(&pos, m) {
            Some(rank) => {
                book.encode(&mut buffer, &(rank as u8))?;
                pos = pos.play(m).unwrap();
            }
            None => return Err(InvalidGameError {}),
        }
    }
    Ok(buffer)
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
}
