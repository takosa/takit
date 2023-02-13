pub mod vcflank {
    use std::cmp::max;
    use std::str::FromStr;
    use structopt::StructOpt;
    use rust_htslib::bcf::{self, Read};
    use rust_htslib::faidx;
    use std::io::Write;

    enum OutputFmt {
        Bracket,
        Fasta,
    }
    
    type ParseError = &'static str;
    
    impl FromStr for OutputFmt {
        type Err = ParseError;
        fn from_str(fmt: &str) -> Result<Self, Self::Err> {
            let fmt = &fmt.to_lowercase()[..];
            match fmt {
                "b" => Ok(OutputFmt::Bracket),
                "f" => Ok(OutputFmt::Fasta),
                _ => Err("Invalid output format."),
            }
        }
    }

    #[derive(StructOpt)]
    /// Get flanking sequence for variant site in VCF file from FASTA file.
    /// 
    /// This program retrieve flanking sequence of variant site which specified 
    /// in VCF file. You can choose two output format; "bracket(...[./.]...)" or 
    /// "fasta".
    pub struct VcflankOpt {
        /// flanking sequence length
        #[structopt(short = "w", long = "width", default_value = "50")]
        width: i64,
        /// output format (b: bracket, f: fasta)
        #[structopt(short = "O", long = "outfmt", default_value = "b")]
        outfmt: OutputFmt,
        /// reference FASTA file
        fasta: std::path::PathBuf,
        /// VCF file for target variant site
        vcf: std::path::PathBuf,
    }

    pub fn vcflank(args: VcflankOpt) {
    
        // FASTAファイルを開く
        let fasta = faidx::Reader::from_path(&args.fasta).unwrap();

        // VCFファイルを開く
        let mut bcf = bcf::Reader::from_path(&args.vcf).unwrap();

        // VCFヘッダーを読み込む
        //let header = bcf.header().clone();

        // VCFのレコード毎にループ
        for (_, record_result) in bcf.records().enumerate() {
            let record = record_result.unwrap();

            // chr ID
            let chr_id = record.rid().unwrap();

            // chr IDを名前に変換
            let header = record.header().clone();
            let chr = String::from_utf8(header.rid2name(chr_id).unwrap().to_vec()).unwrap();

            let site_start = record.pos();
            let site_end = record.end();
            let flank_start = max(0, site_start - args.width);
            let flank_end = site_end + args.width;

            // usize に変換
            let site_start = site_start as usize;
            let site_end = site_end as usize;
            let flank_start = flank_start as usize;
            let flank_end = flank_end as usize;

            let front_flank =  fasta.fetch_seq_string(&chr, flank_start, site_start-1).unwrap();
            let back_flank =  fasta.fetch_seq_string(&chr, site_end, flank_end-1).unwrap();
            let alleles: Vec<String> = record.alleles().into_iter().map(|x| String::from_utf8(x.to_vec()).unwrap()).collect();
            let alleles = alleles.join("/");
    
            let name = format!("{}_{}", chr, site_start+1);
            match args.outfmt {
                OutputFmt::Bracket => {
                    let res = writeln!(&mut std::io::stdout(), "{}\t{}[{}]{}", name, front_flank, alleles, back_flank);
                    if res.is_err() {
                        std::process::exit(0);
                    }
                },
                OutputFmt::Fasta => {
                    let site_start_in_flanking = front_flank.len() + 1;
                    let site_end_in_flanking = site_start_in_flanking + (site_end - site_start);
                    let desc = format!("position={}-{},alleles={}", site_start_in_flanking, site_end_in_flanking, alleles);
                    let flank =  fasta.fetch_seq_string(&chr, flank_start, flank_end).unwrap();
                    let res = writeln!(&mut std::io::stdout(), ">{} {}\n{}", name, desc, flank);
                    if res.is_err() {
                        std::process::exit(0);
                    }
                },
            }
        }
    
    }

}

pub mod randfa {
    use structopt::StructOpt;
    use std::io;
    use bio::io::fasta;
    use rand::Rng;

    #[derive(StructOpt)]
    /// Generate random fasta
    /// 
    pub struct RandfaOpt {
        /// number of sequence
        #[structopt(short = "n", default_value = "5")]
        n: u64,
        /// length of each sequence
        #[structopt(short = "l", default_value = "100000000")]
        length: u64,
    }
    pub fn randfa(args: RandfaOpt) {
        let nucleotides = [b'A', b'C', b'G', b'T'];
        let mut rng = rand::thread_rng();

        let mut writer = fasta::Writer::new(io::stdout());
        
        for j in 0..args.n {
            let seq = (0..args.length).map(|_| {
                let i: usize = rng.gen();
                nucleotides[i % 4]
            }).collect::<Vec<u8>>();
            let id = format!("random{:02}", j);
            writer.write(&id, None, seq.as_slice()).unwrap();
        }
    }
    
}

<<<<<<< HEAD
pub mod randsub {
    use structopt::StructOpt;
    use std::io;
    use bio::io::fasta;
    use rand::{Rng, seq::SliceRandom};

    #[derive(StructOpt)]
    /// Fetch fasta randomly and get sub sequence
    pub struct RandsubOpt {
        /// number of sub sequences
        #[structopt(short = "n", default_value = "5")]
        n: u64,
        /// min length of sub sequence
        #[structopt(long, default_value = "30")]
        min: u64,
        /// max length of sub sequence
        #[structopt(long, default_value = "30")]
        max: u64,
        /// reference FASTA file
        fasta: std::path::PathBuf,
    }
    pub fn randsub(args: RandsubOpt) {

        let fai_path = {
            let mut ext = args.fasta.extension().unwrap().to_os_string();
            ext.push(".fai");
            args.fasta.with_extension(ext)
        };
        let sequences = fasta::Index::from_file(&fai_path).unwrap().sequences();

        let mut faidx = fasta::IndexedReader::from_file(&args.fasta).unwrap();
        
        let mut rng = rand::thread_rng();
        for j in 0..args.n {
            let seqinfo = sequences.choose(&mut rng).unwrap();
            let name = &seqinfo.name;
            let max_start = seqinfo.len - args.min + 1;
            let start = rng.gen_range(0, max_start);
            let max_end = std::cmp::min(seqinfo.len, start + args.max + 1);
            let end = rng.gen_range(start+args.min, max_end);
            faidx.fetch(&seqinfo.name, start, end).unwrap();
            let mut seq = Vec::new();
            faidx.read(&mut seq);
            let seq_str = std::str::from_utf8(&seq).unwrap();
            print!(">{}:{}-{}\n{}\n", name, start+1, end, seq_str);
        }
    }
    
}
=======
pub mod snpdensity {
    use structopt::StructOpt;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::collections::HashMap;

    #[derive(StructOpt)]
    /// Calculate SNP density by each fixed size bin
    /// 
    /// This program calculate SNP density but it is not necessary for your 
    /// variant to be actually SNP. Only variant positions (chr & bp) are required.
    pub struct SnpdensityOpt {
        /// bin size
        #[structopt(short = "b", long = "bin", default_value = "1000000")]
        bin: usize,
        /// position file (1st column: chromosome name, 2nd column: position(bp), no header)
        posfile: std::path::PathBuf,
        /// fasta index file or data which contains chr name in 1st column and length(bp) in 2nd
        /// column (output file of `samtools faidx`)
        faifile: Option<std::path::PathBuf>,
    }

    pub fn snpdensity(args: SnpdensityOpt) {

        let file = File::open(args.posfile).unwrap();
        let reader = BufReader::new(file);
        let mut densities = HashMap::new();
        if let Some(faifile) = args.faifile {
            let file2 = File::open(faifile).unwrap();
            let reader2 = BufReader::new(file2);
            for result in reader2.lines() {
                let line = result.unwrap();
                let data:Vec<&str> = line.split('\t').collect();
                if data.len() < 2 {
                    panic!("Invalid format for fai file.");
                }
                let chr = data[0];
                let len:usize = data[1].parse().unwrap();
                let mut container = Vec::new();
                container.resize(len / args.bin + 1, 0);
                densities.insert(chr.to_string(), container);
            }
        }

        for result in reader.lines() {
            let line = result.unwrap();
            let data:Vec<&str> = line.split('\t').collect();
            if data.len() != 2 {
                panic!("Only 2 columns, TAB delimited text data is accepted.");
            }
            let pos:usize = data[1].parse().unwrap();
            let chr = data[0];
            let i = (pos - 1) / args.bin;
            if let Some(v) = densities.get_mut(&chr.to_string()) {
                if v.len() <= i {
                    v.resize(i + 1, 0);
                }
                v[i] += 1;
            } else {
                let mut container = Vec::new();
                container.resize(i + 1, 0);
                container[i] += 1;
                densities.insert(chr.to_string(), container);
            }
            
        }

        let mut keys: Vec<&String> = densities.keys().collect();
        keys.sort();
        for key in keys {
            let mut s = 1;
            for d in densities[key].iter() {
                let e = s + args.bin - 1;
                println!("{}\t{}\t{}\t{}", key, s, e, d);
                s += args.bin;
            }
        }

    }
}
>>>>>>> 82976a2e41423a17d316748ddef8a60991863483
