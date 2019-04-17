# file-slice-rust
__author:__ [*Sanhu Li*](mailto:lisanhu2014@hotmail.com), *Sunita Chandrasekaran*

## Introduction
This is a simple program that will print out a slice of file specified by command line parameters. The user will need to enter a path to the file to view, the start bytes to view and the number of bytes to view. Those bytes will be printed as UTF-8 encoding characters.

This program is mostly suitable for ASCII files, since ASCII files are compatible with UTF-8 encoding, and they are fixed-width characters, so the position and expected number of bytes are easy to predict.

## Build
After installing the rust compiler
```Shell
$ cargo build --release
```

## Usage
```Shell
$ target/release/file-slice <path-to-the-file> <start-pos> <num-of-bytes>
```

## Performance Evaluation
No performance evaluation at the moment.

## Contact
[Sanhu Li](mailto:lisanhu2014@hotmail.com)
