takit: TAKayuki's toolKIT
=========================

This program is toolkit for me.

## Install

This program is written with Rust. If you haven't installed Rust yet,
please install it before install this program from [here](https://www.rust-lang.org/tools/install).

```
git clone https://github.com/takosa/takit.git
cd takit
cargo build --release
```

Then, you can get excutable binary file in `target/release` directory.

If you want to know usage, please run the binary file with `--help` option.

## subcommand

### 1. vcflank

Get flanking sequence for variant site in VCF file from FASTA file.

This program retrieve flanking sequence of variant site which specified in 
VCF file. You can choose two output format; "bracket(...[./.]...)" or "fasta".

```
USAGE:
    takit vcflank [OPTIONS] <fasta> <vcf>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -O, --outfmt <outfmt>    output format (b: bracket, f: fasta) [default: b]
    -w, --width <width>      flanking sequence length [default: 50]

ARGS:
    <fasta>    reference FASTA file
    <vcf>      VCF file for target variant site
```

### 2. randfa

Generate rondom fasta and output to stdout.

```
Generate random fasta

USAGE:
    takit randfa [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l <length>        length of each sequence [default: 100000000]
    -n <n>             number of sequence [default: 5]
```
