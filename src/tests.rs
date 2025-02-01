use super::*;
use quickcheck_macros::quickcheck;
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
fn encode_decode_consistency_bytes() {
    let moves = short_game_moves();

    let bytes = encode_game(&moves).unwrap().to_bytes();
    let decoded = decode_game(&EncodedGame::from_bytes(&bytes)).unwrap().0;
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
    positions: Vec<Chess>,
}

impl MoveByMoveDecoder for TestDecoder {
    fn decoded_move(&mut self, mv: Move, position: &Chess) -> bool {
        self.moves.push(mv);
        self.positions.push(position.clone());

        true
    }
}

#[test]
fn encode_decode_consistency_move_by_move() {
    let moves = short_game_moves();

    let encoded = encode_game(&moves).unwrap();

    let mut decoder = TestDecoder {
        moves: vec![],
        positions: vec![],
    };
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

struct TestDecoderEarlyStop {
    moves: Vec<Move>,
}

impl MoveByMoveDecoder for TestDecoderEarlyStop {
    fn decoded_move(&mut self, mv: Move, _position: &Chess) -> bool {
        self.moves.push(mv);

        self.moves.len() < 3
    }
}

#[test]
fn decoder_early_stop() {
    let moves = short_game_moves();

    let encoded = encode_game(&moves).unwrap();

    let mut decoder = TestDecoderEarlyStop { moves: vec![] };
    decode_move_by_move(&encoded, &mut decoder).unwrap();
    assert_eq!(decoder.moves.len(), 3);
    assert_eq!(decoder.moves, moves[..3]);
}

#[test]
fn encode_move_by_move() {
    let moves = short_game_moves();

    let mut mbm = MoveByMoveEncoder::new();
    for m in &moves {
        mbm.add_move(&m).unwrap();
    }

    let decoded = decode_game(&mbm.result).unwrap().0;
    assert_eq!(decoded, moves);

    mbm.clear();
    assert_eq!(mbm.result.inner.iter().sum::<u64>(), 0);
}

#[quickcheck]
fn random_games_consistency(move_ids: Vec<u8>) -> bool {
    let mut pos = Chess::default();
    let mut moves: Vec<Move> = vec![];
    let mut positions: Vec<Chess> = vec![];
    let mut encoder = MoveByMoveEncoder::new();
    for m in move_ids {
        let legal_moves = pos.legal_moves();
        if legal_moves.is_empty() {
            break;
        }
        let i = m as usize % legal_moves.len();
        let choice = legal_moves[i].clone();
        pos.play_unchecked(&choice);
        encoder.add_move(&choice).unwrap();
        moves.push(choice);
        positions.push(pos.clone());
    }

    let bits = encoder.result;
    let bits2 = encode_game(&moves).unwrap();
    let (decoded_moves, decoded_positions) = decode_game(&bits).unwrap();
    let (decoded_moves2, decoded_positions2) =
        decode_game(&EncodedGame::from_bytes(&bits.to_bytes())).unwrap();

    let mut decoder = TestDecoder {
        moves: vec![],
        positions: vec![],
    };
    decode_move_by_move(&bits, &mut decoder).unwrap();

    bits == bits2
        && moves == decoded_moves
        && positions == decoded_positions
        && decoded_moves.len() == decoded_positions.len()
        && moves == decoder.moves
        && positions == decoder.positions
        && decoded_moves == decoded_moves2
        && decoded_positions == decoded_positions2
}
