#![crate_name = "chess_huffman"]

mod codes;
mod pgn;
mod psqt;
mod ranking;
#[cfg(test)]
mod tests;

use bit_vec::BitVec;
use huffman_compress::Book;
use shakmaty::san::{ParseSanError, SanError};
use shakmaty::{Chess, Move, PlayError, Position};
use std::fmt;

/// Error when encoding a chess game.
#[derive(Debug)]
pub struct GameEncodeError {
    /// The underlying problem that caused the error.
    pub kind: GameEncodeErrorKind,
    /// A textual explanation for the error.
    pub explanation: String,
}

/// Kind of error when encoding a chess game.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameEncodeErrorKind {
    /// An I/O error.
    IoError,
    /// An error during the Huffman encoding process.
    HuffmanEncodeError,
    /// An illegal or ambiguous SAN (= Standard Algebraic Notation, a notation for a chess move).
    SanError,
    /// An invalid SAN so it could not be parsed.
    ParseSanError,
    /// An illegal move in the sequence of moves to be encoded.
    IllegalMove,
}

impl std::error::Error for GameEncodeError {}

impl fmt::Display for GameEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to encode chess game: {}", &self.explanation)
    }
}

#[derive(Debug)]
pub struct GameDecodeError {}

/// Error when decoding an encoded bit vector into a game, because the bit vector is invalid.
impl std::error::Error for GameDecodeError {}

impl fmt::Display for GameDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot decode invalid bit vector")
    }
}

impl From<huffman_compress::EncodeError> for GameEncodeError {
    fn from(inner: huffman_compress::EncodeError) -> Self {
        Self {
            kind: GameEncodeErrorKind::HuffmanEncodeError,
            explanation: format!("Error during Huffman encoding: {}", inner),
        }
    }
}

impl From<std::io::Error> for GameEncodeError {
    fn from(inner: std::io::Error) -> Self {
        Self {
            kind: GameEncodeErrorKind::IoError,
            explanation: format!("I/O Error: {}", inner),
        }
    }
}

impl From<SanError> for GameEncodeError {
    fn from(inner: SanError) -> Self {
        Self {
            kind: GameEncodeErrorKind::SanError,
            explanation: format!("Illegal or ambiguous SAN: {}", inner),
        }
    }
}

impl From<ParseSanError> for GameEncodeError {
    fn from(inner: ParseSanError) -> Self {
        Self {
            kind: GameEncodeErrorKind::SanError,
            explanation: format!("Unable to parse SAN: {}", inner),
        }
    }
}

impl From<PlayError<Chess>> for GameDecodeError {
    fn from(_: PlayError<Chess>) -> Self {
        Self {}
    }
}

/// Encodes a chess game into a compressed bit vector.
///
/// # Arguments
///
/// * `moves` - The sequence of moves that the game consists of.
///
/// # Errors
///
/// Will return `Err` if the given sequence of moves is invalid.
///
/// # Examples
///
/// ```
/// use shakmaty::{Move, Role, Square};
/// use chess_huffman::encode_game;
///
/// # use bit_vec::BitVec;
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), GameEncodeError> {
/// let moves = vec![
///     Move::Normal {
///         role: Role::Pawn,
///         from: Square::D2,
///         to: Square::D4,
///         capture: None,
///         promotion: None,
///     },
///     Move::Normal {
///         role: Role::Pawn,
///         from: Square::E7,
///         to: Square::E5,
///         capture: None,
///         promotion: None,
///     }
/// ];
///
/// let encoded = encode_game(&moves)?;
/// # Ok(())
/// # }
/// ```
pub fn encode_game(moves: &[Move]) -> Result<BitVec, GameEncodeError> {
    let mut encoder = MoveByMoveEncoder::new();
    for m in moves {
        encoder.add_move(m)?;
    }
    Ok(encoder.buffer)
}

/// Encodes a chess game, represented as a PGN (Portable Game Notation), into a compressed bit vector.
///
/// # Arguments
///
/// * `pgn` - The PGN string of the game.
///
/// # Errors
///
/// Will return `Err` if the PGN is invalid.
///
/// # Examples
///
/// ```
/// # use chess_huffman::encode_pgn;
/// # use bit_vec::BitVec;
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), GameEncodeError> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5").unwrap();
/// # Ok(())
/// # }
/// ```
pub fn encode_pgn<T: AsRef<str>>(pgn: T) -> Result<BitVec, GameEncodeError> {
    let mut reader = pgn_reader::BufferedReader::new_cursor(pgn.as_ref());

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(BitVec::new()))?;
    Ok(bits)
}

/// Encodes a chess game, stored in a PGN file, into a compressed bit vector.
///
/// # Arguments
///
/// * `path` - The path to the PGN file.
///
/// # Errors
///
/// Will return `Err` if the file could not be read or if the PGN is invalid.
pub fn encode_pgn_file<P: AsRef<std::path::Path>>(path: P) -> Result<BitVec, GameEncodeError> {
    let file = std::fs::File::open(path)?;
    let mut reader = pgn_reader::BufferedReader::new(file);

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(BitVec::new()))?;
    Ok(bits)
}

pub fn decode_game(bits: &BitVec) -> Result<(Vec<Move>, Chess), GameDecodeError> {
    let (_, tree) = codes::code_from_lichess_weights();
    let ranks = tree.decoder(bits, 256);
    let mut moves = vec![];
    let mut pos = Chess::default();
    for rank in ranks {
        let m = ranking::from_position(&pos).remove(rank as usize);
        pos = pos.play(&m)?;
        moves.push(m);
    }
    Ok((moves, pos))
}

pub fn decode_move_by_move<T: MoveByMoveDecoder>(
    bits: &BitVec,
    decoder: &mut T,
) -> Result<(), GameDecodeError> {
    let (_, tree) = codes::code_from_lichess_weights();
    let ranks = tree.decoder(bits, 256);
    let mut pos = Chess::default();
    for rank in ranks {
        let m = ranking::from_position(&pos).remove(rank as usize);
        pos = pos.play(&m)?;
        decoder.decoded_move(m, &pos);
    }
    Ok(())
}

/// An encoder that lets you add moves one-by-one, rather than a whole game at once.
///
/// # Examples
///
/// ```
/// # use chess_huffman::MoveByMoveEncoder;
/// use shakmaty::Move;
///
/// # use bit_vec::BitVec;
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), GameEncodeError> {
/// let moves: Vec<Move> = vec![ /* ... */ ];
/// let mut encoder = MoveByMoveEncoder::new();
/// for m in moves {
///    encoder.add_move(&m)?;
/// }
/// # Ok(())
/// # }
/// ```
pub struct MoveByMoveEncoder {
    book: Book<u8>,
    pos: Chess,
    /// The resulting bit vector.
    pub buffer: BitVec,
}

impl MoveByMoveEncoder {
    /// Constructs a new `MoveByMoveEncoder`.
    #[must_use]
    pub fn new() -> Self {
        let (book, _) = codes::code_from_lichess_weights();
        Self {
            book,
            pos: Chess::default(),
            buffer: BitVec::new(),
        }
    }

    /// Adds a new move to the encoder. This affects the value of `buffer`.
    /// # Examples
    ///
    /// ```
    /// # use chess_huffman::MoveByMoveEncoder;
    /// use shakmaty::Move;
    ///
    /// # use bit_vec::BitVec;
    /// # use chess_huffman::GameEncodeError;
    /// # fn try_main() -> Result<(), GameEncodeError> {
    /// let moves: Vec<Move> = vec![ /* ... */ ];
    /// let mut encoder = MoveByMoveEncoder::new();
    /// for m in moves {
    ///    encoder.add_move(&m)?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_move(&mut self, m: &Move) -> Result<(), GameEncodeError> {
        match ranking::move_rank(&self.pos, m) {
            Some(rank) => {
                if rank > 255 {
                    return Err(GameEncodeError {
                        kind: GameEncodeErrorKind::HuffmanEncodeError,
                        explanation: String::from(
                            "Too many possible valid moves - is this a valid chess position?",
                        ),
                    });
                }
                #[allow(clippy::cast_possible_truncation)]
                self.book.encode(&mut self.buffer, &(rank as u8))?;
                self.pos.play_unchecked(m);
            }
            None => {
                return Err(GameEncodeError {
                    kind: GameEncodeErrorKind::IllegalMove,
                    explanation: format!("Illegal move {}", m),
                })
            }
        }

        Ok(())
    }

    /// Clears the encoder: restores the game state to the start position and clears `buffer`.
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
