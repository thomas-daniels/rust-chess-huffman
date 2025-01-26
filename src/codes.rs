use bit_vec::BitVec;
use minimum_redundancy::{BitsPerFragment, Coding, Decoder};
use std::sync::LazyLock;

fn generate_code_from_lichess_weights() -> Coding<u8> {
    Coding::from_frequencies(BitsPerFragment(1), WEIGHTS)
}

// Huffman weights based on:
// https://github.com/lichess-org/compression/blob/master/src/main/java/game/Huffman.java#L64
// They are modified so each value has a unique weight.
const WEIGHTS: [u64; 256] = [
    4291794708, 2564166394, 1691784111, 1318338522, 1083775010, 854516621, 694395945, 600873480,
    540222668, 504269367, 465212587, 438102646, 447170168, 389166683, 388553268, 348005083,
    327081827, 322330459, 314070532, 292020690, 269390360, 271238566, 253712814, 243762438,
    248949248, 226720293, 201564483, 192706721, 157597286, 139412310, 120417136, 102233623,
    86650564, 74352947, 57736573, 47129766, 37069494, 28907569, 22481788, 17835452, 12793441,
    9749907, 7168681, 5262924, 3793958, 2747438, 1962947, 1387874, 994441, 698801, 500479, 355661,
    251275, 178448, 131955, 92967, 70262, 52497, 40166, 30989, 26220, 20710, 16853, 13585, 11210,
    10431, 9063, 7372, 6669, 6061, 4978, 4484, 3800, 3990, 2907, 2223, 2299, 2298, 2185, 1805,
    1425, 1273, 1045, 950, 1044, 627, 626, 570, 608, 532, 551, 513, 399, 285, 171, 190, 228, 227,
    158, 157, 156, 155, 154, 153, 152, 151, 150, 149, 148, 147, 146, 145, 144, 143, 142, 141, 140,
    139, 138, 137, 136, 135, 134, 133, 132, 131, 130, 129, 128, 127, 126, 125, 124, 123, 122, 121,
    120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 110, 109, 108, 107, 106, 105, 104, 103, 102,
    101, 100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79,
    78, 77, 76, 75, 74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64, 63, 62, 61, 60, 59, 58, 57, 56, 55,
    54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31,
    30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6,
    5, 4, 3, 2, 1,
];

static CODE_FROM_LICHESS_WEIGHTS: LazyLock<Coding<u8>> =
    LazyLock::new(generate_code_from_lichess_weights);

pub static BOOK_FROM_LICHESS_WEIGHTS: LazyLock<Book> = LazyLock::new(|| Book {
    codes: (&*CODE_FROM_LICHESS_WEIGHTS)
        .codes_for_values_array()
        .map(|c| {
            let nbits = c.len as usize;
            let mut bv = BitVec::from_elem(nbits, false);
            for i in 0..32 {
                if (c.content >> i) & 1 == 1 {
                    bv.set(nbits - 1 - i, true);
                }
            }

            bv
        }),
});

pub fn get_decoder<'a>() -> Decoder<'a, u8> {
    (&*CODE_FROM_LICHESS_WEIGHTS).decoder()
}

pub struct Book {
    codes: [BitVec; 256],
}

impl Book {
    pub fn encode(&self, buffer: &mut BitVec, symbol: u8) {
        buffer.extend(&self.codes[symbol as usize]);
    }
}

#[cfg(test)]
mod tests {
    use crate::codes::WEIGHTS;

    #[test]
    fn deterministic_code_gen() {
        let code_map = super::generate_code_from_lichess_weights().codes_for_values();
        for _ in 0..1000 {
            let code_map2 = super::generate_code_from_lichess_weights().codes_for_values();
            assert_eq!(code_map, code_map2);
        }
    }

    #[test]
    fn unique_weights() {
        let mut unique_weights = WEIGHTS.to_vec();
        unique_weights.sort_unstable();
        unique_weights.dedup();
        assert_eq!(WEIGHTS.len(), unique_weights.len());
    }
}
