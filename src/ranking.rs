use super::psqt;
use shakmaty::{Chess, Color, Move, Piece, Position, Role, Square};

type Score = i32;

pub fn move_rank(pos: &Chess, m: &Move) -> Option<usize> {
    from_position(pos).iter().position(|x| x == m)
}

pub fn from_position(pos: &Chess) -> Vec<Move> {
    let mut legals = pos.legal_moves();
    legals.sort_unstable_by_key(|m| -move_score(pos, m));
    legals.to_vec()
}

fn move_score(pos: &Chess, m: &Move) -> Score {
    let promotion = Score::from(m.promotion().unwrap_or(Role::Pawn)) - 1;
    let capture = Score::from(m.is_capture());
    let pawn_defense: Score = if any_defending_pawns(pos, m.to()) {
        6 - Score::from(m.role())
    } else {
        6
    };
    let move_value = Score::from(512 + move_value(pos.turn(), m));
    let to = Score::from(m.to());
    let from = Score::from(m.from().expect("no drops"));

    (promotion << 26)
        + (capture << 25)
        + (pawn_defense << 22)
        + (move_value << 12)
        + (to << 6)
        + from
}

fn any_defending_pawns(pos: &Chess, to: Square) -> bool {
    (shakmaty::attacks::pawn_attacks(pos.turn(), to) & pos.board().pawns() & pos.them()).any()
}

// https://github.com/niklasf/rust-pgn-reader/blob/compression-with-spsa/examples/compression.rs#L121
// based on above, but piece roles start from 1 now
fn piece_value(piece: Piece, square: Square) -> i16 {
    let sq = if piece.color.is_white() {
        square.flip_vertical()
    } else {
        square
    };
    psqt::LICHESS[piece.role as usize - 1][usize::from(sq)]
}

// https://github.com/niklasf/rust-pgn-reader/blob/compression-with-spsa/examples/compression.rs#L126
fn move_value(turn: Color, m: &Move) -> i16 {
    let role = m.role();
    piece_value(role.of(turn), m.to()) - piece_value(role.of(turn), m.from().expect("no drops"))
}

#[cfg(test)]
mod tests {
    // These tests are not very extensive at this point.
    // They're mostly there to tell me if I'm badly breaking something.
    use super::*;

    #[test]
    fn test_piece_value() {
        assert_eq!(
            piece_value(
                Piece {
                    color: Color::White,
                    role: Role::Knight
                },
                Square::E3
            ),
            15
        );
        assert_eq!(
            piece_value(
                Piece {
                    color: Color::White,
                    role: Role::Knight
                },
                Square::D5
            ),
            20
        );
    }

    #[test]
    fn test_move_value() {
        assert_eq!(
            move_value(
                Color::White,
                &Move::Normal {
                    role: Role::Knight,
                    from: Square::E3,
                    to: Square::D5,
                    capture: None,
                    promotion: None
                }
            ),
            5
        );
    }

    #[test]
    fn test_any_defending_pawns() {
        assert!(any_defending_pawns(&Chess::default(), Square::E6));
    }

    #[test]
    fn test_move_score() {
        assert_eq!(
            move_score(
                &Chess::default(),
                &Move::Normal {
                    role: Role::Pawn,
                    from: Square::E2,
                    to: Square::E4,
                    capture: None,
                    promotion: None,
                }
            ),
            (6 << 22) + (564 << 12) + (28 << 6) + 12
        );
    }

    #[test]
    fn test_e4_highest_score() {
        let from_starting_position = from_position(&Chess::default());
        let highest_choice = &from_starting_position[0];
        assert_eq!(
            highest_choice,
            &Move::Normal {
                role: Role::Pawn,
                from: Square::E2,
                to: Square::E4,
                capture: None,
                promotion: None,
            }
        );

        assert_eq!(
            move_rank(
                &Chess::default(),
                &Move::Normal {
                    role: Role::Pawn,
                    from: Square::E2,
                    to: Square::E4,
                    capture: None,
                    promotion: None,
                }
            ),
            Some(0)
        );
    }
}
