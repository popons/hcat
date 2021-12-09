use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Names of read files
    files: Vec<String>,

    /// Number of skip line from head
    #[clap(short, long, default_value_t = 0)]
    skip: usize,

    /// print header as file name
    #[clap(short, long)]
    head: bool,

    /// Separator when print items
    #[clap(short, long, default_value = ",")]
    sep: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let sep = &args.sep;
    let files = args.files;
    let lines_coll_ = to_lines_collection(&files);
    let files_: Vec<&String> = lines_coll_.iter().map(|(a, _)| *a).collect();
    let lines_coll: Vec<&Vec<String>> = lines_coll_.iter().map(|(_, b)| b).collect();
    if args.head {
        print_header(&files_, sep);
    }
    print_body(&lines_coll, args.skip, sep);
    Ok(())
}

fn to_lines<P: AsRef<Path>>(path: P) -> Result<(P, Vec<String>), Error> {
    let f = File::open(&path)?;
    let r = BufReader::new(f);
    let lines = r.lines();
    let xs = lines.collect::<Result<Vec<String>, Error>>()?;
    Ok((path, xs))
}

fn to_lines_collection(files: &Vec<String>) -> Vec<(&String, Vec<String>)> {
    files
        .iter()
        .map(|x| to_lines(x))
        .filter_map(Result::ok)
        .collect()
}

fn print_body(lines_coll: &Vec<&Vec<String>>, skip: usize, sep: &String) {
    let empty = "".to_string();
    for i in skip.. {
        if lines_coll.iter().all(|x| x.get(i) == None) {
            break;
        }
        for (j, x) in lines_coll.iter().enumerate() {
            if j != 0 {
                print!("{}", sep);
            }
            let v = x.get(i);
            print!("{}", v.unwrap_or(&empty));
        }
        println!("");
    }
}

fn print_header(files: &Vec<&String>, sep: &String) {
    for (i, x) in files.iter().enumerate() {
        if i != 0 {
            print!("{}", sep);
        }
        print!("{}", x);
    }
    println!("");
}
