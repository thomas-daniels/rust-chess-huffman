# chess-huffman

A Rust crate for [Huffman compression](https://en.wikipedia.org/wiki/Huffman_coding) of chess games. Builds upon [Niklas Fiekas](https://github.com/niklasf)'s crates [`shakmaty`](https://crates.io/crates/shakmaty), [`huffman-compress`](https://crates.io/crates/huffman-compress) and [`pgn-reader`](https://crates.io/crates/pgn-reader), and his [blog post on the topic](https://lichess.org/blog/Wqa7GiAAAOIpBLoY/developer-update-275-improved-game-compression) and [Java implementation](https://github.com/lichess-org/compression/tree/master/src/main/java/game).

Refer to the documentation for up-to-date usage examples:

* Encoding a game: `encode_game`, `encode_pgn`, `MoveByMoveEncoder`
* Decoding a game: `decode_game`, `MoveByMoveDecoder`