use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let files = args().skip(1).collect::<Vec<String>>();
    let lines_coll = to_lines_collection(&files);
    let files_ = lines_coll.iter().map(|(a, _)| *a).collect::<Vec<&String>>();
    let bodies = lines_coll
        .iter()
        .map(|(_, b)| b)
        .collect::<Vec<&Vec<String>>>();
    print_header(files_);
    print_body(bodies);
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
    let lines_coll = files
        .iter()
        .map(|x| to_lines(x))
        .filter_map(Result::ok)
        .collect::<Vec<(&String, Vec<String>)>>();
    lines_coll
}

fn print_body(lines_coll: Vec<&Vec<String>>) {
    let empty = "".to_string();
    for i in 0.. {
        if lines_coll.iter().all(|x| x.get(i) == None) {
            break;
        }
        for (j, x) in lines_coll.iter().enumerate() {
            if j != 0 {
                print!(",");
            }
            let v = x.get(i);
            print!("{}", v.unwrap_or(&empty));
        }
        println!("");
    }
}

fn print_header(files: Vec<&String>) {
    for (i, x) in files.iter().enumerate() {
        if i != 0 {
            print!(",");
        }
        print!("{}", x);
    }
    println!("");
}
