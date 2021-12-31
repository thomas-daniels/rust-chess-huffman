use crate::{GameEncodeError, MoveByMoveEncoder};
use bit_vec::BitVec;
use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::san::San;

pub struct Encoder {
    mbm: MoveByMoveEncoder,
    error: Option<GameEncodeError>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            mbm: MoveByMoveEncoder::new(),
            error: None,
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn san_may_error(&mut self, san_plus: SanPlus) -> Result<(), GameEncodeError> {
        let m = san_plus
            .san
            .to_string()
            .parse::<San>()?
            .to_move(&self.mbm.pos)?;
        self.mbm.add_move(&m)?;
        Ok(())
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for Encoder {
    type Result = std::result::Result<BitVec, GameEncodeError>;

    fn begin_game(&mut self) {
        self.error = None;
        self.mbm.clear();
    }

    #[allow(clippy::needless_pass_by_value)]
    fn san(&mut self, san_plus: SanPlus) {
        match self.san_may_error(san_plus) {
            Ok(()) => {}
            Err(e) => self.error = Some(e),
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        if self.error.is_none() {
            Ok(self.mbm.buffer.clone())
        } else {
            let mut swap = None;
            std::mem::swap(&mut swap, &mut self.error);
            Err(swap.unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode_game;
    use pgn_reader::BufferedReader;
    use shakmaty::{Move, Role, Square};

    #[test]
    fn short_game() {
        let pgn = b"1. d4 e5 2. dxe5 Ke7 3. Qd2";

        let mut reader = BufferedReader::new_cursor(&pgn[..]);

        let mut encoder = Encoder::new();
        let bits = reader.read_game(&mut encoder).unwrap().unwrap().unwrap();

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

        assert_eq!(decode_game(&bits).unwrap().0, moves);
    }

    #[test]
    fn error() {
        let pgn = b"1. d4 e5 2. dxe5 Ke7 3. Qc2";

        let mut reader = BufferedReader::new_cursor(&pgn[..]);

        let mut encoder = Encoder::new();
        assert!(reader.read_game(&mut encoder).unwrap().unwrap().is_err());
    }
}
