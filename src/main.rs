use structopt::StructOpt;
use bio::io::fasta;
use rust_htslib::bcf::{self, Read};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(StructOpt)]
enum Takit {
    Flankvcf {
        fasta: std::path::PathBuf,
        vcf: std::path::PathBuf,
    },
    Randomfasta,
}

fn main() {
    let args = Takit::from_args();
    match args {
        Takit::Flankvcf{fasta, vcf} => {
            let fasta_handle = std::fs::File::open(fasta).expect("Could not open fasta file.");

            let fasta_reader = fasta::Reader::new(fasta_handle);
            
            // fastaの配列をHashMapに格納
            let mut seqs = HashMap::new();
            for result in fasta_reader.records() {
                let record = result.expect("Error during fasta record parsing");
                let id = String::from(record.id());
                let seq: Vec<u8> = record.seq().to_vec();
                seqs.insert(id, seq);

            }

            let mut vcf_reader = bcf::Reader::from_path(vcf).expect("Could not open vcf file.");
            let header = vcf_reader.header().clone();
            for (i, record_result) in vcf_reader.records().enumerate() {
                let record = record_result.expect("Fail to read record");
                let chrom = match header.rid2name(record.rid().unwrap()) {
                    Ok(name) => {std::str::from_utf8(name).unwrap().to_string()},
                    Err(_) => {(record.rid().unwrap()+1).to_string()},
                };
                //let chrom = record.rid().unwrap()+1;
                //let chrom = chrom.to_string();
                let p:usize = record.pos().try_into().unwrap();
                let flanking_front = std::str::from_utf8(&seqs.get(&chrom).unwrap()[p-5..p]).unwrap();
                let refe = std::str::from_utf8(&record.alleles()[0]).unwrap();
                let alt = std::str::from_utf8(&record.alleles()[1]).unwrap();
                let flanking_back = std::str::from_utf8(&seqs.get(&chrom).unwrap()[p+refe.len()..p+refe.len()+5]).unwrap();
                println!("{}[{}/{}]{}", flanking_front, refe, alt, flanking_back);
                //let mut s = String::new();
                // for allele in record.alleles() {
                //     for c in allele {
                //         s.push(char::from(*c))
                //     }
                //     s.push(' ')
                //}
                // 0-based position and the list of alleles
                //println!("Locus: {}, Alleles: {}", record.pos(), s);
                // number of sample in the vcf
                //let sample_count = usize::try_from(record.sample_count()).unwrap();
            
                // Counting ref, alt and missing alleles for each sample
                //let mut n_ref = vec![0; sample_count];
                //let mut n_alt = vec![0; sample_count];
                //let mut n_missing = vec![0; sample_count];
                //let gts = record.genotypes().expect("Error reading genotypes");
                //for sample_index in 0..sample_count {
                //    // for each sample
                //    for gta in gts.get(sample_index).iter() {
                //        // for each allele
                //        match gta.index() {
                //            Some(0) => n_ref[sample_index] += 1,  // reference allele
                //            Some(_) => n_alt[sample_index] += 1,  // alt allele
                //            None => n_missing[sample_index] += 1, // missing allele
                //        }
                //    }
                //}
            }
        },
        Takit::Randomfasta => {
            let mut seed = 42;

            let nucleotides = [b'A', b'C', b'G', b'T'];

            let mut writer = fasta::Writer::new(std::io::stdout());

            for _ in 0..10 {
                let seq = (0..100).map(|_| {
                    seed = ((seed ^ seed << 13) ^ seed >> 7) ^ seed << 17; // don't use this random generator
                    nucleotides[seed % 4]
                }).collect::<Vec<u8>>();

                writer.write("random", None, seq.as_slice()).expect("Error writing record.");
            }
        }
    }
}
