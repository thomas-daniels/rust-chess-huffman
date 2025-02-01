use chess_huffman::{decode_game, encode_pgn, EncodedGame};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shakmaty::Square;

static PGN: &'static str = "1. e4 c5 2. c3 d5 3. exd5 Nf6 4. Bb5+ Bd7 5. Bxd7+ Qxd7 
    6. d4 cxd4 7. Qxd4 Qxd5 8. Nf3 Nc6 9. Qxd5 Nxd5 10. O-O e5 11. Re1 f6 
    12. Nbd2 Kf7 13. Nb3 Be7 14. Nfd2 Rhd8 15. Ne4 b6 16. g3 Rac8 17. a4 h6 
    18. a5 f5 19. Ned2 b5 20. Nf3 Bf6 21. a6 e4 22. Nfd2 b4 23. c4 Nb6 24. f3 Ne5 
    25. c5 Nbd7 26. fxe4 fxe4 27. Rf1 Nxc5 28. Nxc5 Rxc5 29. Nxe4 Rc2 30. Bxh6 Kg6 
    31. Be3 Ng4 32. Bxa7 Bxb2 33. Rad1 Re8 34. Rf4 Nf6 35. Nxf6 Bxf6 36. Bf2 Ra2 
    37. Rxb4 Rxa6 38. Rg4+ Kf7 39. Rf4 Rae6 40. Rf1 R8e7 41. Bd4 Kg6 42. Bxf6 Rxf6 
    43. Rxf6+ gxf6 44. Rf4 Kf7 45. Kg2 Re5 46. h4 Re2+ 47. Kf3 Re5 48. Rg4 Rf5+ 
    49. Rf4 Re5 50. Kg4 Kg6 51. Kh3 f5 52. Rf3 Re4 53. Kg2 Kf6 54. Rd3 f4 
    55. g4 Re1 56. Rd8 Re3 57. Kf2 Rg3 58. Rg8 Ke5 59. Re8+ Kd4 60. Rd8+ Ke4 
    61. Rg8 Kd4 62. g5 Ke4 63. g6 Rf3+ 64. Kg2 Re3 65. Kh2 Kf5 66. h5 Kg4 
    67. Rf8 Re2+ 68. Kg1 Re3 69. Kf1 Kf3 70. Kg1 Re2 71. Kf1 Rf2+ 72. Ke1 Re2+ 
    73. Kd1 Rg2 74. Kc1 Rf2 75. Kb1 Rf1+ 76. Kb2 Rf2+ 77. Kb3 Re2 0-1";

fn bench_encode_pgn(c: &mut Criterion) {
    let pgn = black_box(PGN);

    c.bench_function("encode-pgn", |b| {
        b.iter(|| {
            let encoded = encode_pgn(pgn).unwrap();

            assert!(encoded.inner.len() > 154);
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let bits = encode_pgn(black_box(PGN)).unwrap();

    c.bench_function("decode", |b| {
        b.iter(|| {
            let (moves, positions) = decode_game(&bits).unwrap();

            assert_eq!(moves.len(), positions.len());
            assert_eq!(moves.last().unwrap().to(), Square::E2);
        })
    });
}

fn bench_encode_pgn_bytes(c: &mut Criterion) {
    let pgn = black_box(PGN);

    c.bench_function("encode-pgn-bytes", |b| {
        b.iter(|| {
            let encoded = encode_pgn(pgn).unwrap().to_bytes();

            assert!(encoded.len() > 19);
        })
    });
}

fn bench_decode_bytes(c: &mut Criterion) {
    let bytes = encode_pgn(black_box(PGN)).unwrap().to_bytes();

    c.bench_function("decode-bytes", |b| {
        b.iter(|| {
            let encoded = EncodedGame::from_bytes(&bytes);
            let (moves, positions) = decode_game(&encoded).unwrap();

            assert_eq!(moves.len(), positions.len());
            assert_eq!(moves.last().unwrap().to(), Square::E2);
        })
    });
}

criterion_group!(
    benches,
    bench_encode_pgn,
    bench_decode,
    bench_encode_pgn_bytes,
    bench_decode_bytes
);

criterion_main!(benches);
