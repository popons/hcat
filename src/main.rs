use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Error, Write};

/// horizontal cat(concatenate) program
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opt {
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

    /// Align the output width
    #[clap(short, long)]
    pretty: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::parse();
    let mut output = output(&opt.output)?;
    let sep = &opt.sep;
    let files = &opt.files;
    let lines_coll_ = to_lines_collection(files, &opt);
    let files_: Vec<&String> = lines_coll_.iter().map(|(a, _)| *a).collect();
    let lines_coll: Vec<&Column> = lines_coll_.iter().map(|(_, b)| b).collect();
    if opt.head {
        print_header(&mut output, &files_, sep)?;
    }
    print_body(&mut output, &lines_coll, opt.skip, sep)?;
    Ok(())
}

fn output(s: &Option<String>) -> Result<Box<dyn Write>, Error> {
    if let Some(s) = s {
        let f = File::create(s)?;
        let b = BufWriter::new(f);
        Ok(Box::new(b))
    } else {
        let b = BufWriter::new(io::stdout());
        Ok(Box::new(b))
    }
}

struct Column {
    rows: Vec<String>,
    width: usize,
}

fn to_lines<'a>(path: &'a String, opt: &Opt) -> Result<(&'a String, Column), Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let lines = r.lines();
    let xs = lines.collect::<Result<Vec<String>, Error>>()?;
    let _width = xs.iter().skip(opt.skip).map(|x| x.len()).max().unwrap_or(0);
    let width = if opt.head {
        std::cmp::max(_width, path.len())
    } else {
        _width
    };
    Ok((
        path,
        Column {
            rows: xs,
            width: width,
        },
    ))
}

fn to_lines_collection<'a>(files: &'a Vec<String>, opt: &Opt) -> Vec<(&'a String, Column)> {
    files
        .iter()
        .map(|x| to_lines(x, opt))
        .filter_map(Result::ok)
        .collect()
}

fn print_body(
    w: &mut dyn Write,
    lines_coll: &Vec<&Column>,
    skip: usize,
    sep: &String,
) -> Result<(), Error> {
    let empty = "".to_string();
    for i in skip.. {
        if lines_coll.iter().all(|x| x.rows.get(i) == None) {
            break;
        }
        for (j, x) in lines_coll.iter().enumerate() {
            if j != 0 {
                write!(w, "{}", sep)?;
            }
            let v = x.rows.get(i);
            write!(w, "{:>width$}", v.unwrap_or(&empty), width = x.width)?;
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
