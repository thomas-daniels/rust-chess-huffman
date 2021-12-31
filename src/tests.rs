use super::*;
use shakmaty::{Role, Square};

fn short_game_moves() -> Vec<Move> {
    vec![
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
    ]
}

#[test]
fn encode_decode_consistency() {
    let moves = short_game_moves();

    let encoded = encode_game(&moves).unwrap();
    let decoded = decode_game(&encoded).unwrap().0;
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
    let moves = short_game_moves();

    let encoded = encode_game(&moves).unwrap();

    let mut decoder = TestDecoder { moves: vec![] };
    decode_move_by_move(&encoded, &mut decoder).unwrap();
    assert_eq!(decoder.moves, moves);
}

#[test]
fn encode_decode_consistency_pgn() {
    let moves = short_game_moves();

    let encoded = encode_pgn("1. d4 e5 2. dxe5 Ke7 3. Qd2").unwrap();
    let decoded = decode_game(&encoded).unwrap().0;
    assert_eq!(decoded, moves);
}
