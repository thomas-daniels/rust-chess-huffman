use super::*;
use quickcheck_macros::quickcheck;
use shakmaty::{Position, Role, Square};

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

#[test]
fn encode_decode_consistency_pgn() {
    let moves = short_game_moves();

    let encoded = encode_pgn("1. d4 e5 2. dxe5 Ke7 3. Qd2").unwrap();
    let decoded = decode_game(&encoded).unwrap().0;
    assert_eq!(decoded, moves);
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

#[test]
fn iterator_consistency() {
    let encoded = encode_pgn("1. Nf3 c5 2. g3 Nc6 3. Bg2 e5 4. O-O e4 5. Ne1 f5 
    6. d3 d5 7. dxe4 fxe4 8. Bf4 Nf6 9. e3 Be7 10. Nd2 Bg4 11. c3 Bxd1 12. Rxd1 d4 13. Nc2 d3 14. Na3 O-O 15. Nb5 a6 
    16. Bc7 axb5 17. Bxd8 Raxd8 18. Nxe4 Nxe4 19. Bxe4 c4 20. Bxc6 bxc6 21. Ra1 Ra8 22. Rfd1 c5 23. a3 b4 24. cxb4 cxb4 
    25. a4 Rac8 26. Kf1 Rfd8 27. f4 c3 28. bxc3 bxc3 29. Kf2 c2 30. Rdc1 Bf6 31. Ra2 d2 32. Raxc2 Rxc2 33. Rxc2 d1=Q 0-1").unwrap();

    let mbm1 = MoveByMoveDecoder::new(&encoded);
    let mbm2 = MoveByMoveDecoder::new(&encoded);
    let mbm3 = MoveByMoveDecoder::new(&encoded);

    let moves1 = mbm1
        .into_iter_moves()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let positions1 = mbm2
        .into_iter_positions()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let combination = mbm3
        .into_iter_moves_and_positions()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let moves2 = combination
        .iter()
        .map(|(m, _)| m.clone())
        .collect::<Vec<_>>();
    let positions2 = combination
        .iter()
        .map(|(_, p)| p.clone())
        .collect::<Vec<_>>();

    assert_eq!(moves1, moves2);
    assert_eq!(positions1, positions2);

    assert_eq!(moves1[12].to(), Square::E4);
    assert_eq!(positions1[12].board().role_at(Square::E4), Some(Role::Pawn));
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

    bits == bits2
        && moves == decoded_moves
        && positions == decoded_positions
        && decoded_moves.len() == decoded_positions.len()
        && decoded_moves == decoded_moves2
        && decoded_positions == decoded_positions2
}
