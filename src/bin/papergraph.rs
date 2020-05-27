use clap::Clap;
use dotenv::dotenv;
use flate2::read::GzDecoder;
use itertools::Itertools;
use papergraph::io::Paper;
use serde_json;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::io::BufWriter;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Denny Britz <dennybritz@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Insert records into the database
    #[clap(name = "make-triples", version = "0.1")]
    MakeTriples(MakeTriples),    
}

/// Generate RDF Triples
#[derive(Clap, Debug)]
struct MakeTriples {
    /// Read JSON records from this path
    #[clap(short = "d", long = "data")]
    data: String,

    /// Ignore papers with fewer incoming citations than this
    #[clap(
        long = "min-citations",
        short = "mc",
        default_value = "1",
        required = false
    )]
    min_citations: usize,

    /// Only insert papers with these fields of study
    #[clap(
        long = "field_of_study",
        short = "fos",
        default_value = "Computer Science",
        multiple = true,
        required = false
    )]
    field_of_study: Vec<String>,
}

fn make_triples(opts: MakeTriples) {
    log::info!("reading records from {}", &opts.data);
    let file = File::open(&opts.data).expect("failed to open data file");
    let reader: Box<dyn BufRead> = if opts.data.ends_with(".gz") {
        Box::new(io::BufReader::new(GzDecoder::new(file)))
    } else {
        Box::new(io::BufReader::new(file))
    };

    let min_citations = opts.min_citations;
    let records = reader
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(|l| serde_json::from_str(&l).expect("failed to parse paper"))
        .filter(|p: &Paper| p.in_citations.len() >= min_citations)
        .filter(|p: &Paper| {
            opts.field_of_study
                .iter()
                .any(|fos| p.fields_of_study.contains(fos))
        });
    
    let mut out_writer = BufWriter::new(std::io::stdout());
    for record in records {
        let triples = papergraph::dgraph::s2_record_to_rdf_triples(&record);
        for triple in triples.iter() {
            match write!(out_writer, "{}\n", &triple) {
                Ok(_) => {},
                Err(e) => log::error!("{}", e)
            }
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();
    let opts: Opts = Opts::parse();

    match opts.cmd {
        SubCommand::MakeTriples(opts) => make_triples(opts),
    }
}
