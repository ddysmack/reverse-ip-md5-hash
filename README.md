# Reverse IP MD5 Hash Search

Reverse IP MD5 Hash Search is a program written in rust that accepts multiple MD5 hashes and checks if the entered MD5 hash matches the hash of an IPv4 Address. If a result is matched, the program prints out the corresponding IP Address and the time it took to find a match.

## Installation/Compilation

Use the rust compiler [cargo](https://www.rust-lang.org/tools/install) to compile Reverse IP MD5 Hash Search.

```bash
cargo build --release
```

## Usage

```bash
.\target\release\reverse_ip_md5_hasher.exe [hash1] [hash2] [etc]
```

## License

[MIT](https://choosealicense.com/licenses/mit/)