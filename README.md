# CCCompress
Rust implementation of a compression tool using a [Huffman Coding Tree](https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html). Followed [Compression Tool Coding Challenge](https://codingchallenges.fyi/challenges/challenge-huffman/).

## Usage
```
# Compress a file
cccompress -i <input_file> -o <output_file> compress
# or
cargo run -- -i <input_file> -o <output_file> compress

# Uncompress a file
cccompress -i <input_file> -o <output_file> uncompress
# or
cargo run -- -i <input_file> -o <output_file> uncompress
```
