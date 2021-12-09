use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Write};
use std::path::Path;

/// horizontal cat(concatenate) program
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

    /// output file to write
    #[clap(short, long)]
    output: Option<String>,
}

fn output(s: &Option<String>) -> Result<Box<dyn Write>, Error> {
    if let Some(s) = s {
        let f = File::create(s)?;
        Ok(Box::new(f))
    } else {
        Ok(Box::new(io::stdout()))
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut output = output(&args.output)?;
    let sep = &args.sep;
    let files = args.files;
    let lines_coll_ = to_lines_collection(&files);
    let files_: Vec<&String> = lines_coll_.iter().map(|(a, _)| *a).collect();
    let lines_coll: Vec<&Vec<String>> = lines_coll_.iter().map(|(_, b)| b).collect();
    if args.head {
        print_header(&mut output, &files_, sep)?;
    }
    print_body(&mut output, &lines_coll, args.skip, sep)?;
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

fn print_body(
    w: &mut dyn Write,
    lines_coll: &Vec<&Vec<String>>,
    skip: usize,
    sep: &String,
) -> Result<(), Error> {
    let empty = "".to_string();
    for i in skip.. {
        if lines_coll.iter().all(|x| x.get(i) == None) {
            break;
        }
        for (j, x) in lines_coll.iter().enumerate() {
            if j != 0 {
                write!(w, "{}", sep)?;
            }
            let v = x.get(i);
            write!(w, "{}", v.unwrap_or(&empty))?;
        }
        writeln!(w, "")?;
    }
    Ok(())
}

fn print_header(w: &mut dyn Write, files: &Vec<&String>, sep: &String) -> Result<(), Error> {
    for (i, x) in files.iter().enumerate() {
        if i != 0 {
            write!(w, "{}", sep)?;
        }
        write!(w, "{}", x)?;
    }
    writeln!(w, "")?;
    Ok(())
}
