# CCCompress
Rust implementation of a compression tool using a [Huffman Coding Tree](https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html). Followed [Compression Tool Coding Challenge](https://codingchallenges.fyi/challenges/challenge-huffman/).

## Usage
```bash
# Compress a file
cccompress -i input.txt -o compressed.txt compress
# or
cargo run -- -i input.txt -o compressed.txt compress

# Uncompress a file
cccompress -i compressed.txt -o output.txt uncompress
# or
cargo run -- -i compressed.txt -o output.txt uncompress
```
