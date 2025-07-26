use crate::{EncodedGame, GameEncodeError, MoveByMoveEncoder};
use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::san::San;
use std::ops::ControlFlow;

pub struct Encoder<'a> {
    mbm: MoveByMoveEncoder<'a>,
}

impl Encoder<'_> {
    pub fn new() -> Self {
        Self {
            mbm: MoveByMoveEncoder::new(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn san_may_error(&mut self, san_plus: SanPlus) -> Result<(), GameEncodeError> {
        let m = san_plus
            .san
            .to_string()
            .parse::<San>()?
            .to_move(&self.mbm.pos)?;
        self.mbm.add_move(m)?;
        Ok(())
    }
}

impl Default for Encoder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for Encoder<'_> {
    type Tags = ();
    type Movetext = Option<GameEncodeError>;
    type Output = std::result::Result<EncodedGame, GameEncodeError>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(())
    }

    fn begin_movetext(&mut self, _tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        self.mbm.clear();
        ControlFlow::Continue(None)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn san(
        &mut self,
        _movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        match self.san_may_error(san_plus) {
            Ok(()) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }

    fn begin_variation(
        &mut self,
        _movetext: &mut Self::Movetext,
    ) -> ControlFlow<Self::Output, Skip> {
        ControlFlow::Continue(Skip(true))
    }

    fn end_game(&mut self, _movetext: Self::Movetext) -> Self::Output {
        Ok(self.mbm.result.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode_game;
    use pgn_reader::Reader;
    use shakmaty::{Move, Role, Square};
    use std::io::Cursor;

    #[test]
    fn short_game() {
        let pgn = b"1. d4 e5 2. dxe5 Ke7 3. Qd2";

        let mut reader = Reader::new(Cursor::new(pgn));

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

        let mut reader = Reader::new(Cursor::new(pgn));

        let mut encoder = Encoder::new();
        assert!(reader.read_game(&mut encoder).unwrap().unwrap().is_err());
    }

    #[test]
    fn valid_games_after_error() {
        let pgn = b"
        [Event \"Invalid\"]
        1. d4 e5 2. dxe5 Ke7 3. Qc2

        [Event \"Valid 1\"]
        1. d4 e5 2. dxe5 Ke7 3. Qd2

        [Event \"Valid 2\"]
        1. b4 e5 2. e4 Nf6 3. d3
        ";

        let mut reader = Reader::new(Cursor::new(pgn));
        let mut encoder = Encoder::new();
        let mut results = reader.read_games(&mut encoder);
        assert!(results.next().unwrap().unwrap().is_err());

        let valid1 = results.next().unwrap().unwrap().unwrap();
        let valid2 = results.next().unwrap().unwrap().unwrap();

        assert_ne!(valid1.to_bytes(), valid2.to_bytes());

        let decoded1 = decode_game(&valid1).unwrap().0;
        let decoded2 = decode_game(&valid2).unwrap().0;

        assert_eq!(decoded1.len(), 5);
        assert_eq!(decoded2.len(), 5);
        assert_eq!(decoded1[0].to(), Square::D4);
        assert_eq!(decoded2[0].to(), Square::B4);
    }
}
