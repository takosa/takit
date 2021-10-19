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