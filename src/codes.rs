use bit_vec::BitVec;
use std::collections::BTreeMap;

/// Huffman code as used in production by lichess.org:
/// https://github.com/lichess-org/compression/blob/master/src/main/java/game/Huffman.java#L64
pub fn lichess_code() -> BTreeMap<u8, BitVec> {
    let mut map = BTreeMap::new();
    map.insert(0, BitVec::new());
    // TODO: actual code
    map
}