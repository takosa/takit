takit: TAKayuki's toolKIT
=========================

This program is toolkit for me but if you think it is usefull, you can use it freely.

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
USAGE:
    takit randfa [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l <length>        length of each sequence [default: 100000000]
    -n <n>             number of sequence [default: 5]
```

## 3. snpdensity

Calculate SNP density by each fix-sized bin.

This program calculate SNP density but it is not necessary for your variant to be actually SNP. Only variant positions (chr & bp) are required.

```
USAGE:
    takit snpdensity [OPTIONS] <posfile> [faifile]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -b, --bin <bin>    
            bin size [default: 1000000]


ARGS:
    <posfile>    
            position file (1st column: chromosome name, 2nd column: position(bp), no header)

    <faifile>    
            fasta index file or data which contains chr name in 1st column and length(bp) in 2nd column (output file of
            `samtools faidx`)
```

## 4. randsub

Fetch fasta randomly and get sub sequence.

This program randomly cut out sequence in fasta file and output the sub-sequence.

```
USAGE:
    takit randsub [OPTIONS] <fasta>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --max <max>    max length of sub sequence [default: 30]
        --min <min>    min length of sub sequence [default: 30]
    -n <n>             number of sub sequences [default: 5]

ARGS:
    <fasta>    reference FASTA file
```

## 5. abc

Check what kind of alphabet is used in fasta

```
USAGE:
    takit abc [FLAGS] <fasta>

FLAGS:
    -i               case insensitive (make all base to uppercase)
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <fasta>    target FASTA file
```

## 6. cmpfa

Compare two fasta sequence. if there are same sequences, print id pair for them.

```
USAGE:
    takit cmpfa <fasta1> <fasta2>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <fasta1>    target FASTA file 1
    <fasta2>    target FASTA file 2
```

