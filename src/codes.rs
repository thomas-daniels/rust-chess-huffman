use bit_vec::BitVec;
use std::collections::BTreeMap;
use std::iter::FromIterator;

macro_rules! bits {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

/// Huffman code as used in production by lichess.org:
/// https://github.com/lichess-org/compression/blob/master/src/main/java/game/Huffman.java#L64
pub fn lichess_code() -> BTreeMap<u8, BitVec> {
    let mut map: BTreeMap<u8, BitVec> = BTreeMap::new();
    map.insert(0, BitVec::from_iter(vec![false, false])); // 0: 225883932
    map.insert(1, BitVec::from_iter(vec![true, false, false])); // 1: 134956126
    map.insert(2, BitVec::from_iter(vec![true, true, false, true])); // 2: 89041269
    map.insert(3, BitVec::from_iter(vec![true, false, true, false])); // 3: 69386238
    map.insert(4, BitVec::from_iter(vec![false, true, false, true])); // 4: 57040790
    map.insert(5, BitVec::from_iter(vec![true, true, true, false, true])); // 5: 44974559
    map.insert(6, BitVec::from_iter(vec![true, false, true, true, true])); // 6: 36547155
    map.insert(7, BitVec::from_iter(vec![false, true, true, true, false])); // 7: 31624920
    map.insert(8, BitVec::from_iter(vec![false, true, true, false, false])); // 8: 28432772
    map.insert(9, BitVec::from_iter(vec![false, true, false, false, false])); // 9: 26540493
    map.insert(
        10,
        BitVec::from_iter(vec![true, true, true, true, false, true]),
    ); // 10: 24484873
    map.insert(
        11,
        BitVec::from_iter(vec![true, true, true, false, false, true]),
    ); // 11: 23058034
    map.insert(
        12,
        BitVec::from_iter(vec![true, true, true, true, false, false]),
    ); // 12: 23535272
    map.insert(
        13,
        BitVec::from_iter(vec![true, true, false, false, true, true]),
    ); // 13: 20482457
    map.insert(
        14,
        BitVec::from_iter(vec![true, true, false, false, true, false]),
    ); // 14: 20450172
    map.insert(
        15,
        BitVec::from_iter(vec![true, true, false, false, false, false]),
    ); // 15: 18316057
    map.insert(
        16,
        BitVec::from_iter(vec![true, false, true, true, false, true]),
    ); // 16: 17214833
    map.insert(
        17,
        BitVec::from_iter(vec![true, false, true, true, false, false]),
    ); // 17: 16964761
    map.insert(
        18,
        BitVec::from_iter(vec![false, true, true, true, true, true]),
    ); // 18: 16530028
    map.insert(
        19,
        BitVec::from_iter(vec![false, true, true, false, true, true]),
    ); // 19: 15369510
    map.insert(
        20,
        BitVec::from_iter(vec![false, true, false, false, true, true]),
    ); // 20: 14178440
    map.insert(
        21,
        BitVec::from_iter(vec![false, true, true, false, true, false]),
    ); // 21: 14275714
    map.insert(
        22,
        BitVec::from_iter(vec![true, true, true, true, true, true, true]),
    ); // 22: 13353306
    map.insert(
        23,
        BitVec::from_iter(vec![true, true, true, true, true, false, true]),
    ); // 23: 12829602
    map.insert(
        24,
        BitVec::from_iter(vec![true, true, true, true, true, true, false]),
    ); // 24: 13102592
    map.insert(
        25,
        BitVec::from_iter(vec![true, true, true, true, true, false, false]),
    ); // 25: 11932647
    map.insert(
        26,
        BitVec::from_iter(vec![true, true, true, false, false, false, false]),
    ); // 26: 10608657
    map.insert(
        27,
        BitVec::from_iter(vec![true, true, false, false, false, true, true]),
    ); // 27: 10142459
    map.insert(
        28,
        BitVec::from_iter(vec![false, true, true, true, true, false, true]),
    ); // 28: 8294594
    map.insert(
        29,
        BitVec::from_iter(vec![false, true, false, false, true, false, true]),
    ); // 29: 7337490
    map.insert(
        30,
        BitVec::from_iter(vec![false, true, false, false, true, false, false]),
    ); // 30: 6337744
    map.insert(
        31,
        BitVec::from_iter(vec![true, true, true, false, false, false, true, false]),
    ); // 31: 5380717
    map.insert(
        32,
        BitVec::from_iter(vec![true, true, false, false, false, true, false, true]),
    ); // 32: 4560556
    map.insert(
        33,
        BitVec::from_iter(vec![false, true, true, true, true, false, false, true]),
    ); // 33: 3913313
    map.insert(
        34,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, true,
        ]),
    ); // 34: 3038767
    map.insert(
        35,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, true,
        ]),
    ); // 35: 2480514
    map.insert(
        36,
        BitVec::from_iter(vec![
            false, true, true, true, true, false, false, false, true,
        ]),
    ); // 36: 1951026
    map.insert(
        37,
        BitVec::from_iter(vec![
            false, true, true, true, true, false, false, false, false,
        ]),
    ); // 37: 1521451
    map.insert(
        38,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, false,
        ]),
    ); // 38: 1183252
    map.insert(
        39,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, false,
        ]),
    ); // 39: 938708
    map.insert(
        40,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, false,
        ]),
    ); // 40: 673339
    map.insert(
        41,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, false,
        ]),
    ); // 41: 513153
    map.insert(
        42,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, false,
        ]),
    ); // 42: 377299
    map.insert(
        43,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, false,
        ]),
    ); // 43: 276996
    map.insert(
        44,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, false,
        ]),
    ); // 44: 199682
    map.insert(
        45,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, false,
        ]),
    ); // 45: 144602
    map.insert(
        46,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, false,
        ]),
    ); // 46: 103313
    map.insert(
        47,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            false,
        ]),
    ); // 47: 73046
    map.insert(
        48,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            false,
        ]),
    ); // 48: 52339
    map.insert(
        49,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, false,
        ]),
    ); // 49: 36779
    map.insert(
        50,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, false,
        ]),
    ); // 50: 26341
    map.insert(
        51,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, false,
        ]),
    ); // 51: 18719
    map.insert(
        52,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, true,
        ]),
    ); // 52: 13225
    map.insert(
        53,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, true,
        ]),
    ); // 53: 9392
    map.insert(
        54,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, true,
        ]),
    ); // 54: 6945
    map.insert(
        55,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, false,
        ]),
    ); // 55: 4893
    map.insert(
        56,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, false,
        ]),
    ); // 56: 3698
    map.insert(
        57,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, true,
        ]),
    ); // 57: 2763
    map.insert(
        58,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, true,
        ]),
    ); // 58: 2114
    map.insert(
        59,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, false,
        ]),
    ); // 59: 1631
    map.insert(
        60,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, false, false,
        ]),
    ); // 60: 1380
    map.insert(
        61,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, true,
        ]),
    ); // 61: 1090
    map.insert(
        62,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, false,
        ]),
    ); // 62: 887
    map.insert(
        63,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, false, true, false,
        ]),
    ); // 63: 715
    map.insert(
        64,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, false, true,
        ]),
    ); // 64: 590
    map.insert(
        65,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, false, false,
        ]),
    ); // 65: 549
    map.insert(
        66,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, false,
        ]),
    ); // 66: 477
    map.insert(
        67,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, false, true, true, false,
        ]),
    ); // 67: 388
    map.insert(
        68,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, true,
        ]),
    ); // 68: 351
    map.insert(
        69,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, true,
        ]),
    ); // 69: 319
    map.insert(
        70,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, true,
        ]),
    ); // 70: 262
    map.insert(
        71,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, true, false,
        ]),
    ); // 71: 236
    map.insert(
        72,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, false, true, true, true, false,
        ]),
    ); // 72: 200
    map.insert(
        73,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, false, true, true, true, true,
        ]),
    ); // 73: 210
    map.insert(
        74,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, false, false,
        ]),
    ); // 74: 153
    map.insert(
        75,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, true, true,
        ]),
    ); // 75: 117
    map.insert(
        76,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, false,
        ]),
    ); // 76: 121
    map.insert(
        77,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, true, true, true,
        ]),
    ); // 77: 121
    map.insert(
        78,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, true, false,
        ]),
    ); // 78: 115
    map.insert(
        79,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, false, true, true,
        ]),
    ); // 79: 95
    map.insert(
        80,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, true,
        ]),
    ); // 80: 75
    map.insert(
        81,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, true,
        ]),
    ); // 81: 67
    map.insert(
        82,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, true, true,
        ]),
    ); // 82: 55
    map.insert(
        83,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, true,
        ]),
    ); // 83: 50
    map.insert(
        84,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, true, false,
        ]),
    ); // 84: 55
    map.insert(
        85,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, true,
        ]),
    ); // 85: 33
    map.insert(
        86,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, false, true, false, false,
        ]),
    ); // 86: 33
    map.insert(
        87,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, true,
        ]),
    ); // 87: 30
    map.insert(
        88,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, true, true, false, true, true,
        ]),
    ); // 88: 32
    map.insert(
        89,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, true, true, false, false, true,
        ]),
    ); // 89: 28
    map.insert(
        90,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, true, true, false, true, false,
        ]),
    ); // 90: 29
    map.insert(
        91,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, true, true, false, false, false,
        ]),
    ); // 91: 27
    map.insert(
        92,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, false, true, false, true, true,
        ]),
    ); // 92: 21
    map.insert(
        93,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, true,
        ]),
    ); // 93: 15
    map.insert(
        94,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, false, true, false, true, false,
            false,
        ]),
    ); // 94: 9
    map.insert(
        95,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, true, false, true, false, true, false,
            true,
        ]),
    ); // 95: 10
    map.insert(
        96,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, true, false,
        ]),
    ); // 96: 12
    map.insert(
        97,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, true, true,
        ]),
    ); // 97: 12
    map.insert(
        98,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, true,
            true,
        ]),
    ); // 98: 8
    map.insert(
        99,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            true,
        ]),
    ); // 99: 7
    map.insert(
        100,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, true,
            false, true,
        ]),
    ); // 100: 2
    map.insert(
        101,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, false, true,
            true,
        ]),
    ); // 101: 4
    map.insert(
        102,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, false, true,
            false,
        ]),
    ); // 102: 5
    map.insert(
        103,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, false, false,
            false,
        ]),
    ); // 103: 5
    map.insert(
        104,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, false, false,
            true, false,
        ]),
    ); // 104
    map.insert(
        105,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            false, false, true,
        ]),
    ); // 105: 5
    map.insert(
        106,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, false, false, true, true, true, false, false, false, false, false, false,
            true, true,
        ]),
    ); // 106: 1
    map.insert(
        107,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            false, false, false,
        ]),
    ); // 107: 1
    map.insert(
        108,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, true, true,
        ]),
    ); // 108
    map.insert(
        109,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, true, true, false,
        ]),
    ); // 109: 1
    map.insert(
        110,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, true, false,
        ]),
    ); // 110: 2
    map.insert(
        111,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, true, true,
        ]),
    ); // 111: 1
    map.insert(
        112,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, true, false, true,
        ]),
    ); // 112: 1
    map.insert(
        113,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, true, false,
        ]),
    ); // 113
    map.insert(
        114,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, false, false, false,
        ]),
    ); // 114
    map.insert(
        115,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, true, false, true,
        ]),
    ); // 115: 1
    map.insert(
        116,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, false, false, false,
        ]),
    ); // 116
    map.insert(
        117,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, false, true, false,
        ]),
    ); // 117
    map.insert(
        118,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, true, false, true,
        ]),
    ); // 118
    map.insert(
        119,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, true,
            false, false, true, true,
        ]),
    ); // 119
    map.insert(
        120,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, true,
            false, false, true, false,
        ]),
    ); // 120
    map.insert(
        121,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, true,
            false, false, false, true,
        ]),
    ); // 121
    map.insert(
        122,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, true,
            false, false, false, false,
        ]),
    ); // 122
    map.insert(
        123,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, false, true, true,
        ]),
    ); // 123
    map.insert(
        124,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, false, true, false,
        ]),
    ); // 124
    map.insert(
        125,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, false, false, true,
        ]),
    ); // 125
    map.insert(
        126,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, false, false, false,
        ]),
    ); // 126
    map.insert(
        127,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, true, true, true,
        ]),
    ); // 127
    map.insert(
        128,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, true, true, false,
        ]),
    ); // 128
    map.insert(
        129,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, false, true, true,
        ]),
    ); // 129
    map.insert(
        130,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, false, true, false,
        ]),
    ); // 130
    map.insert(
        131,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, false, true, true,
        ]),
    ); // 131
    map.insert(
        132,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, false, true, false,
        ]),
    ); // 132
    map.insert(
        133,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, false, false, true,
        ]),
    ); // 133
    map.insert(
        134,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, false, false, false,
        ]),
    ); // 134
    map.insert(
        135,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            false, true, false, true,
        ]),
    ); // 135
    map.insert(
        136,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            false, true, false, false,
        ]),
    ); // 136
    map.insert(
        137,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, true, false, true,
        ]),
    ); // 137
    map.insert(
        138,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, true, false, false,
        ]),
    ); // 138
    map.insert(
        139,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, true, true, true,
        ]),
    ); // 139
    map.insert(
        140,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, true, false, true,
        ]),
    ); // 140
    map.insert(
        141,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            true, true, false, false,
        ]),
    ); // 141
    map.insert(
        142,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, false, false, true,
        ]),
    ); // 142
    map.insert(
        143,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            false, false, false, false,
        ]),
    ); // 143
    map.insert(
        144,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, true, true, true,
        ]),
    ); // 144
    map.insert(
        145,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, true, true, false,
        ]),
    ); // 145
    map.insert(
        146,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, true, false, false,
        ]),
    ); // 146
    map.insert(
        147,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            false, true, true, true,
        ]),
    ); // 147
    map.insert(
        148,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, true,
            false, true, true, false,
        ]),
    ); // 148
    map.insert(
        149,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, false, false, true,
        ]),
    ); // 149
    map.insert(
        150,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, true, false, false,
        ]),
    ); // 150
    map.insert(
        151,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, false, true, true,
        ]),
    ); // 151
    map.insert(
        152,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, false, true, false,
        ]),
    ); // 152
    map.insert(
        153,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            false, false, false, true,
        ]),
    ); // 153
    map.insert(
        154,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, false, false, false, false,
            true, false, true, true,
        ]),
    ); // 154
    map.insert(
        155,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, true, true, true,
        ]),
    ); // 155
    map.insert(
        156,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, true, true, false,
        ]),
    ); // 156
    map.insert(
        157,
        BitVec::from_iter(vec![
            true, true, false, false, false, true, false, false, false, true, true, true, true,
            true, true, true, false, true, false, true, false, false, true, false, false, false,
            true, true, false, false,
        ]),
    ); // 157
    map.insert(
        158,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, true, false, true,
        ]),
    ); // 158
    map.insert(
        159,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, true, true, true,
        ]),
    ); // 159
    map.insert(
        160,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, false, true, false,
        ]),
    ); // 160
    map.insert(
        161,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, true, true, true,
        ]),
    ); // 161
    map.insert(
        162,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, true, false, false,
        ]),
    ); // 162
    map.insert(
        163,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, false, false, true,
        ]),
    ); // 163
    map.insert(
        164,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, false, true, false,
        ]),
    ); // 164
    map.insert(
        165,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, false, true, false,
        ]),
    ); // 165
    map.insert(
        166,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, false, false, false,
        ]),
    ); // 166
    map.insert(
        167,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, false, true, false,
        ]),
    ); // 167
    map.insert(
        168,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, false, true, true,
        ]),
    ); // 168
    map.insert(
        169,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, false, true, false,
        ]),
    ); // 169
    map.insert(
        170,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, false, false, true,
        ]),
    ); // 170
    map.insert(
        171,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, false, false, false,
        ]),
    ); // 171
    map.insert(
        172,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, true, true, true,
        ]),
    ); // 172
    map.insert(
        173,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, true, true, false,
        ]),
    ); // 173
    map.insert(
        174,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, true, false, true,
        ]),
    ); // 174
    map.insert(
        175,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, true, false, false,
        ]),
    ); // 175
    map.insert(
        176,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, true, true, true,
        ]),
    ); // 176
    map.insert(
        177,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, true, true, false,
        ]),
    ); // 177
    map.insert(
        178,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, true, false, true,
        ]),
    ); // 178
    map.insert(
        179,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, true, false, false,
        ]),
    ); // 179
    map.insert(
        180,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, true, true, true,
        ]),
    ); // 180
    map.insert(
        181,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, true, true, false,
        ]),
    ); // 181
    map.insert(
        182,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, true, false, true,
        ]),
    ); // 182
    map.insert(
        183,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, true, false, false,
        ]),
    ); // 183
    map.insert(
        184,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, true, false, true,
        ]),
    ); // 184
    map.insert(
        185,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, true, false, false,
        ]),
    ); // 185
    map.insert(
        186,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, true, true, true,
        ]),
    ); // 186
    map.insert(
        187,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, true, true, false,
        ]),
    ); // 187
    map.insert(
        188,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, false, false, true,
        ]),
    ); // 188
    map.insert(
        189,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, false, false, false,
        ]),
    ); // 189
    map.insert(
        190,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, false, true, true,
        ]),
    ); // 190
    map.insert(
        191,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, false, true, false,
        ]),
    ); // 191
    map.insert(
        192,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, true, false, true,
        ]),
    ); // 192
    map.insert(
        193,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, true, false, false,
        ]),
    ); // 193
    map.insert(
        194,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, false, true, true,
        ]),
    ); // 194
    map.insert(
        195,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, false, true, false,
        ]),
    ); // 195
    map.insert(
        196,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, true, true, true,
        ]),
    ); // 196
    map.insert(
        197,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, true, true, false,
        ]),
    ); // 197
    map.insert(
        198,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, false, false, true,
        ]),
    ); // 198
    map.insert(
        199,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, true,
            false, false, false, false,
        ]),
    ); // 199
    map.insert(
        200,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, false, true, true,
        ]),
    ); // 200
    map.insert(
        201,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, false, true, false,
        ]),
    ); // 201
    map.insert(
        202,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, false, false, true,
        ]),
    ); // 202
    map.insert(
        203,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, false, false, false,
        ]),
    ); // 203
    map.insert(
        204,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, false, false, true,
        ]),
    ); // 204
    map.insert(
        205,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, false, false, false,
        ]),
    ); // 205
    map.insert(
        206,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, true, true, true,
        ]),
    ); // 206
    map.insert(
        207,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, true, true, false,
        ]),
    ); // 207
    map.insert(
        208,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, false, true, true,
        ]),
    ); // 208
    map.insert(
        209,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, false, true, false,
        ]),
    ); // 209
    map.insert(
        210,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, true, false, true,
        ]),
    ); // 210
    map.insert(
        211,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            true, true, false, false,
        ]),
    ); // 211
    map.insert(
        212,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, true, true, true,
        ]),
    ); // 212
    map.insert(
        213,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, true, true, false,
        ]),
    ); // 213
    map.insert(
        214,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, false, false, true,
        ]),
    ); // 214
    map.insert(
        215,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, false, false,
            false, false, false, false,
        ]),
    ); // 215
    map.insert(
        216,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, false, false, true,
        ]),
    ); // 216
    map.insert(
        217,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, false, false, false,
        ]),
    ); // 217
    map.insert(
        218,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, true, false, true,
        ]),
    ); // 218
    map.insert(
        219,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            false, true, false, false,
        ]),
    ); // 219
    map.insert(
        220,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, true, false, true,
        ]),
    ); // 220
    map.insert(
        221,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, true, false, false,
        ]),
    ); // 221
    map.insert(
        222,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, false, true, true,
        ]),
    ); // 222
    map.insert(
        223,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            false, false, true, false,
        ]),
    ); // 223
    map.insert(
        224,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, false, false, true,
        ]),
    ); // 224
    map.insert(
        225,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, false, false, false,
        ]),
    ); // 225
    map.insert(
        226,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, true, true, true,
        ]),
    ); // 226
    map.insert(
        227,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, true, false,
            true, true, true, false,
        ]),
    ); // 227
    map.insert(
        228,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, false, true, true,
        ]),
    ); // 228
    map.insert(
        229,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, false, true, false,
        ]),
    ); // 229
    map.insert(
        230,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, false, true, true,
        ]),
    ); // 230
    map.insert(
        231,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, false, true, false,
        ]),
    ); // 231
    map.insert(
        232,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, false, true, true,
        ]),
    ); // 232
    map.insert(
        233,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            true, false, true, false,
        ]),
    ); // 233
    map.insert(
        234,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, false, false, true,
        ]),
    ); // 234
    map.insert(
        235,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            true, false, false, false,
        ]),
    ); // 235
    map.insert(
        236,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, true, true, true,
        ]),
    ); // 236
    map.insert(
        237,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, true, true, false,
        ]),
    ); // 237
    map.insert(
        238,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, false, false, true,
        ]),
    ); // 238
    map.insert(
        239,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, false,
            false, false, false, false,
        ]),
    ); // 239
    map.insert(
        240,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, true, false, false,
        ]),
    ); // 240
    map.insert(
        241,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, true, true, true,
        ]),
    ); // 241
    map.insert(
        242,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, true, true, false,
        ]),
    ); // 242
    map.insert(
        243,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, true, true, false,
        ]),
    ); // 243
    map.insert(
        244,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, false, true, true,
        ]),
    ); // 244
    map.insert(
        245,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, false, false, true,
        ]),
    ); // 245
    map.insert(
        246,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, false, false, false,
        ]),
    ); // 246
    map.insert(
        247,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, true, true, false,
        ]),
    ); // 247
    map.insert(
        248,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, false,
            false, true, false, true,
        ]),
    ); // 248
    map.insert(
        249,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, true, false, true,
        ]),
    ); // 249
    map.insert(
        250,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, true, false, false,
        ]),
    ); // 250
    map.insert(
        251,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, false, false, false,
        ]),
    ); // 251
    map.insert(
        252,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            true, false, true, true,
        ]),
    ); // 252
    map.insert(
        253,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, false, false, true,
        ]),
    ); // 253
    map.insert(
        254,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, true, false, true,
            false, false, true, true,
        ]),
    ); // 254
    map.insert(
        255,
        BitVec::from_iter(vec![
            true, true, true, false, false, false, true, true, false, true, true, true, true, true,
            true, true, true, false, true, false, false, true, false, true, false, true, true,
            true, false, true, true,
        ]),
    ); // 255
    map
}
