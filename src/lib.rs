#![crate_name = "chess_huffman"]

mod codes;
mod pgn;
mod psqt;
mod ranking;
#[cfg(test)]
mod tests;

use bit_vec::BitVec;
use huffman_compress2::Book;
use shakmaty::san::{ParseSanError, SanError};
use shakmaty::{Chess, Move, PlayError, Position};
use std::fmt;

/// The result of an encoding operation.
pub type EncodeResult<T> = Result<T, GameEncodeError>;
/// The result of a decoding operation.
pub type DecodeResult<T> = Result<T, GameDecodeError>;

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

/// Error when decoding an encoded bit vector into a game, because the bit vector is invalid.
#[derive(Debug)]
pub struct GameDecodeError {}

impl std::error::Error for GameDecodeError {}

impl fmt::Display for GameDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot decode invalid bit vector")
    }
}

impl From<huffman_compress2::EncodeError> for GameEncodeError {
    fn from(inner: huffman_compress2::EncodeError) -> Self {
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
/// [`GameEncodeError`] if the given sequence of moves is invalid.
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
pub fn encode_game(moves: &[Move]) -> EncodeResult<BitVec> {
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
/// * `pgn` - The PGN of the game.
///
/// # Errors
///
/// [`GameEncodeError`] if the PGN is invalid.
///
/// # Examples
///
/// ```
/// # use chess_huffman::encode_pgn;
/// # use bit_vec::BitVec;
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), GameEncodeError> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5")?;
/// let encoded = encode_pgn(b"1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5")?;
/// let encoded = encode_pgn(String::from("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5"))?;
/// # Ok(())
/// # }
/// ```
pub fn encode_pgn<T: AsRef<[u8]>>(pgn: T) -> EncodeResult<BitVec> {
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
/// [`GameEncodeError`] if the file could not be read or if the PGN is invalid,
/// its `kind` attribute tells more about the error reason.
pub fn encode_pgn_file<P: AsRef<std::path::Path>>(path: P) -> EncodeResult<BitVec> {
    let file = std::fs::File::open(path)?;
    let mut reader = pgn_reader::BufferedReader::new(file);

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(BitVec::new()))?;
    Ok(bits)
}

/// Decodes a bit vector into a game, returning both a vector of all moves
/// and all positions. The N'th position in the position vector is the
/// position after the N'th move in the move vector.
///
/// # Arguments
///
/// * `bits` - A bit vector of a compressed chess game.
///
/// # Errors
///
/// [`GameDecodeError`] if the game contains invalid moves.
///
/// # Examples
///
/// ```
/// # use chess_huffman::{encode_pgn, decode_game};
/// # use bit_vec::BitVec;
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5")?;
/// let decoded = decode_game(&encoded)?;
/// # Ok(())
/// # }
pub fn decode_game(bits: &BitVec) -> DecodeResult<(Vec<Move>, Vec<Chess>)> {
    let (_, tree) = &*codes::CODE_FROM_LICHESS_WEIGHTS;
    let ranks = tree.unbounded_decoder(bits);
    let mut moves = vec![];
    let mut pos = Chess::default();
    let mut positions = vec![];
    for rank in ranks {
        let valid_moves = ranking::from_position(&pos);
        let m = valid_moves.get(rank as usize).ok_or(GameDecodeError {})?;
        pos.play_unchecked(m);
        moves.push(m.clone());
        positions.push(pos.clone());
    }
    Ok((moves, positions))
}

/// Decodes a bit vector into a game, calling an implementation of
/// the [`MoveByMoveDecoder`] trait for each move. This allows for more
/// fine-grained processing than [`decode_game`].
///
/// # Arguments
///
/// * `bits` - A bit vector of a compressed chess game.
/// * `decoder` - A decoder implementing [`MoveByMoveDecoder`].
///
/// # Errors
///
/// [`GameDecodeError`] if the game contains invalid moves.
///
/// # Examples
///
/// ```
/// # use chess_huffman::{encode_pgn, decode_move_by_move};
/// # use bit_vec::BitVec;
/// # use chess_huffman::{GameEncodeError, MoveByMoveDecoder};
/// use shakmaty::{Chess, Move};
///
/// struct ExampleDecoder {
///     capture_count: u8
/// }
///
/// impl MoveByMoveDecoder for ExampleDecoder {
///     fn decoded_move(&mut self, mv: &Move, _position: &Chess) -> bool {
///         if (mv.is_capture()) {
///             self.capture_count += 1;
///         }
///
///         true
///     }
/// }
///
/// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. exd5")?;
/// let mut decoder = ExampleDecoder { capture_count: 0 };
/// decode_move_by_move(&encoded, &mut decoder)?;
/// assert_eq!(decoder.capture_count, 1);
/// # Ok(())
/// # }
pub fn decode_move_by_move<T: MoveByMoveDecoder>(
    bits: &BitVec,
    decoder: &mut T,
) -> DecodeResult<()> {
    let (_, tree) = &*codes::CODE_FROM_LICHESS_WEIGHTS;
    let ranks = tree.unbounded_decoder(bits);
    let mut pos = Chess::default();
    for rank in ranks {
        let valid_moves = ranking::from_position(&pos);
        let m = valid_moves.get(rank as usize).ok_or(GameDecodeError {})?;
        pos.play_unchecked(m);
        let cont = decoder.decoded_move(m, &pos);
        if !cont {
            break;
        }
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
pub struct MoveByMoveEncoder<'a> {
    book: &'a Book<u8>,
    /// The current chess position.
    pub pos: Chess,
    /// The resulting bit vector.
    pub buffer: BitVec,
}

impl MoveByMoveEncoder<'_> {
    /// Constructs a new [`MoveByMoveEncoder`].
    #[must_use]
    pub fn new() -> Self {
        let (book, _) = &*codes::CODE_FROM_LICHESS_WEIGHTS;
        Self {
            book,
            pos: Chess::default(),
            buffer: BitVec::new(),
        }
    }

    /// Adds a new move to the encoder. This affects the value of `buffer`.
    ///
    /// # Errors
    ///
    /// [`GameEncodeError`] if the move is invalid or `pos` is an invalid chess position.
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
    pub fn add_move(&mut self, m: &Move) -> EncodeResult<()> {
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

    /// Clears the encoder: restores the game state to the start position and empties `buffer`.
    pub fn clear(&mut self) {
        self.pos = Chess::default();
        self.buffer.truncate(0);
    }
}

impl Default for MoveByMoveEncoder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait for decoding games move-by-move.
///
/// # Examples
///
/// ```
/// # use chess_huffman::{encode_pgn, decode_move_by_move};
/// # use bit_vec::BitVec;
/// # use chess_huffman::{GameEncodeError, MoveByMoveDecoder};
/// use shakmaty::{Chess, Move};
///
/// struct ExampleDecoder {
///     capture_count: u8
/// }
///
/// impl MoveByMoveDecoder for ExampleDecoder {
///     fn decoded_move(&mut self, mv: &Move, _position: &Chess) -> bool {
///         if (mv.is_capture()) {
///             self.capture_count += 1;
///         }
///
///         true
///     }
/// }
///
/// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. exd5")?;
/// let mut decoder = ExampleDecoder { capture_count: 0 };
/// decode_move_by_move(&encoded, &mut decoder)?;
/// assert_eq!(decoder.capture_count, 1);
/// # Ok(())
/// # }
pub trait MoveByMoveDecoder {
    /// Called when a move is decoded.
    ///
    /// The returned boolean signals whether decoding the rest of the game should continue.
    /// Return `false` to stop the decoding process early.
    ///
    /// # Arguments
    ///
    /// * `mv` - The decoded move.
    /// * `position` - The chess position after the decoded move has been played.
    fn decoded_move(&mut self, mv: &Move, position: &Chess) -> bool;
}
