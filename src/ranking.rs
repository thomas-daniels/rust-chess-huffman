use super::psqt;
use shakmaty::{Chess, Color, Move, MoveList, Piece, Position, Role, Setup, Square};

type Score = i32;

fn move_score(pos: &Chess, m: &Move) -> Score {
    let promotion = Score::from(m.promotion().unwrap_or(Role::Pawn)) - 1;
    let capture = Score::from(m.is_capture());
    let pawn_defense: Score = if any_defending_pawns(pos, m) {
        6 - Score::from(m.role())
    } else {
        6
    };
    let move_value = Score::from(512 + move_value(pos.turn(), m));
    let to = Score::from(m.to());
    let from = Score::from(m.from().expect("no drops"));

    promotion << 26 + capture << 25 + pawn_defense << 22 + move_value << 12 + to << 6 + from
}

fn any_defending_pawns(pos: &Chess, m: &Move) -> bool {
    (shakmaty::attacks::pawn_attacks(pos.turn(), m.to()) & pos.board().pawns() & pos.them()).any()
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
