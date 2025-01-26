[![crates.io](https://img.shields.io/crates/v/chess-huffman.svg)](https://crates.io/crates/chess-huffman)
[![docs.rs](https://docs.rs/chess-huffman/badge.svg)](https://docs.rs/chess-huffman)

# chess-huffman

A Rust crate for [Huffman compression](https://en.wikipedia.org/wiki/Huffman_coding) of chess games. Builds upon Piotr Beling's [`minimum_redundancy`](https://crates.io/crates/minimum_redundancy), [Niklas Fiekas](https://github.com/niklasf)'s crates [`shakmaty`](https://crates.io/crates/shakmaty) and [`pgn-reader`](https://crates.io/crates/pgn-reader), and his [blog post on the topic](https://lichess.org/blog/Wqa7GiAAAOIpBLoY/developer-update-275-improved-game-compression) and [Java implementation](https://github.com/lichess-org/compression/tree/master/src/main/java/game).

Refer to the documentation for up-to-date usage examples:

* Encoding a game: `encode_game`, `encode_pgn`, `MoveByMoveEncoder`
* Decoding a game: `decode_game`, `MoveByMoveDecoder`