#![crate_name = "chess_huffman"]

mod codes;
mod pgn;
mod psqt;
mod ranking;
#[cfg(test)]
mod tests;

use bitm::BitAccess;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codes::Book;
use minimum_redundancy::{Decoder, DecodingResult};
use shakmaty::san::{ParseSanError, SanError};
use shakmaty::{Chess, Move, PlayError, Position};
use std::fmt;
use std::io::Cursor;

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

impl From<std::io::Error> for GameEncodeError {
    fn from(inner: std::io::Error) -> Self {
        Self {
            kind: GameEncodeErrorKind::IoError,
            explanation: format!("I/O Error: {inner}"),
        }
    }
}

impl From<SanError> for GameEncodeError {
    fn from(inner: SanError) -> Self {
        Self {
            kind: GameEncodeErrorKind::SanError,
            explanation: format!("Illegal or ambiguous SAN: {inner}"),
        }
    }
}

impl From<ParseSanError> for GameEncodeError {
    fn from(inner: ParseSanError) -> Self {
        Self {
            kind: GameEncodeErrorKind::SanError,
            explanation: format!("Unable to parse SAN: {inner}"),
        }
    }
}

impl From<PlayError<Chess>> for GameDecodeError {
    fn from(_: PlayError<Chess>) -> Self {
        Self {}
    }
}

/// Representation of an encoded chess game.
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct EncodedGame {
    pub inner: Vec<u64>,
    pub bit_index: usize,
}

impl EncodedGame {
    /// Convert the encoded chess game to a byte vector. Use `from_bytes` to convert the result back to an `EncodedGame`.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let byte_count = if self.bit_index % 8 == 0 {
            self.bit_index / 8
        } else {
            self.bit_index / 8 + 1
        };
        #[allow(clippy::cast_possible_truncation)]
        let m = (self.bit_index % 64) as u8;
        let padding = if m == 0 { 0 } else { 64 - m };

        let mut wrt = Vec::with_capacity(self.inner.len() * 8 + 1);
        for x in &self.inner {
            wrt.write_u64::<LittleEndian>(*x).unwrap();
        }
        wrt.truncate(byte_count);
        wrt.push(padding);

        wrt
    }

    /// Convert a byte vector (that was the output of `to_bytes`) to an `EncodedGame`.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let total_len_minus_one = bytes.len() - 1;
        let padding = bytes[total_len_minus_one] as usize;
        let padding_bytes = padding / 8;
        let bit_index = total_len_minus_one * 8 - (padding % 8);
        let content_slice = &bytes[0..total_len_minus_one];
        let full_slice = if padding_bytes != 0 {
            &[content_slice, &vec![0; padding_bytes]].concat()
        } else {
            content_slice
        };
        let mut rdr = Cursor::new(full_slice);
        let mut buffer = vec![];
        while let Ok(x) = rdr.read_u64::<LittleEndian>() {
            buffer.push(x);
        }
        EncodedGame {
            inner: buffer,
            bit_index,
        }
    }

    fn new() -> Self {
        Self {
            inner: vec![0; 256 / 64],
            bit_index: 0,
        }
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
pub fn encode_game(moves: &[Move]) -> EncodeResult<EncodedGame> {
    let mut encoder = MoveByMoveEncoder::new();
    for m in moves {
        encoder.add_move(m)?;
    }
    Ok(encoder.result)
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
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), GameEncodeError> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5")?;
/// let encoded = encode_pgn(b"1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5")?;
/// let encoded = encode_pgn(String::from("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5"))?;
/// # Ok(())
/// # }
/// ```
pub fn encode_pgn<T: AsRef<[u8]>>(pgn: T) -> EncodeResult<EncodedGame> {
    let mut reader = pgn_reader::BufferedReader::new_cursor(pgn.as_ref());

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(EncodedGame::new()))?;
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
pub fn encode_pgn_file<P: AsRef<std::path::Path>>(path: P) -> EncodeResult<EncodedGame> {
    let file = std::fs::File::open(path)?;
    let mut reader = pgn_reader::BufferedReader::new(file);

    let mut encoder = pgn::Encoder::new();
    let bits = reader
        .read_game(&mut encoder)?
        .unwrap_or_else(|| Ok(EncodedGame::new()))?;
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
/// # use chess_huffman::GameEncodeError;
/// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. e5")?;
/// let decoded = decode_game(&encoded)?;
/// # Ok(())
/// # }
pub fn decode_game(encoded: &EncodedGame) -> DecodeResult<(Vec<Move>, Vec<Chess>)> {
    let mut moves = vec![];
    let mut positions = vec![];

    let decoder = MoveByMoveDecoder::new(encoded);
    for d in decoder {
        let (m, pos) = d?;
        moves.push(m);
        positions.push(pos);
    }

    Ok((moves, positions))
}

/// Iterator to decode a game move by move, rather than all at once.
/// This allows for more fine-grained processing than [`decode_game`].
///
/// # Examples
///
/// ```
/// # use chess_huffman::{encode_pgn, MoveByMoveDecoder};
/// # use shakmaty::Position;
///
/// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let encoded = encode_pgn("1. e4 c5 2. Nf3 e6 3. c3 d5 4. exd5")?;
/// let mut capture_count = 0;
/// let mut decoder = MoveByMoveDecoder::new(&encoded);
/// for d in decoder {
///     let (mv, _) = d?;
///     if mv.is_capture() {
///         capture_count += 1;
///     }
/// }
/// assert_eq!(capture_count, 1);
/// # Ok(())
/// # }
pub struct MoveByMoveDecoder<'a> {
    bit_iter: bitm::BitIterator<'a>,
    huff_decoder: Decoder<'a, u8>,
    pos: Chess,
}

impl<'a> MoveByMoveDecoder<'a> {
    /// Construct a new [`MoveByMoveDecoder`] from an [`EncodedGame`].
    #[must_use]
    pub fn new(encoded: &'a EncodedGame) -> Self {
        let huff_decoder = codes::get_decoder();
        let bit_iter = encoded.inner.bit_in_range_iter(0..encoded.bit_index);
        Self {
            bit_iter,
            huff_decoder,
            pos: Chess::default(),
        }
    }
}

impl<'a> Iterator for MoveByMoveDecoder<'a> {
    type Item = DecodeResult<(Move, Chess)>;

    /// Returns the next move and the resulting position when the move is played.
    fn next(&mut self) -> Option<Self::Item> {
        match self.huff_decoder.decode_next(&mut self.bit_iter) {
            DecodingResult::Value(rank) => {
                let m =
                    ranking::nth_from_position(*rank as usize, &self.pos).ok_or(GameDecodeError {});
                match m {
                    Ok(m) => {
                        self.pos.play_unchecked(&m);

                        Some(Ok((m, self.pos.clone())))
                    }
                    Err(e) => Some(Err(e)),
                }
            }
            DecodingResult::Invalid => {
                Some(Err(GameDecodeError {}))
                // this shouldn't happen though: according to minimum_redundancy's docs, Invalid can only be returned if the bits per fragment > 1
            }
            DecodingResult::Incomplete => {
                if self.huff_decoder.consumed_fragments() == 0 {
                    None
                } else {
                    Some(Err(GameDecodeError {}))
                }
            }
        }
    }
}

/// An encoder that lets you add moves one-by-one, rather than a whole game at once.
///
/// # Examples
///
/// ```
/// # use chess_huffman::MoveByMoveEncoder;
/// use shakmaty::Move;
///
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
    book: &'a Book,
    /// The current chess position.
    pub pos: Chess,
    /// The resulting encoded game.
    pub result: EncodedGame,
}

impl MoveByMoveEncoder<'_> {
    /// Constructs a new [`MoveByMoveEncoder`].
    #[must_use]
    pub fn new() -> Self {
        let book = &*codes::BOOK_FROM_LICHESS_WEIGHTS;
        Self {
            book,
            pos: Chess::default(),
            result: EncodedGame::new(),
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
                self.book.encode(&mut self.result, rank as u8);
                self.pos.play_unchecked(m);
            }
            None => {
                return Err(GameEncodeError {
                    kind: GameEncodeErrorKind::IllegalMove,
                    explanation: format!("Illegal move {m}"),
                })
            }
        }

        Ok(())
    }

    /// Clears the encoder: restores the game state to the start position and empties `buffer`.
    pub fn clear(&mut self) {
        self.pos = Chess::default();
        self.result = EncodedGame::new();
    }
}

impl Default for MoveByMoveEncoder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
