# Directory Flattener
This project provides a binary that takes an input string, and preps it for regex usage, effectively replacing known generics and producing a usage-regex string for programmatic parsing.

This project provides a binary that takes a directory path as input, and flattens the contents of said directory recursively, bringing all files to the input directory root, while deleteing any resulting empty directories

## Build & MacOS Installation Instructions
> For Installing Rust and Cargo (Rust's Project Management Binary), please see: https://www.rust-lang.org/tools/install
> Then, run the following lines in terminal:
```sh
git clone https://github.com/13alvone/rusty_dir_flattener.git
https://github.com/13alvone/rusty_regex/edit/main/Readme.md
cargo build --bins --release
sudo cp target/release/rusty_directory_flattener /usr/local/bin/flattener
sudo chmod +x /usr/local/bin/flattener
```

## Usage and Flags
```sh
Usage:
  flattener [OPTIONS]

Flatten a target directory's contents.

Optional arguments:
  -h,--help             Show this help message and exit
  -d,--target_dir TARGET_DIR
                        Path.
```

## Example (Fake Data):
```sh
# flattener -d "test/"
[i] `test/` --> Flattening Starting Here.
[+] test/test/test/test/10 --> test/10: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
[+] test/test/test/9 --> test/9: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
[+] test/test/test/7 --> test/7: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
[+] test/test/test/6 --> test/6: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
[+] test/test/test/8 --> test/8: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
  > Empty Directory Deleted: `test/test/test/test`
[+] test/test/4 --> test/4: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
[+] test/test/3 --> test/3: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
[+] test/test/5 --> test/5: (U: 501, G: 20, CURRENT: 501, Perm: 0o100644)
  > Empty Directory Deleted: `test/test/test`
  > Empty Directory Deleted: `test/test`
```

